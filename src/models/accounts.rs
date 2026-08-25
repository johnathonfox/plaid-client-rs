//! Models for the Accounts product endpoints.

use crate::models::account::Account;
use crate::models::item::Item;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/accounts/get`.
#[derive(Debug, Serialize)]
pub struct AccountsGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// Response from `/accounts/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    /// The accounts associated with the item.
    pub accounts: Vec<Account>,
    /// The item the accounts belong to.
    pub item: Item,
    /// A unique identifier for the request.
    pub request_id: String,
}
