//! Account balance endpoints.

use crate::models::balance::{AccountsBalanceGetRequest, AccountsBalanceGetResponse};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/accounts/balance/get` to retrieve real-time balances for
    /// an item's accounts.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn accounts_balance_get(
        &self,
        access_token: &SecretString,
    ) -> Result<AccountsBalanceGetResponse, PlaidError> {
        let request = AccountsBalanceGetRequest { access_token };
        self.post("/accounts/balance/get", &request).await
    }
}
