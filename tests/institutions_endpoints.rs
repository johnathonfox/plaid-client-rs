#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::{Config, Environment, PlaidClient, PlaidError};
use secrecy::SecretString;
use serde_json::json;
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

fn sample_institution() -> serde_json::Value {
    json!({
        "institution_id": "ins_109508",
        "name": "First Platypus Bank",
        "products": ["auth", "transactions"],
        "country_codes": ["US"],
        "url": "https://www.firstplatypusbank.com",
        "logo": "aGVsbG8=",
        "primary_color": "#0aa1c2"
    })
}

#[tokio::test]
async fn institutions_get_by_id_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/institutions/get_by_id",
        json!({
            "institution": sample_institution(),
            "request_id": "req-institutions-get"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .institutions_get_by_id("ins_109508", &["US".to_string()])
        .await
        .unwrap();

    let institution = &response.institution;
    assert_eq!(institution.institution_id, "ins_109508");
    assert_eq!(institution.name, "First Platypus Bank");
    assert_eq!(institution.products, vec!["auth", "transactions"]);
    assert_eq!(institution.country_codes, vec!["US"]);
    assert_eq!(
        institution.url.as_deref(),
        Some("https://www.firstplatypusbank.com")
    );
    assert_eq!(institution.logo.as_deref(), Some("aGVsbG8="));
    assert_eq!(institution.primary_color.as_deref(), Some("#0aa1c2"));
    assert_eq!(response.request_id, "req-institutions-get");
}

#[tokio::test]
async fn institutions_search_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/institutions/search",
        json!({
            "institutions": [sample_institution()],
            "request_id": "req-institutions-search"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .institutions_search(
            "platypus",
            &["transactions".to_string()],
            &["US".to_string()],
        )
        .await
        .unwrap();

    assert_eq!(response.institutions.len(), 1);
    assert_eq!(response.institutions[0].institution_id, "ins_109508");
    assert_eq!(response.institutions[0].name, "First Platypus Bank");
    assert_eq!(response.request_id, "req-institutions-search");
}

#[tokio::test]
async fn institutions_get_by_id_api_error() {
    let mock = PlaidMockServer::new().await;
    Mock::given(method("POST"))
        .and(path("/institutions/get_by_id"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error_type": "INVALID_INPUT",
            "error_code": "INVALID_INSTITUTION",
            "error_message": "institution_id is invalid",
            "request_id": "req-institutions-error"
        })))
        .mount(&mock.server)
        .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client
        .institutions_get_by_id("ins_bogus", &["US".to_string()])
        .await
        .unwrap_err();

    match error {
        PlaidError::Api {
            error_type,
            error_code,
            request_id,
            ..
        } => {
            assert_eq!(error_type, "INVALID_INPUT");
            assert_eq!(error_code, "INVALID_INSTITUTION");
            assert_eq!(request_id, "req-institutions-error");
        }
        other => panic!("expected PlaidError::Api, got {other:?}"),
    }
}
