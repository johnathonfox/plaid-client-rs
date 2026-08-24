//! Models for the Link token endpoints.

use serde::{Deserialize, Serialize};

/// Request for `/link/token/create`.
#[derive(Debug, Clone, Serialize)]
pub struct LinkTokenCreateRequest {
    /// A unique identifier for the end user.
    pub client_name: String,
    /// The language of the Link interface.
    pub language: String,
    /// The country codes of the end user.
    pub country_codes: Vec<String>,
    /// The Plaid products to initialize.
    pub products: Vec<String>,
    /// The user's information.
    pub user: LinkTokenUser,
}

/// User information for Link token creation.
#[derive(Debug, Clone, Serialize)]
pub struct LinkTokenUser {
    /// A unique ID for the end user.
    pub client_user_id: String,
}

/// Response from `/link/token/create`.
#[derive(Debug, Clone, Deserialize)]
pub struct LinkTokenCreateResponse {
    /// A Link token that can be used to initialize Link.
    pub link_token: String,
    /// The expiration time of the token.
    pub expiration: String,
    /// A unique identifier for the request.
    pub request_id: String,
}
