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
async fn item_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/item/get",
        serde_json::json!({
            "item": {
                "item_id": "Ed6bjNrDLJfGvZWwnkQlfxwoNz54B5C97ejBr",
                "institution_id": "ins_109508",
                "webhook": "https://plaid.com/example/hook",
                "available_products": ["balance", "auth"],
                "billed_products": ["identity", "transactions"]
            },
            "request_id": "req-item-get"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.item_get(&test_access_token()).await.unwrap();

    let item = &response.item;
    assert_eq!(item.item_id, "Ed6bjNrDLJfGvZWwnkQlfxwoNz54B5C97ejBr");
    assert_eq!(item.institution_id.as_deref(), Some("ins_109508"));
    assert_eq!(
        item.webhook.as_deref(),
        Some("https://plaid.com/example/hook")
    );
    assert_eq!(item.available_products, ["balance", "auth"]);
    assert_eq!(item.billed_products, ["identity", "transactions"]);
    assert_eq!(response.request_id, "req-item-get");
}

#[tokio::test]
async fn item_get_handles_null_fields() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/item/get",
        serde_json::json!({
            "item": {
                "item_id": "item-no-institution",
                "institution_id": null,
                "webhook": null,
                "available_products": [],
                "billed_products": ["auth"]
            },
            "request_id": "req-item-get-null"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.item_get(&test_access_token()).await.unwrap();

    assert_eq!(response.item.item_id, "item-no-institution");
    assert!(response.item.institution_id.is_none());
    assert!(response.item.webhook.is_none());
    assert!(response.item.available_products.is_empty());
}

#[tokio::test]
async fn item_remove_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/item/remove",
        serde_json::json!({
            "request_id": "req-item-remove"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.item_remove(&test_access_token()).await.unwrap();

    assert_eq!(response.request_id, "req-item-remove");
}
