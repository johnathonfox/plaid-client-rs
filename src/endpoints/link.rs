//! Link token endpoints.

use crate::models::link::{LinkTokenCreateRequest, LinkTokenCreateResponse};
use crate::{PlaidClient, PlaidError};

impl PlaidClient {
    /// Call `/link/token/create` to create a Link token for initializing
    /// Plaid Link.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn link_token_create(
        &self,
        request: &LinkTokenCreateRequest,
    ) -> Result<LinkTokenCreateResponse, PlaidError> {
        self.post("/link/token/create", request).await
    }
}
