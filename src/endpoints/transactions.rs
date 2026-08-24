//! Transactions product endpoints.

use crate::models::transactions::{TransactionsSyncRequest, TransactionsSyncResponse};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/transactions/sync` to fetch transaction updates for an
    /// item using a cursor.
    ///
    /// Pass `None` as `cursor` for the initial sync, then iterate while
    /// `has_more` is true.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn transactions_sync(
        &self,
        access_token: &SecretString,
        cursor: Option<String>,
        count: Option<u32>,
    ) -> Result<TransactionsSyncResponse, PlaidError> {
        let request = TransactionsSyncRequest {
            access_token,
            cursor,
            count,
        };
        self.post("/transactions/sync", &request).await
    }
}
