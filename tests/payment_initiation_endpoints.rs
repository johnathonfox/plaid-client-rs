#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::models::payment_initiation::RecipientBacs;
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

#[tokio::test]
async fn payment_initiation_recipient_create_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/payment_initiation/recipient/create",
        json!({
            "recipient_id": "recipient-id-sandbox-xxx",
            "request_id": "req-recipient-create"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .payment_initiation_recipient_create("Wonder Wallet", Some("GB29NWBK60161331926819"), None)
        .await
        .unwrap();

    assert_eq!(response.recipient_id, "recipient-id-sandbox-xxx");
    assert_eq!(response.request_id, "req-recipient-create");
}

#[tokio::test]
async fn payment_initiation_recipient_create_with_bacs_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/payment_initiation/recipient/create",
        json!({
            "recipient_id": "recipient-id-sandbox-bacs",
            "request_id": "req-recipient-bacs"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let bacs = RecipientBacs {
        account: "26207729".to_string(),
        sort_code: "560029".to_string(),
    };
    let response = client
        .payment_initiation_recipient_create("John Doe", None, Some(bacs))
        .await
        .unwrap();

    assert_eq!(response.recipient_id, "recipient-id-sandbox-bacs");

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["name"], "John Doe");
    assert_eq!(body["bacs"]["account"], "26207729");
    assert_eq!(body["bacs"]["sort_code"], "560029");
    assert!(body.get("iban").is_none());
}

#[tokio::test]
async fn payment_initiation_recipient_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/payment_initiation/recipient/get",
        json!({
            "recipient_id": "recipient-id-sandbox-xxx",
            "name": "Wonder Wallet",
            "iban": "GB29NWBK60161331926819",
            "bacs": null,
            "request_id": "req-recipient-get"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .payment_initiation_recipient_get("recipient-id-sandbox-xxx")
        .await
        .unwrap();

    assert_eq!(response.recipient_id, "recipient-id-sandbox-xxx");
    assert_eq!(response.name, "Wonder Wallet");
    assert_eq!(response.iban.as_deref(), Some("GB29NWBK60161331926819"));
    assert!(response.bacs.is_none());
    assert_eq!(response.request_id, "req-recipient-get");
}

#[tokio::test]
async fn payment_initiation_payment_create_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/payment_initiation/payment/create",
        json!({
            "payment_id": "payment-id-sandbox-xxx",
            "status": "PAYMENT_STATUS_INPUT_NEEDED",
            "request_id": "req-payment-create"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .payment_initiation_payment_create("recipient-id-sandbox-xxx", "TestPayment", 100.0, "GBP")
        .await
        .unwrap();

    assert_eq!(response.payment_id, "payment-id-sandbox-xxx");
    assert_eq!(response.status, "PAYMENT_STATUS_INPUT_NEEDED");
    assert_eq!(response.request_id, "req-payment-create");

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["recipient_id"], "recipient-id-sandbox-xxx");
    assert_eq!(body["reference"], "TestPayment");
    assert_eq!(body["amount"]["value"], 100.0);
    assert_eq!(body["amount"]["currency"], "GBP");
}

#[tokio::test]
async fn payment_initiation_payment_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/payment_initiation/payment/get",
        json!({
            "payment_id": "payment-id-sandbox-xxx",
            "recipient_id": "recipient-id-sandbox-xxx",
            "reference": "Account Funding 99744",
            "amount": {
                "value": 100.0,
                "currency": "GBP"
            },
            "status": "PAYMENT_STATUS_INITIATED",
            "request_id": "req-payment-get"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .payment_initiation_payment_get("payment-id-sandbox-xxx")
        .await
        .unwrap();

    assert_eq!(response.payment_id, "payment-id-sandbox-xxx");
    assert_eq!(response.recipient_id, "recipient-id-sandbox-xxx");
    assert_eq!(response.reference, "Account Funding 99744");
    assert_eq!(response.amount.value, 100.0);
    assert_eq!(response.amount.currency, "GBP");
    assert_eq!(response.status, "PAYMENT_STATUS_INITIATED");
    assert_eq!(response.request_id, "req-payment-get");
}

#[tokio::test]
async fn payment_initiation_payment_list_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/payment_initiation/payment/list",
        json!({
            "payments": [{
                "payment_id": "payment-id-sandbox-xxx",
                "reference": "Account Funding 99744",
                "amount": {
                    "value": 100.0,
                    "currency": "GBP"
                },
                "status": "PAYMENT_STATUS_EXECUTED"
            }],
            "next_cursor": "2020-01-01T00:00:00Z",
            "request_id": "req-payment-list"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .payment_initiation_payment_list(Some(10), None)
        .await
        .unwrap();

    assert_eq!(response.payments.len(), 1);
    assert_eq!(response.payments[0].payment_id, "payment-id-sandbox-xxx");
    assert_eq!(response.payments[0].status, "PAYMENT_STATUS_EXECUTED");
    assert_eq!(
        response.next_cursor.as_deref(),
        Some("2020-01-01T00:00:00Z")
    );
    assert_eq!(response.request_id, "req-payment-list");
}

#[tokio::test]
async fn payment_initiation_payment_list_omits_none_fields() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/payment_initiation/payment/list",
        json!({
            "payments": [],
            "next_cursor": null,
            "request_id": "req-payment-list"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    client
        .payment_initiation_payment_list(None, None)
        .await
        .unwrap();

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert!(body.get("count").is_none());
    assert!(body.get("cursor").is_none());
}

#[tokio::test]
async fn payment_initiation_payment_get_maps_plaid_error() {
    let mock = PlaidMockServer::new().await;
    Mock::given(method("POST"))
        .and(path("/payment_initiation/payment/get"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error_type": "PAYMENT_ERROR",
            "error_code": "PAYMENT_NOT_FOUND",
            "error_message": "payment not found",
            "request_id": "req-error"
        })))
        .mount(&mock.server)
        .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client
        .payment_initiation_payment_get("payment-id-unknown")
        .await
        .unwrap_err();

    match error {
        PlaidError::Api {
            error_type,
            error_code,
            ..
        } => {
            assert_eq!(error_type, "PAYMENT_ERROR");
            assert_eq!(error_code, "PAYMENT_NOT_FOUND");
        }
        other => panic!("expected PlaidError::Api, got {other:?}"),
    }
}
