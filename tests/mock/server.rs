//! Helper mock server for integration tests.
//!
//! Not compiled yet — Cargo only builds top-level files in `tests/`.
//! Wire this in from an integration test with a `#[path = "../mock/server.rs"]`
//! module (or move it to `tests/common/`) when the first endpoint lands.

use wiremock::matchers::{method, path};
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

    /// Mock the `/sandbox/public_token/create` endpoint.
    pub async fn mock_public_token_create(&self) {
        Mock::given(method("POST"))
            .and(path("/sandbox/public_token/create"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "public_token": "public-sandbox-xxx",
                    "expiration": "2026-08-25T00:00:00Z"
                })),
            )
            .mount(&self.server)
            .await;
    }
}
