//! Transfer product endpoints.

use crate::models::transfer::{
    TransferAuthorizationCreateResponse, TransferCreateRequest, TransferCreateResponse,
    TransferGetRequest, TransferGetResponse, TransferParams,
};
use crate::{PlaidClient, PlaidError};

impl PlaidClient {
    /// Call `/transfer/authorization/create` to authorize a transfer
    /// before creating it.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn transfer_authorization_create(
        &self,
        params: TransferParams<'_>,
    ) -> Result<TransferAuthorizationCreateResponse, PlaidError> {
        self.post("/transfer/authorization/create", &params).await
    }

    /// Call `/transfer/create` to originate a transfer using a prior
    /// authorization.
    ///
    /// `idempotency_key` makes creation safe to retry: reused keys return
    /// the existing transfer.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn transfer_create(
        &self,
        authorization_id: &str,
        description: &str,
        idempotency_key: &str,
        params: TransferParams<'_>,
    ) -> Result<TransferCreateResponse, PlaidError> {
        let request = TransferCreateRequest {
            params,
            authorization_id,
            description,
            idempotency_key,
        };
        self.post("/transfer/create", &request).await
    }

    /// Call `/transfer/get` to retrieve a transfer by ID.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn transfer_get(&self, transfer_id: &str) -> Result<TransferGetResponse, PlaidError> {
        let request = TransferGetRequest { transfer_id };
        self.post("/transfer/get", &request).await
    }
}
