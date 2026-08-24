//! Helper mock server for integration tests.
//!
//! Included from integration tests via
//! `#[path = "mock/server.rs"] mod server;`.
//!
//! wiremock matches mocks by priority (default 5), then mount order; a
//! mock with `up_to_n_times` stops matching once exhausted.
#![allow(dead_code)]

use serde_json::{json, Value};
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A mock Plaid API server for testing.
pub struct PlaidMockServer {
    pub server: MockServer,
}

impl PlaidMockServer {
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    /// The base URL of the mock server, for `Config::base_url`.
    pub fn uri(&self) -> String {
        self.server.uri()
    }

    /// Mock a successful `POST {endpoint}` with the given JSON body.
    pub async fn mock_ok(&self, endpoint: &str, body: Value) {
        Mock::given(method("POST"))
            .and(path(endpoint))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&self.server)
            .await;
    }

    /// Mock `POST {endpoint}` failing `failures` times with a Plaid
    /// `API_ERROR` body before falling through to an earlier-mounted mock.
    ///
    /// wiremock matches by priority (default 5), so this mock is mounted
    /// with priority 1 to shadow the success case until `up_to_n_times`
    /// exhausts it.
    pub async fn mock_flaky(&self, endpoint: &str, failures: u64) {
        Mock::given(method("POST"))
            .and(path(endpoint))
            .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                "error_type": "API_ERROR",
                "error_code": "INTERNAL_SERVER_ERROR",
                "error_message": "an unexpected error occurred",
                "request_id": "req-flaky"
            })))
            .with_priority(1)
            .up_to_n_times(failures)
            .mount(&self.server)
            .await;
    }

    /// Mock a successful `/sandbox/public_token/create` response.
    ///
    /// Only matches requests whose body contains the given `client_id`
    /// and `secret`, proving the client injects its credentials.
    pub async fn mock_public_token_create(&self, client_id: &str, secret: &str) {
        Mock::given(method("POST"))
            .and(path("/sandbox/public_token/create"))
            .and(body_partial_json(json!({
                "client_id": client_id,
                "secret": secret,
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "public_token": "public-sandbox-xxx",
                "expiration": "2026-08-25T00:00:00Z",
                "request_id": "req-123"
            })))
            .mount(&self.server)
            .await;
    }

    /// Mock a failing `/sandbox/public_token/create` response with a
    /// Plaid error body.
    pub async fn mock_public_token_create_error(&self) {
        Mock::given(method("POST"))
            .and(path("/sandbox/public_token/create"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error_type": "INVALID_INPUT",
                "error_code": "INVALID_API_KEYS",
                "error_message": "invalid client_id or secret",
                "request_id": "req-456"
            })))
            .mount(&self.server)
            .await;
    }

    /// A sample account JSON object reused across endpoint mocks.
    pub fn sample_account() -> Value {
        json!({
            "account_id": "acc-1",
            "name": "Plaid Checking",
            "official_name": "Plaid Gold Standard 0% Interest Checking",
            "type": "depository",
            "subtype": "checking",
            "balances": {
                "available": 100.0,
                "current": 110.0,
                "iso_currency_code": "USD"
            }
        })
    }

    /// Mock a successful `/link/token/create` response.
    pub async fn mock_link_token_create(&self) {
        self.mock_ok(
            "/link/token/create",
            json!({
                "link_token": "link-sandbox-xxx",
                "expiration": "2026-08-24T23:00:00Z",
                "request_id": "req-link"
            }),
        )
        .await;
    }

    /// Mock a successful `/item/public_token/exchange` response.
    pub async fn mock_public_token_exchange(&self) {
        self.mock_ok(
            "/item/public_token/exchange",
            json!({
                "access_token": "access-sandbox-xxx",
                "item_id": "item-1",
                "request_id": "req-exchange"
            }),
        )
        .await;
    }

    /// Mock a successful `/auth/get` response.
    pub async fn mock_auth_get(&self) {
        self.mock_ok(
            "/auth/get",
            json!({
                "accounts": [Self::sample_account()],
                "numbers": {
                    "ach": [{
                        "account_id": "acc-1",
                        "account": "1111222233330000",
                        "routing": "011401533",
                        "wire_routing": "021000021"
                    }]
                },
                "request_id": "req-auth"
            }),
        )
        .await;
    }

    /// Mock a successful `/accounts/balance/get` response.
    pub async fn mock_balance_get(&self) {
        self.mock_ok(
            "/accounts/balance/get",
            json!({
                "accounts": [Self::sample_account()],
                "request_id": "req-balance"
            }),
        )
        .await;
    }

    /// Mock a successful `/transactions/sync` response.
    pub async fn mock_transactions_sync(&self) {
        self.mock_ok(
            "/transactions/sync",
            json!({
                "added": [{
                    "transaction_id": "txn-1",
                    "account_id": "acc-1",
                    "amount": 12.74,
                    "date": "2026-08-20",
                    "name": "Uber",
                    "merchant_name": "Uber",
                    "pending": false
                }],
                "modified": [],
                "removed": [],
                "next_cursor": "cursor-2",
                "has_more": false,
                "request_id": "req-sync"
            }),
        )
        .await;
    }

    /// Mock a successful `/webhook_verification_key/get` response
    /// returning the given JWK.
    pub async fn mock_webhook_verification_key_get(&self, jwk: Value) {
        self.mock_ok(
            "/webhook_verification_key/get",
            json!({
                "key": jwk,
                "request_id": "req-key"
            }),
        )
        .await;
    }

    /// The number of requests the mock server has received.
    pub async fn received_request_count(&self) -> usize {
        self.server
            .received_requests()
            .await
            .map_or(0, |requests| requests.len())
    }
}
