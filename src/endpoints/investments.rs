//! Investments product endpoints.

use crate::models::investments::{
    InvestmentsHoldingsGetRequest, InvestmentsHoldingsGetResponse,
    InvestmentsTransactionsGetRequest, InvestmentsTransactionsGetResponse,
};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/investments/holdings/get` to retrieve the holdings for an
    /// item's investment accounts.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn investments_holdings_get(
        &self,
        access_token: &SecretString,
    ) -> Result<InvestmentsHoldingsGetResponse, PlaidError> {
        let request = InvestmentsHoldingsGetRequest { access_token };
        self.post("/investments/holdings/get", &request).await
    }

    /// Call `/investments/transactions/get` to retrieve investment
    /// transactions for an item within a date range.
    ///
    /// `start_date` and `end_date` must be in `YYYY-MM-DD` format.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn investments_transactions_get(
        &self,
        access_token: &SecretString,
        start_date: &str,
        end_date: &str,
    ) -> Result<InvestmentsTransactionsGetResponse, PlaidError> {
        let request = InvestmentsTransactionsGetRequest {
            access_token,
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
        };
        self.post("/investments/transactions/get", &request).await
    }
}
