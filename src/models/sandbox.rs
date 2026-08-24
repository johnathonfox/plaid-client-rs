//! Models for the Sandbox endpoints.

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
