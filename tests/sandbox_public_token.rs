#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::{Config, Environment, PlaidClient, PlaidError};
use secrecy::SecretString;
use server::PlaidMockServer;

const CLIENT_ID: &str = "test-client-id";
const SECRET: &str = "test-secret";

fn test_config(base_url: &str) -> Config {
    Config {
        client_id: SecretString::from(CLIENT_ID),
        secret: SecretString::from(SECRET),
        environment: Environment::Sandbox,
        base_url: Some(base_url.parse().unwrap()),
        ..Config::default()
    }
}

#[tokio::test]
async fn sandbox_public_token_create_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_public_token_create(CLIENT_ID, SECRET).await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap();

    assert_eq!(response.public_token, "public-sandbox-xxx");
    assert_eq!(response.request_id, "req-123");
}

#[tokio::test]
async fn sandbox_public_token_create_maps_plaid_error() {
    let mock = PlaidMockServer::new().await;
    mock.mock_public_token_create_error().await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client
        .sandbox_public_token_create("ins_109508", &["auth".to_string()])
        .await
        .unwrap_err();

    match error {
        PlaidError::Api {
            error_type,
            error_code,
            error_message,
            request_id,
        } => {
            assert_eq!(error_type, "INVALID_INPUT");
            assert_eq!(error_code, "INVALID_API_KEYS");
            assert_eq!(error_message, "invalid client_id or secret");
            assert_eq!(request_id, "req-456");
        }
        other => panic!("expected PlaidError::Api, got {other:?}"),
    }
}
