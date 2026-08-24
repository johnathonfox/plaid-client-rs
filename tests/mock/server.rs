//! Helper mock server for integration tests.
//!
//! Included from integration tests via
//! `#[path = "mock/server.rs"] mod server;`.

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

    /// Mock a successful `/sandbox/public_token/create` response.
    ///
    /// Only matches requests whose body contains the given `client_id`
    /// and `secret`, proving the client injects its credentials.
    pub async fn mock_public_token_create(&self, client_id: &str, secret: &str) {
        Mock::given(method("POST"))
            .and(path("/sandbox/public_token/create"))
            .and(body_partial_json(serde_json::json!({
                "client_id": client_id,
                "secret": secret,
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
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
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "error_type": "INVALID_INPUT",
                "error_code": "INVALID_API_KEYS",
                "error_message": "invalid client_id or secret",
                "request_id": "req-456"
            })))
            .mount(&self.server)
            .await;
    }
}
