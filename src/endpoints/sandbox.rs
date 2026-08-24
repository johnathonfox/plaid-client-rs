//! Sandbox endpoints.
//!
//! These endpoints only work against the Sandbox environment and exist
//! to support testing integrations.

use crate::models::sandbox::{SandboxPublicTokenCreateRequest, SandboxPublicTokenCreateResponse};
use crate::{PlaidClient, PlaidError};

impl PlaidClient {
    /// Call `/sandbox/public_token/create` to create a public token for
    /// a sandbox institution without going through Link.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn sandbox_public_token_create(
        &self,
        institution_id: &str,
        initial_products: &[String],
    ) -> Result<SandboxPublicTokenCreateResponse, PlaidError> {
        let request = SandboxPublicTokenCreateRequest {
            institution_id: institution_id.to_owned(),
            initial_products: initial_products.to_vec(),
        };
        self.post("/sandbox/public_token/create", &request).await
    }
}
