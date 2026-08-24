//! Models for the Auth product endpoints.

use crate::models::account::Account;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/auth/get`.
#[derive(Debug, Serialize)]
pub struct AuthGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// ACH routing numbers for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchNumbers {
    /// The Plaid account ID.
    pub account_id: String,
    /// The ACH account number.
    pub account: String,
    /// The ACH routing number.
    pub routing: String,
    /// The wire transfer routing number, if available.
    pub wire_routing: Option<String>,
}

/// Account and routing numbers, grouped by scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthNumbers {
    /// ACH numbers for US accounts.
    #[serde(default)]
    pub ach: Vec<AchNumbers>,
}

/// Response from `/auth/get`.
#[derive(Debug, Deserialize)]
pub struct AuthGetResponse {
    /// The accounts associated with the item.
    pub accounts: Vec<Account>,
    /// The account and routing numbers.
    pub numbers: AuthNumbers,
    /// A unique identifier for the request.
    pub request_id: String,
}
