//! Models for webhook verification endpoints.

use jsonwebtoken::jwk::Jwk;
use serde::{Deserialize, Serialize};

/// Request for `/webhook_verification_key/get`.
#[derive(Debug, Serialize)]
pub struct WebhookVerificationKeyGetRequest {
    /// The `kid` (key ID) from the webhook JWT header.
    pub key_id: String,
}

/// Response from `/webhook_verification_key/get`.
#[derive(Debug, Deserialize)]
pub struct WebhookVerificationKeyGetResponse {
    /// The JSON Web Key used to verify webhook signatures.
    pub key: Jwk,
    /// A unique identifier for the request.
    pub request_id: String,
}
