#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::models::transfer::{
    TransferAchClass, TransferNetwork, TransferParams, TransferType, TransferUser,
};
use plaid_client_rs::{Config, Environment, PlaidClient};
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

fn test_params(access_token: &SecretString) -> TransferParams<'_> {
    TransferParams {
        access_token,
        account_id: "acc-1",
        transfer_type: TransferType::Debit,
        network: TransferNetwork::Ach,
        amount: "12.34",
        ach_class: TransferAchClass::Ppd,
        user: TransferUser {
            legal_name: "Jane Doe",
        },
    }
}

fn sample_transfer() -> serde_json::Value {
    json!({
        "id": "transfer-1",
        "account_id": "acc-1",
        "authorization_id": "authz-1",
        "type": "debit",
        "network": "ach",
        "amount": "12.34",
        "description": "Payment",
        "status": "pending",
        "ach_class": "ppd",
        "created": "2026-08-24T22:00:00Z"
    })
}

#[tokio::test]
async fn transfer_authorization_create_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/transfer/authorization/create",
        json!({
            "authorization": {
                "id": "authz-1",
                "created": "2026-08-24T21:59:00Z",
                "authorization_decision": "approved"
            },
            "request_id": "req-authz"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .transfer_authorization_create(test_params(&test_access_token()))
        .await
        .unwrap();

    assert_eq!(response.authorization.id, "authz-1");
    assert_eq!(response.authorization.authorization_decision, "approved");
    assert_eq!(response.authorization.created, "2026-08-24T21:59:00Z");
    assert_eq!(response.request_id, "req-authz");

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["account_id"], "acc-1");
    assert_eq!(body["type"], "debit");
    assert_eq!(body["amount"], "12.34");
    assert_eq!(body["user"]["legal_name"], "Jane Doe");
}

#[tokio::test]
async fn transfer_create_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/transfer/create",
        json!({
            "transfer": sample_transfer(),
            "request_id": "req-create"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .transfer_create(
            "authz-1",
            "Payment",
            "idem-key-1",
            test_params(&test_access_token()),
        )
        .await
        .unwrap();

    let transfer = &response.transfer;
    assert_eq!(transfer.id, "transfer-1");
    assert_eq!(transfer.account_id, "acc-1");
    assert_eq!(transfer.authorization_id.as_deref(), Some("authz-1"));
    assert_eq!(transfer.transfer_type, TransferType::Debit);
    assert_eq!(transfer.network, TransferNetwork::Ach);
    assert_eq!(transfer.amount, "12.34");
    assert_eq!(transfer.description, "Payment");
    assert_eq!(transfer.status, "pending");
    assert_eq!(transfer.ach_class, TransferAchClass::Ppd);
    assert_eq!(transfer.created, "2026-08-24T22:00:00Z");
    assert_eq!(response.request_id, "req-create");

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["authorization_id"], "authz-1");
    assert_eq!(body["idempotency_key"], "idem-key-1");
    assert_eq!(body["type"], "debit");
}

#[tokio::test]
async fn transfer_get_succeeds_and_omits_access_token() {
    let mock = PlaidMockServer::new().await;
    let mut transfer = sample_transfer();
    transfer["authorization_id"] = serde_json::Value::Null;
    mock.mock_ok(
        "/transfer/get",
        json!({
            "transfer": transfer,
            "request_id": "req-get"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.transfer_get("transfer-1").await.unwrap();

    assert_eq!(response.transfer.id, "transfer-1");
    assert_eq!(response.transfer.authorization_id, None);
    assert_eq!(response.request_id, "req-get");

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["transfer_id"], "transfer-1");
    assert!(body.get("access_token").is_none());
}
