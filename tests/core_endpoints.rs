#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::models::link::{LinkTokenCreateRequest, LinkTokenUser};
use plaid_client_rs::{Config, Environment, PlaidClient};
use rust_decimal::dec;
use secrecy::{ExposeSecret, SecretString};
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
async fn link_token_create_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_link_token_create().await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let request = LinkTokenCreateRequest {
        client_name: "Test App".to_string(),
        language: "en".to_string(),
        country_codes: vec!["US".to_string()],
        products: vec!["auth".to_string()],
        user: LinkTokenUser {
            client_user_id: "user-1".to_string(),
        },
    };
    let response = client.link_token_create(&request).await.unwrap();

    assert_eq!(response.link_token, "link-sandbox-xxx");
    assert_eq!(response.request_id, "req-link");
}

#[tokio::test]
async fn item_public_token_exchange_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_public_token_exchange().await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .item_public_token_exchange("public-sandbox-xxx")
        .await
        .unwrap();

    assert_eq!(response.access_token.expose_secret(), "access-sandbox-xxx");
    assert_eq!(response.item_id, "item-1");
}

#[tokio::test]
async fn auth_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_auth_get().await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.auth_get(&test_access_token()).await.unwrap();

    assert_eq!(response.accounts.len(), 1);
    assert_eq!(response.accounts[0].account_id, "acc-1");
    assert_eq!(response.numbers.ach[0].routing, "011401533");
    assert_eq!(response.request_id, "req-auth");
}

#[tokio::test]
async fn accounts_balance_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_balance_get().await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .accounts_balance_get(&test_access_token())
        .await
        .unwrap();

    let balances = &response.accounts[0].balances;
    assert_eq!(balances.current, Some(dec!(110.0)));
    assert_eq!(balances.iso_currency_code.as_deref(), Some("USD"));
}

#[tokio::test]
async fn transactions_sync_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_transactions_sync().await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .transactions_sync(&test_access_token(), None, None)
        .await
        .unwrap();

    assert_eq!(response.added.len(), 1);
    assert_eq!(response.added[0].transaction_id, "txn-1");
    assert_eq!(response.next_cursor, "cursor-2");
    assert!(!response.has_more);
}

#[tokio::test]
async fn transactions_sync_omits_none_fields() {
    let mock = PlaidMockServer::new().await;
    mock.mock_transactions_sync().await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    client
        .transactions_sync(&test_access_token(), None, None)
        .await
        .unwrap();

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert!(body.get("cursor").is_none());
    assert!(body.get("count").is_none());
}
