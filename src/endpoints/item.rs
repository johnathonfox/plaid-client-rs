//! Item management endpoints.

use crate::models::item::{ItemPublicTokenExchangeRequest, ItemPublicTokenExchangeResponse};
use crate::{PlaidClient, PlaidError};

impl PlaidClient {
    /// Call `/item/public_token/exchange` to exchange a public token
    /// from Link for an access token.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn item_public_token_exchange(
        &self,
        public_token: &str,
    ) -> Result<ItemPublicTokenExchangeResponse, PlaidError> {
        let request = ItemPublicTokenExchangeRequest {
            public_token: public_token.to_owned(),
        };
        self.post("/item/public_token/exchange", &request).await
    }
}
