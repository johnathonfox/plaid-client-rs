//! Webhook verification endpoints.

use crate::models::webhook::{WebhookVerificationKeyGetRequest, WebhookVerificationKeyGetResponse};
use crate::webhooks::WebhookEnvelope;
use crate::{PlaidClient, PlaidError};

impl PlaidClient {
    /// Call `/webhook_verification_key/get` to fetch the JWK for a
    /// webhook JWT `kid`.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn webhook_verification_key_get(
        &self,
        key_id: &str,
    ) -> Result<WebhookVerificationKeyGetResponse, PlaidError> {
        let request = WebhookVerificationKeyGetRequest {
            key_id: key_id.to_owned(),
        };
        self.post("/webhook_verification_key/get", &request).await
    }

    /// Verify a webhook delivery and parse its payload.
    ///
    /// `jwt` is the value of the `Plaid-Verification` header and
    /// `raw_body` must be the exact, unparsed request body. Fetches the
    /// verification key identified by the JWT's `kid`, then checks the
    /// signature, token freshness, and body hash (see ADR-0004).
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::WebhookVerification`] if the signature,
    /// freshness, or body-hash check fails, and [`PlaidError::Api`] /
    /// [`PlaidError::Http`] if fetching the verification key fails.
    pub async fn verify_webhook(
        &self,
        jwt: &str,
        raw_body: &[u8],
    ) -> Result<WebhookEnvelope, PlaidError> {
        let key_id = crate::webhooks::decode_key_id(jwt)?;
        let response = self.webhook_verification_key_get(&key_id).await?;
        crate::webhooks::verify(&response.key, jwt, raw_body)
    }
}
