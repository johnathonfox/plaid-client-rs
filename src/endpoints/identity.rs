//! Identity product endpoints.

use crate::models::identity::{IdentityGetRequest, IdentityGetResponse};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/identity/get` to retrieve account holder identity information
    /// (names, addresses, emails, phone numbers) for an item.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn identity_get(
        &self,
        access_token: &SecretString,
    ) -> Result<IdentityGetResponse, PlaidError> {
        let request = IdentityGetRequest { access_token };
        self.post("/identity/get", &request).await
    }
}
