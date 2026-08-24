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

/// Request for `/item/get`.
#[derive(Debug, Serialize)]
pub struct ItemGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// Metadata about a Plaid item (a login at a financial institution).
// Field names mirror Plaid's JSON schema.
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// The Plaid item ID. Always unique; linking the same account at the
    /// same institution twice results in two items with different IDs.
    pub item_id: String,
    /// The Plaid institution ID associated with the item, or `None` for
    /// items created without an institution connection.
    pub institution_id: Option<String>,
    /// The URL registered to receive webhooks for the item.
    pub webhook: Option<String>,
    /// The products available for the item that have not yet been accessed.
    pub available_products: Vec<String>,
    /// The products that have been billed for the item.
    pub billed_products: Vec<String>,
}

/// Response from `/item/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGetResponse {
    /// Metadata about the item.
    pub item: Item,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Request for `/item/remove`.
#[derive(Debug, Serialize)]
pub struct ItemRemoveRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// Response from `/item/remove`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRemoveResponse {
    /// A unique identifier for the request.
    pub request_id: String,
}
