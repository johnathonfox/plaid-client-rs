//! Retry middleware with exponential backoff.

use super::{Middleware, Next, Request, ResponseBody};
use crate::PlaidError;
use std::time::Duration;

/// Retries failed requests with exponential backoff.
///
/// Retries are attempted for:
///
/// - connection errors (the request never reached the server), and
/// - responses with a 429 or 5xx status, whether the body is a Plaid
///   error ([`PlaidError::Api`]) or not ([`PlaidError::UnexpectedStatus`],
///   e.g. a load-balancer error page).
///
/// Timeouts and response-decode failures are **not** retried: the request
/// may already have been processed server-side, and Plaid writes are not
/// universally idempotent. Where available, prefer endpoints that accept
/// an idempotency key (e.g. `transfer_create`).
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
        fn retryable_status(status: u16) -> bool {
            status == 429 || status >= 500
        }

        match error {
            PlaidError::Http(e) => e.is_connect(),
            PlaidError::Api { status, .. } | PlaidError::UnexpectedStatus { status, .. } => {
                retryable_status(*status)
            }
            _ => false,
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RetryPolicy {
    async fn handle(&self, request: &Request, next: &Next<'_>) -> Result<ResponseBody, PlaidError> {
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
