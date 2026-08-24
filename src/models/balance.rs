//! Models for account balance endpoints.

use crate::models::account::Account;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/accounts/balance/get`.
#[derive(Debug, Serialize)]
pub struct AccountsBalanceGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// Response from `/accounts/balance/get`.
#[derive(Debug, Deserialize)]
pub struct AccountsBalanceGetResponse {
    /// The accounts with their current balances.
    pub accounts: Vec<Account>,
    /// A unique identifier for the request.
    pub request_id: String,
}
