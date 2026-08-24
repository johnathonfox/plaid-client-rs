//! Models for item management endpoints.

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/item/public_token/exchange`.
#[derive(Debug, Serialize)]
pub struct ItemPublicTokenExchangeRequest {
    /// The public token returned by Link.
    pub public_token: String,
}

/// Response from `/item/public_token/exchange`.
#[derive(Debug, Deserialize)]
pub struct ItemPublicTokenExchangeResponse {
    /// The access token for the new item. Treat as a secret.
    pub access_token: SecretString,
    /// The Plaid item ID.
    pub item_id: String,
    /// A unique identifier for the request.
    pub request_id: String,
}
