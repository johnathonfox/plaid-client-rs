#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::middleware::{RateLimiter, RetryPolicy, TracingLogger};
use plaid_client_rs::{Config, Environment, PlaidClient, PlaidError};
use secrecy::SecretString;
use server::PlaidMockServer;
use std::sync::Arc;
use std::time::{Duration, Instant};

const RETRY_PATH: &str = "/sandbox/public_token/create";

fn test_config(base_url: &str) -> Config {
    Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        base_url: Some(base_url.parse().unwrap()),
        ..Config::default()
    }
}

fn fast_retry(max_attempts: u32) -> RetryPolicy {
    RetryPolicy {
        max_attempts,
        initial_backoff: Duration::from_millis(1),
    }
}

#[tokio::test]
async fn retry_recovers_from_transient_failures() {
    let mock = PlaidMockServer::new().await;
    // Success mock first, then a 2-failure mock shadowing it.
    mock.mock_public_token_create("test-client-id", "test-secret")
        .await;
    mock.mock_flaky(RETRY_PATH, 2).await;

    let mut config = test_config(&mock.uri());
    config.middleware = vec![Arc::new(fast_retry(3))];
    let client = PlaidClient::new(config).unwrap();

    let response = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap();

    assert_eq!(response.public_token, "public-sandbox-xxx");
    assert_eq!(mock.received_request_count().await, 3);
}

#[tokio::test]
async fn retry_gives_up_after_max_attempts() {
    let mock = PlaidMockServer::new().await;
    mock.mock_flaky(RETRY_PATH, 10).await;

    let mut config = test_config(&mock.uri());
    config.middleware = vec![Arc::new(fast_retry(3))];
    let client = PlaidClient::new(config).unwrap();

    let error = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap_err();

    assert!(matches!(error, PlaidError::Api { .. }));
    assert_eq!(mock.received_request_count().await, 3);
}

#[tokio::test]
async fn retry_does_not_retry_client_errors() {
    let mock = PlaidMockServer::new().await;
    // 400 INVALID_INPUT is not retryable.
    mock.mock_public_token_create_error().await;

    let mut config = test_config(&mock.uri());
    config.middleware = vec![Arc::new(fast_retry(3))];
    let client = PlaidClient::new(config).unwrap();

    let error = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap_err();

    assert!(matches!(error, PlaidError::Api { .. }));
    assert_eq!(mock.received_request_count().await, 1);
}

#[tokio::test]
async fn rate_limiter_spaces_requests() {
    let mock = PlaidMockServer::new().await;
    mock.mock_public_token_create("test-client-id", "test-secret")
        .await;

    let mut config = test_config(&mock.uri());
    config.middleware = vec![Arc::new(RateLimiter::new(Duration::from_millis(200)))];
    let client = PlaidClient::new(config).unwrap();

    let start = Instant::now();
    for _ in 0..2 {
        client
            .sandbox_public_token_create("ins_109508", &["auth".to_string()])
            .await
            .unwrap();
    }

    assert!(start.elapsed() >= Duration::from_millis(200));
}

#[tokio::test]
async fn middleware_chain_composes() {
    let mock = PlaidMockServer::new().await;
    mock.mock_public_token_create("test-client-id", "test-secret")
        .await;
    mock.mock_flaky(RETRY_PATH, 1).await;

    let mut config = test_config(&mock.uri());
    config.middleware = vec![
        Arc::new(TracingLogger),
        Arc::new(fast_retry(3)),
        Arc::new(RateLimiter::new(Duration::from_millis(1))),
    ];
    let client = PlaidClient::new(config).unwrap();

    let response = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap();

    assert_eq!(response.public_token, "public-sandbox-xxx");
    assert_eq!(mock.received_request_count().await, 2);
}

/// Mount a load-balancer-style 502 with a non-Plaid body, `failures` times.
async fn mock_bad_gateway(mock: &PlaidMockServer, failures: u64) {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    Mock::given(method("POST"))
        .and(path(RETRY_PATH))
        .respond_with(ResponseTemplate::new(502).set_body_string("<html>Bad Gateway</html>"))
        .with_priority(1)
        .up_to_n_times(failures)
        .mount(&mock.server)
        .await;
}

#[tokio::test]
async fn retry_retries_5xx_with_non_plaid_body() {
    let mock = PlaidMockServer::new().await;
    mock.mock_public_token_create("test-client-id", "test-secret")
        .await;
    mock_bad_gateway(&mock, 2).await;

    let mut config = test_config(&mock.uri());
    config.middleware = vec![Arc::new(fast_retry(3))];
    let client = PlaidClient::new(config).unwrap();

    let response = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap();

    assert_eq!(response.public_token, "public-sandbox-xxx");
    assert_eq!(mock.received_request_count().await, 3);
}

#[tokio::test]
async fn persistent_non_plaid_5xx_maps_to_unexpected_status() {
    let mock = PlaidMockServer::new().await;
    mock_bad_gateway(&mock, 10).await;

    let mut config = test_config(&mock.uri());
    config.middleware = vec![Arc::new(fast_retry(3))];
    let client = PlaidClient::new(config).unwrap();

    let error = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap_err();

    match error {
        PlaidError::UnexpectedStatus { status, body } => {
            assert_eq!(status, 502);
            assert!(body.contains("Bad Gateway"));
        }
        other => panic!("expected PlaidError::UnexpectedStatus, got {other:?}"),
    }
    assert_eq!(mock.received_request_count().await, 3);
}
