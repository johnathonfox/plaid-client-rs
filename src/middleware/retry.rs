//! Retry middleware with exponential backoff.

use super::{Middleware, Next, Request};
use crate::PlaidError;
use serde_json::Value;
use std::time::Duration;

/// Retries failed requests with exponential backoff.
///
/// Retries transport failures ([`PlaidError::Http`]) and Plaid API errors
/// whose `error_type` is `API_ERROR` or `RATE_LIMIT_EXCEEDED` (the types
/// Plaid returns with 5xx and 429 statuses). All other API errors are
/// returned immediately.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of attempts, including the initial request.
    pub max_attempts: u32,
    /// Delay before the first retry; doubles each attempt.
    pub initial_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
        }
    }
}

impl RetryPolicy {
    fn is_retryable(error: &PlaidError) -> bool {
        match error {
            PlaidError::Http(_) => true,
            PlaidError::Api { error_type, .. } => {
                matches!(error_type.as_str(), "API_ERROR" | "RATE_LIMIT_EXCEEDED")
            }
            _ => false,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RetryPolicy {
    async fn handle(&self, request: &Request, next: &Next<'_>) -> Result<Value, PlaidError> {
        let mut attempt = 0;
        loop {
            attempt += 1;
            match next.run(request).await {
                Ok(value) => return Ok(value),
                Err(error) if attempt < self.max_attempts && Self::is_retryable(&error) => {
                    let delay = self.initial_backoff * 2u32.saturating_pow(attempt - 1);
                    tracing::debug!(
                        path = %request.path,
                        attempt,
                        ?delay,
                        %error,
                        "retrying plaid request"
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(error) => return Err(error),
            }
        }
    }
}
