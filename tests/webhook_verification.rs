#[path = "mock/server.rs"]
mod server;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use plaid_client_rs::{Config, Environment, PlaidClient, PlaidError};
use secrecy::SecretString;
use serde::Serialize;
use serde_json::json;
use server::PlaidMockServer;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

// Test-only EC P-256 keypair (PKCS#8). Never used for real credentials.
const TEST_PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg2UjWP0HtUAVHGzDP
MAtK5BGcNcGkaGAV/SDxQiiHHW6hRANCAAQgGjKaYcdXNCneT6EwLrUVEH+MKksH
DT9Z71F+rxWQCXPquydc+MKjsE2uBuHg5O0MtaQZlpYqlDQYhuN2F2xE
-----END PRIVATE KEY-----";

const TEST_KID: &str = "test-kid";

fn test_jwk() -> serde_json::Value {
    json!({
        "kty": "EC",
        "crv": "P-256",
        "x": "IBoymmHHVzQp3k-hMC61FRB_jCpLBw0_We9Rfq8VkAlz",
        "y": "6rsnXPjCo7BNrgbh4OTtDLWkGZaWKpQ0GIbjdhdsRA",
        "kid": TEST_KID,
        "use": "sig",
        "alg": "ES256"
    })
}

#[derive(Serialize)]
struct TestClaims {
    request_body_sha256: String,
    iat: u64,
    exp: u64,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn sign_webhook_jwt(body: &[u8], iat: u64) -> String {
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(TEST_KID.to_string());
    let claims = TestClaims {
        request_body_sha256: hex::encode(Sha256::digest(body)),
        iat,
        exp: iat + 300,
    };
    let key = EncodingKey::from_ec_pem(TEST_PRIVATE_KEY_PEM.as_bytes()).unwrap();
    encode(&header, &claims, &key).unwrap()
}

fn test_config(base_url: &str) -> Config {
    Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        base_url: Some(base_url.parse().unwrap()),
        ..Config::default()
    }
}

fn webhook_body() -> Vec<u8> {
    serde_json::to_vec(&json!({
        "webhook_type": "TRANSACTIONS",
        "webhook_code": "SYNC_UPDATES_AVAILABLE",
        "item_id": "item-1",
        "environment": "sandbox"
    }))
    .unwrap()
}

#[tokio::test]
async fn verify_webhook_accepts_valid_signature() {
    let mock = PlaidMockServer::new().await;
    mock.mock_webhook_verification_key_get(test_jwk()).await;

    let body = webhook_body();
    let jwt = sign_webhook_jwt(&body, now());

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let envelope = client.verify_webhook(&jwt, &body).await.unwrap();

    assert_eq!(envelope.webhook_type, "TRANSACTIONS");
    assert_eq!(envelope.webhook_code, "SYNC_UPDATES_AVAILABLE");
    assert_eq!(envelope.item_id.as_deref(), Some("item-1"));
    assert_eq!(envelope.data["environment"], "sandbox");
}

#[tokio::test]
async fn verify_webhook_rejects_tampered_body() {
    let mock = PlaidMockServer::new().await;
    mock.mock_webhook_verification_key_get(test_jwk()).await;

    let body = webhook_body();
    let jwt = sign_webhook_jwt(&body, now());
    let tampered = serde_json::to_vec(&json!({
        "webhook_type": "TRANSACTIONS",
        "webhook_code": "WEBHOOK_UPDATE_ACKNOWLEDGED"
    }))
    .unwrap();

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client.verify_webhook(&jwt, &tampered).await.unwrap_err();

    assert!(matches!(error, PlaidError::WebhookVerification(_)));
}

#[tokio::test]
async fn verify_webhook_rejects_stale_token() {
    let mock = PlaidMockServer::new().await;
    mock.mock_webhook_verification_key_get(test_jwk()).await;

    let body = webhook_body();
    let jwt = sign_webhook_jwt(&body, now() - 3600);

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client.verify_webhook(&jwt, &body).await.unwrap_err();

    assert!(matches!(error, PlaidError::WebhookVerification(_)));
}

#[tokio::test]
async fn verify_webhook_rejects_garbage_jwt() {
    let mock = PlaidMockServer::new().await;
    // No key mock needed: header decoding fails before any HTTP call.

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client
        .verify_webhook("not-a-jwt", &webhook_body())
        .await
        .unwrap_err();

    assert!(matches!(error, PlaidError::WebhookVerification(_)));
}

#[tokio::test]
async fn verify_webhook_rejects_future_dated_token() {
    let mock = PlaidMockServer::new().await;
    mock.mock_webhook_verification_key_get(test_jwk()).await;

    let body = webhook_body();
    let jwt = sign_webhook_jwt(&body, now() + 3600);

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let error = client.verify_webhook(&jwt, &body).await.unwrap_err();

    assert!(matches!(error, PlaidError::WebhookVerification(_)));
}
