use plaid_client_rs::{Config, Environment, PlaidClient};
use secrecy::SecretString;

#[tokio::test]
async fn client_can_be_created() {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        ..Config::default()
    });

    assert!(client.is_ok());
}

#[tokio::test]
async fn client_uses_correct_base_url_for_sandbox() {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        ..Config::default()
    })
    .unwrap();

    assert_eq!(client.base_url(), "https://sandbox.plaid.com");
}

#[tokio::test]
async fn client_uses_correct_base_url_for_production() {
    let client = PlaidClient::new(Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Production,
        ..Config::default()
    })
    .unwrap();

    assert_eq!(client.base_url(), "https://production.plaid.com");
}
