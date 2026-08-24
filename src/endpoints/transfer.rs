//! Transfer product endpoints.

use crate::models::transfer::{
    TransferAuthorizationCreateRequest, TransferAuthorizationCreateResponse, TransferCreateRequest,
    TransferCreateResponse, TransferGetRequest, TransferGetResponse, TransferUser,
};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/transfer/authorization/create` to authorize a transfer
    /// before creating it.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    #[allow(clippy::too_many_arguments)]
    pub async fn transfer_authorization_create(
        &self,
        access_token: &SecretString,
        account_id: &str,
        transfer_type: &str,
        network: &str,
        amount: &str,
        ach_class: &str,
        legal_name: &str,
    ) -> Result<TransferAuthorizationCreateResponse, PlaidError> {
        let request = TransferAuthorizationCreateRequest {
            access_token,
            account_id,
            transfer_type,
            network,
            amount,
            ach_class,
            user: TransferUser { legal_name },
        };
        self.post("/transfer/authorization/create", &request).await
    }

    /// Call `/transfer/create` to originate a transfer using a prior
    /// authorization.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    #[allow(clippy::too_many_arguments)]
    pub async fn transfer_create(
        &self,
        access_token: &SecretString,
        account_id: &str,
        authorization_id: &str,
        transfer_type: &str,
        network: &str,
        amount: &str,
        description: &str,
        ach_class: &str,
        legal_name: &str,
        idempotency_key: &str,
    ) -> Result<TransferCreateResponse, PlaidError> {
        let request = TransferCreateRequest {
            access_token,
            account_id,
            authorization_id,
            transfer_type,
            network,
            amount,
            description,
            ach_class,
            user: TransferUser { legal_name },
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
