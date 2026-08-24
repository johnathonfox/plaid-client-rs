//! Models for the Sandbox endpoints.

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/sandbox/public_token/create`.
#[derive(Debug, Clone, Serialize)]
pub struct SandboxPublicTokenCreateRequest {
    /// The institution ID to create the public token for (e.g. `ins_109508`).
    pub institution_id: String,
    /// The Plaid products to initialize the item with.
    pub initial_products: Vec<String>,
}

/// Response from `/sandbox/public_token/create`.
#[derive(Debug, Clone, Deserialize)]
pub struct SandboxPublicTokenCreateResponse {
    /// A public token that can be exchanged for an access token.
    pub public_token: String,
    /// The expiration time of the token.
    pub expiration: String,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Request for `/sandbox/item/fire_webhook`.
#[derive(Debug, Serialize)]
pub struct SandboxItemFireWebhookRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
    /// The webhook code to fire (e.g. `DEFAULT_UPDATE`).
    pub webhook_code: &'a str,
}

/// Response from `/sandbox/item/fire_webhook`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookResponse {
    /// Whether the webhook was fired.
    pub webhook_fired: bool,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Request for `/sandbox/item/reset_login`.
#[derive(Debug, Serialize)]
pub struct SandboxItemResetLoginRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// Response from `/sandbox/item/reset_login`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemResetLoginResponse {
    /// Whether the login was reset.
    pub reset_login: bool,
    /// A unique identifier for the request.
    pub request_id: String,
}
