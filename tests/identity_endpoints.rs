#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::{Config, Environment, PlaidClient};
use rust_decimal::dec;
use secrecy::SecretString;
use serde_json::json;
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

fn identity_get_body() -> serde_json::Value {
    json!({
        "accounts": [{
            "account_id": "acc-1",
            "name": "Plaid Checking",
            "official_name": "Plaid Gold Standard 0% Interest Checking",
            "type": "depository",
            "subtype": "checking",
            "balances": {
                "available": 100.0,
                "current": 110.0,
                "iso_currency_code": "USD"
            },
            "owners": [{
                "names": ["Alberta Charleson", "Bobby Charleson"],
                "addresses": [{
                    "data": {
                        "street": "2992 Cameron Road",
                        "city": "Malakoff",
                        "region": "NY",
                        "postal_code": "14236",
                        "country": "US"
                    },
                    "primary": true
                }],
                "emails": [{
                    "data": "accountholder0@example.com",
                    "primary": true,
                    "type": "primary"
                }],
                "phone_numbers": [{
                    "data": "1112223333",
                    "primary": false,
                    "type": "home"
                }]
            }]
        }],
        "request_id": "req-identity"
    })
}

#[tokio::test]
async fn identity_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok("/identity/get", identity_get_body()).await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.identity_get(&test_access_token()).await.unwrap();

    assert_eq!(response.request_id, "req-identity");
    assert_eq!(response.accounts.len(), 1);

    let account = &response.accounts[0];
    assert_eq!(account.account.account_id, "acc-1");
    assert_eq!(account.account.name, "Plaid Checking");
    assert_eq!(
        account.account.official_name.as_deref(),
        Some("Plaid Gold Standard 0% Interest Checking")
    );
    assert_eq!(account.account.account_type, "depository");
    assert_eq!(account.account.subtype.as_deref(), Some("checking"));
    assert_eq!(account.account.balances.current, Some(dec!(110.0)));
    assert_eq!(account.owners.len(), 1);

    let owner = &account.owners[0];
    assert_eq!(owner.names, vec!["Alberta Charleson", "Bobby Charleson"]);

    let address = &owner.addresses[0];
    assert!(address.primary);
    assert_eq!(address.data.street, "2992 Cameron Road");
    assert_eq!(address.data.city, "Malakoff");
    assert_eq!(address.data.region.as_deref(), Some("NY"));
    assert_eq!(address.data.postal_code.as_deref(), Some("14236"));
    assert_eq!(address.data.country.as_deref(), Some("US"));

    let email = &owner.emails[0];
    assert!(email.primary);
    assert_eq!(email.data, "accountholder0@example.com");
    assert_eq!(email.email_type, "primary");

    let phone = &owner.phone_numbers[0];
    assert!(!phone.primary);
    assert_eq!(phone.data, "1112223333");
    assert_eq!(phone.phone_type, "home");
}

#[tokio::test]
async fn identity_get_handles_missing_optional_fields() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/identity/get",
        json!({
            "accounts": [{
                "account_id": "acc-2",
                "name": "Plaid Saving",
                "official_name": null,
                "type": "depository",
                "subtype": null,
                "balances": {
                    "available": null,
                    "current": 50.0,
                    "iso_currency_code": "USD"
                },
                "owners": [{
                    "names": ["Alberta Charleson"],
                    "addresses": [{
                        "data": {
                            "street": "2992 Cameron Road",
                            "city": "Malakoff",
                            "region": null,
                            "postal_code": null,
                            "country": null
                        },
                        "primary": false
                    }],
                    "emails": [],
                    "phone_numbers": []
                }]
            }],
            "request_id": "req-identity-min"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.identity_get(&test_access_token()).await.unwrap();

    let account = &response.accounts[0];
    assert!(account.account.official_name.is_none());
    assert!(account.account.subtype.is_none());

    let owner = &account.owners[0];
    assert_eq!(owner.names.len(), 1);
    let address = &owner.addresses[0];
    assert!(!address.primary);
    assert!(address.data.region.is_none());
    assert!(address.data.postal_code.is_none());
    assert!(address.data.country.is_none());
    assert!(owner.emails.is_empty());
    assert!(owner.phone_numbers.is_empty());
}

#[tokio::test]
async fn identity_get_returns_api_error() {
    let mock = PlaidMockServer::new().await;
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/identity/get"))
        .respond_with(wiremock::ResponseTemplate::new(400).set_body_json(json!({
            "error_type": "ITEM_ERROR",
            "error_code": "ITEM_LOGIN_REQUIRED",
            "error_message": "the login details of this item have changed",
            "request_id": "req-error"
        })))
        .mount(&mock.server)
        .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let result = client.identity_get(&test_access_token()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn identity_balances_are_exact_decimals() {
    // The hand-written IdentityAccount decoder must not let the balance
    // through a float (ADR-0006). Through f64 this balance reads
    // 12345678901234.568.
    let mock = PlaidMockServer::new().await;
    mock.mock_ok_raw(
        "/identity/get",
        r#"{"accounts":[{"account_id":"acc-1","name":"Checking","type":"depository",
            "balances":{"available":0.1,"current":12345678901234.567,"iso_currency_code":"USD"},
            "owners":[]}],"request_id":"req-exact"}"#,
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.identity_get(&test_access_token()).await.unwrap();
    let balances = &response.accounts[0].account.balances;
    assert_eq!(balances.available.unwrap().to_string(), "0.1");
    assert_eq!(balances.current.unwrap().to_string(), "12345678901234.567");
    assert!(response.accounts[0].owners.is_empty());
}
