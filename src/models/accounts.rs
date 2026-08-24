//! Models for the Accounts product endpoints.

use crate::models::account::Account;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/accounts/get`.
#[derive(Debug, Serialize)]
pub struct AccountsGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// The item whose accounts were returned by `/accounts/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsGetItem {
    /// The Plaid item ID.
    pub item_id: String,
    /// The Plaid institution ID, if the item is associated with one.
    pub institution_id: Option<String>,
}

/// Response from `/accounts/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    /// The accounts associated with the item.
    pub accounts: Vec<Account>,
    /// The item the accounts belong to.
    pub item: AccountsGetItem,
    /// A unique identifier for the request.
    pub request_id: String,
}
