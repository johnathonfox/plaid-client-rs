//! Models for Auth and sandbox token endpoints.

use serde::{Deserialize, Serialize};

/// Request for `/sandbox/public_token/create`.
#[derive(Debug, Clone, Serialize)]
pub struct SandboxPublicTokenCreateRequest {
    /// The ID of the institution to create a token for.
    pub institution_id: String,
    /// The products to initialize.
    pub initial_products: Vec<String>,
}

/// Response from `/sandbox/public_token/create`.
#[derive(Debug, Clone, Deserialize)]
pub struct SandboxPublicTokenCreateResponse {
    /// A public token that can be exchanged for an access token.
    pub public_token: String,
    /// The expiration time of the token, if applicable.
    pub expiration: Option<String>,
}
