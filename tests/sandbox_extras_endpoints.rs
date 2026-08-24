#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;
use server::PlaidMockServer;

fn test_config(base_url: &str) -> Config {
    Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        base_url: Some(base_url.parse().unwrap()),
        ..Config::default()
    }
}

fn test_access_token() -> SecretString {
    SecretString::from("access-sandbox-xxx")
}

#[tokio::test]
async fn sandbox_item_fire_webhook_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/sandbox/item/fire_webhook",
        serde_json::json!({
            "webhook_fired": true,
            "request_id": "req-fire-webhook"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .sandbox_item_fire_webhook(&test_access_token(), "DEFAULT_UPDATE")
        .await
        .unwrap();

    assert!(response.webhook_fired);
    assert_eq!(response.request_id, "req-fire-webhook");
}

#[tokio::test]
async fn sandbox_item_fire_webhook_sends_webhook_code() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/sandbox/item/fire_webhook",
        serde_json::json!({
            "webhook_fired": true,
            "request_id": "req-fire-webhook"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    client
        .sandbox_item_fire_webhook(&test_access_token(), "DEFAULT_UPDATE")
        .await
        .unwrap();

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["webhook_code"], "DEFAULT_UPDATE");
    assert_eq!(body["access_token"], "access-sandbox-xxx");
}

#[tokio::test]
async fn sandbox_item_reset_login_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/sandbox/item/reset_login",
        serde_json::json!({
            "reset_login": true,
            "request_id": "req-reset-login"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .sandbox_item_reset_login(&test_access_token())
        .await
        .unwrap();

    assert!(response.reset_login);
    assert_eq!(response.request_id, "req-reset-login");
}

#[tokio::test]
async fn sandbox_item_reset_login_fails_on_missing_fields() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok("/sandbox/item/reset_login", serde_json::json!({}))
        .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let result = client.sandbox_item_reset_login(&test_access_token()).await;

    assert!(result.is_err());
}
