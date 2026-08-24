//! Sandbox endpoints.
//!
//! These endpoints only work against the Sandbox environment and exist
//! to support testing integrations.

use crate::models::sandbox::{
    SandboxItemFireWebhookRequest, SandboxItemFireWebhookResponse, SandboxItemResetLoginRequest,
    SandboxItemResetLoginResponse, SandboxPublicTokenCreateRequest,
    SandboxPublicTokenCreateResponse,
};
use crate::{PlaidClient, PlaidError};
use secrecy::SecretString;

impl PlaidClient {
    /// Call `/sandbox/public_token/create` to create a public token for
    /// a sandbox institution without going through Link.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn sandbox_public_token_create(
        &self,
        institution_id: &str,
        initial_products: &[String],
    ) -> Result<SandboxPublicTokenCreateResponse, PlaidError> {
        let request = SandboxPublicTokenCreateRequest {
            institution_id: institution_id.to_owned(),
            initial_products: initial_products.to_vec(),
        };
        self.post("/sandbox/public_token/create", &request).await
    }
}

impl PlaidClient {
    /// Call `/sandbox/item/fire_webhook` to trigger a webhook for a
    /// sandbox item, allowing webhook handling to be tested without
    /// real institution data changing.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn sandbox_item_fire_webhook(
        &self,
        access_token: &SecretString,
        webhook_code: &str,
    ) -> Result<SandboxItemFireWebhookResponse, PlaidError> {
        let request = SandboxItemFireWebhookRequest {
            access_token,
            webhook_code,
        };
        self.post("/sandbox/item/fire_webhook", &request).await
    }

    /// Call `/sandbox/item/reset_login` to force a sandbox item into an
    /// `ITEM_LOGIN_REQUIRED` error state, for testing update-mode
    /// re-authentication flows.
    ///
    /// # Errors
    ///
    /// Returns [`PlaidError::Api`] if Plaid rejects the request, or
    /// [`PlaidError::Http`] / [`PlaidError::Serialization`] on transport
    /// or decoding failures.
    pub async fn sandbox_item_reset_login(
        &self,
        access_token: &SecretString,
    ) -> Result<SandboxItemResetLoginResponse, PlaidError> {
        let request = SandboxItemResetLoginRequest { access_token };
        self.post("/sandbox/item/reset_login", &request).await
    }
}
