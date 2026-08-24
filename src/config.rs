//! Client configuration and environment selection.

use crate::middleware::Middleware;
use secrecy::SecretString;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// Plaid API environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Sandbox environment for testing.
    Sandbox,
    /// Production environment.
    Production,
}

/// Configuration for the Plaid client.
///
/// `Config` is not `Clone` because `SecretString` deliberately is not;
/// construct a fresh one (or wrap in `Arc`) if you need to share it.
#[derive(Debug)]
pub struct Config {
    /// Your Plaid client ID.
    pub client_id: SecretString,
    /// Your Plaid secret.
    pub secret: SecretString,
    /// API environment.
    pub environment: Environment,
    /// Base URL override. When set, takes precedence over `environment`
    /// (useful for tests against a mock server). Defaults to `None`.
    pub base_url: Option<Url>,
    /// Middleware chain wrapping every request, outermost first.
    /// Defaults to empty. See ADR-0003.
    pub middleware: Vec<Arc<dyn Middleware>>,
    /// Request timeout. Defaults to 30 seconds.
    pub timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: SecretString::from(""),
            secret: SecretString::from(""),
            environment: Environment::Sandbox,
            base_url: None,
            middleware: Vec::new(),
            timeout: Duration::from_secs(30),
        }
    }
}
