//! Liabilities product endpoints.

use crate::models::liabilities::{LiabilitiesGetRequest, LiabilitiesGetResponse};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/liabilities/get` to retrieve liabilities data (credit,
    /// student, and mortgage loans) for an item.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn liabilities_get(
        &self,
        access_token: &SecretString,
    ) -> Result<LiabilitiesGetResponse, PlaidError> {
        let request = LiabilitiesGetRequest { access_token };
        self.post("/liabilities/get", &request).await
    }
}
