//! Auth product endpoints.

use crate::models::auth::{AuthGetRequest, AuthGetResponse};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/auth/get` to retrieve account and routing numbers for an
    /// item.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn auth_get(
        &self,
        access_token: &SecretString,
    ) -> Result<AuthGetResponse, PlaidError> {
        let request = AuthGetRequest { access_token };
        self.post("/auth/get", &request).await
    }
}
