//! The main Plaid API client.

use crate::{Config, PlaidError};
use reqwest::Client;
use secrecy::SecretString;
use std::sync::Arc;

/// The main Plaid API client.
///
/// Clone is cheap — `PlaidClient` uses an `Arc` internally.
#[derive(Debug, Clone)]
pub struct PlaidClient {
    inner: Arc<PlaidClientInner>,
}

#[derive(Debug)]
struct PlaidClientInner {
    http: Client,
    config: Config,
}

impl PlaidClient {
    /// Create a new client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be built.
    pub fn new(config: Config) -> Result<Self, PlaidError> {
        let http = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(PlaidError::HttpClient)?;

        let inner = Arc::new(PlaidClientInner { http, config });

        Ok(Self { inner })
    }

    /// Returns the base URL for the configured environment.
    #[must_use]
    pub fn base_url(&self) -> &str {
        match self.inner.config.environment {
            crate::Environment::Sandbox => "https://sandbox.plaid.com",
            crate::Environment::Production => "https://production.plaid.com",
        }
    }

    // Used by endpoint modules as they are implemented; not dead code long-term.
    /// Returns a reference to the underlying HTTP client.
    #[allow(dead_code)]
    pub(crate) fn http(&self) -> &Client {
        &self.inner.http
    }

    /// Returns the client ID.
    #[allow(dead_code)]
    pub(crate) fn client_id(&self) -> &SecretString {
        &self.inner.config.client_id
    }

    /// Returns the secret.
    #[allow(dead_code)]
    pub(crate) fn secret(&self) -> &SecretString {
        &self.inner.config.secret
    }
}
