//! Item management endpoints.

use crate::models::item::{
    ItemGetRequest, ItemGetResponse, ItemPublicTokenExchangeRequest,
    ItemPublicTokenExchangeResponse, ItemRemoveRequest, ItemRemoveResponse,
};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

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

    /// Call `/item/get` to retrieve the status and metadata of an item.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn item_get(
        &self,
        access_token: &SecretString,
    ) -> Result<ItemGetResponse, PlaidError> {
        let request = ItemGetRequest { access_token };
        self.post("/item/get", &request).await
    }

    /// Call `/item/remove` to remove an item, invalidating its access
    /// token and any associated processor tokens.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn item_remove(
        &self,
        access_token: &SecretString,
    ) -> Result<ItemRemoveResponse, PlaidError> {
        let request = ItemRemoveRequest { access_token };
        self.post("/item/remove", &request).await
    }
}
