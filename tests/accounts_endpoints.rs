#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::{Config, Environment, PlaidClient, PlaidError};
use secrecy::SecretString;
use server::PlaidMockServer;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

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
async fn accounts_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/accounts/get",
        serde_json::json!({
            "accounts": [PlaidMockServer::sample_account()],
            "item": {
                "item_id": "item-1",
                "institution_id": "ins_109508",
                "webhook": null,
                "available_products": ["auth", "balance"],
                "billed_products": ["auth"]
            },
            "request_id": "req-accounts"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.accounts_get(&test_access_token()).await.unwrap();

    assert_eq!(response.accounts.len(), 1);
    assert_eq!(response.accounts[0].account_id, "acc-1");
    assert_eq!(response.accounts[0].name, "Plaid Checking");
    assert_eq!(response.item.item_id, "item-1");
    assert_eq!(response.item.institution_id.as_deref(), Some("ins_109508"));
    assert_eq!(response.item.available_products, vec!["auth", "balance"]);
    assert_eq!(response.request_id, "req-accounts");
}

#[tokio::test]
async fn accounts_get_handles_missing_institution_id() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/accounts/get",
        serde_json::json!({
            "accounts": [PlaidMockServer::sample_account()],
            "item": {
                "item_id": "item-1",
                "institution_id": null,
                "webhook": null,
                "available_products": [],
                "billed_products": []
            },
            "request_id": "req-accounts"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.accounts_get(&test_access_token()).await.unwrap();

    assert_eq!(response.item.institution_id, None);
}

#[tokio::test]
async fn accounts_get_returns_api_error() {
    let mock = PlaidMockServer::new().await;
    Mock::given(method("POST"))
        .and(path("/accounts/get"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error_type": "INVALID_INPUT",
            "error_code": "INVALID_ACCESS_TOKEN",
            "error_message": "could not find matching access token",
            "request_id": "req-err"
        })))
        .mount(&mock.server)
        .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client.accounts_get(&test_access_token()).await.unwrap_err();

    assert!(matches!(
        error,
        PlaidError::Api { ref error_code, .. } if error_code == "INVALID_ACCESS_TOKEN"
    ));
}
