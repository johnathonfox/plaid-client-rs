//! Webhook payload types and signature verification.
//!
//! Plaid signs webhooks with an ES256 JWT in the `Plaid-Verification`
//! header (see ADR-0004). Use [`crate::PlaidClient::verify_webhook`] to
//! verify a delivery and parse its payload.

use crate::PlaidError;
use jsonwebtoken::{jwk::Jwk, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

/// Maximum accepted age of a webhook JWT, per Plaid's documentation.
const MAX_TOKEN_AGE_SECS: u64 = 300;

/// Tolerance for the receiving host's clock running slightly behind
/// Plaid's signing clock.
const CLOCK_SKEW_LEEWAY_SECS: u64 = 30;

/// The common fields of every Plaid webhook payload.
///
/// Product-specific fields are captured in [`WebhookEnvelope::data`].
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookEnvelope {
    /// The webhook type (e.g. `TRANSACTIONS`, `ITEM`).
    pub webhook_type: String,
    /// The webhook code (e.g. `SYNC_UPDATES_AVAILABLE`).
    pub webhook_code: String,
    /// The item this webhook concerns, when applicable.
    #[serde(default)]
    pub item_id: Option<String>,
    /// All remaining product-specific fields.
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Claims Plaid includes in the webhook JWT.
#[derive(Debug, Deserialize)]
struct WebhookClaims {
    request_body_sha256: String,
    iat: u64,
}

fn verification_error(message: impl std::fmt::Display) -> PlaidError {
    PlaidError::WebhookVerification(message.to_string())
}

/// Compare two byte strings without a length-dependent early exit.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Extract the `kid` (key ID) from a webhook JWT header without
/// verifying the signature.
pub(crate) fn decode_key_id(jwt: &str) -> Result<String, PlaidError> {
    let header = jsonwebtoken::decode_header(jwt)
        .map_err(|e| verification_error(format!("invalid JWT header: {e}")))?;
    header
        .kid
        .ok_or_else(|| verification_error("JWT header is missing `kid`"))
}

/// Verify a webhook JWT against a verification key and the raw body.
///
/// Checks the ES256 signature, the 5-minute `iat` freshness window, and
/// that the `request_body_sha256` claim matches the SHA-256 hex of
/// `raw_body`. On success, parses and returns the payload envelope.
pub(crate) fn verify(key: &Jwk, jwt: &str, raw_body: &[u8]) -> Result<WebhookEnvelope, PlaidError> {
    let decoding_key =
        DecodingKey::from_jwk(key).map_err(|e| verification_error(format!("invalid JWK: {e}")))?;

    // Plaid tokens are freshness-checked via `iat` (docs mandate a
    // 5-minute window); `exp` validation is disabled so tokens without
    // an `exp` claim are not spuriously rejected.
    let mut validation = Validation::new(Algorithm::ES256);
    validation.validate_exp = false;
    validation.required_spec_claims = HashSet::new();

    let token = jsonwebtoken::decode::<WebhookClaims>(jwt, &decoding_key, &validation)
        .map_err(|e| verification_error(format!("signature verification failed: {e}")))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| verification_error(format!("system clock error: {e}")))?
        .as_secs();
    let iat = token.claims.iat;
    // Plaid's rule: issued within the last 5 minutes. Future-dated
    // tokens are rejected too, with a small allowance for clock skew.
    if iat > now + CLOCK_SKEW_LEEWAY_SECS {
        return Err(verification_error("webhook token is dated in the future"));
    }
    if now.saturating_sub(iat) > MAX_TOKEN_AGE_SECS {
        return Err(verification_error("webhook token is stale"));
    }

    let body_hash = Sha256::digest(raw_body);
    let claim_hash = hex::decode(&token.claims.request_body_sha256).map_err(|e| {
        verification_error(format!("request_body_sha256 claim is not valid hex: {e}"))
    })?;
    if !constant_time_eq(&body_hash, &claim_hash) {
        return Err(verification_error(
            "request body hash does not match the JWT claim",
        ));
    }

    serde_json::from_slice(raw_body)
        .map_err(|e| verification_error(format!("verified payload is not valid webhook JSON: {e}")))
}
