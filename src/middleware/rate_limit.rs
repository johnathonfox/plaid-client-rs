//! Rate-limiting middleware.

use super::{Middleware, Next, Request};
use crate::PlaidError;
use serde_json::Value;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Enforces a minimum interval between outgoing requests.
///
/// Requests are serialized through a mutex: each request waits until at
/// least `min_interval` has passed since the previous one was released.
#[derive(Debug)]
pub struct RateLimiter {
    min_interval: Duration,
    last_release: Mutex<Option<Instant>>,
}

impl RateLimiter {
    /// Create a limiter that spaces requests at least `min_interval` apart.
    #[must_use]
    pub fn new(min_interval: Duration) -> Self {
        Self {
            min_interval,
            last_release: Mutex::new(None),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimiter {
    async fn handle(&self, request: &Request, next: &Next<'_>) -> Result<Value, PlaidError> {
        let mut guard = self.last_release.lock().await;
        if let Some(last) = *guard {
            let elapsed = last.elapsed();
            if elapsed < self.min_interval {
                tokio::time::sleep(self.min_interval.saturating_sub(elapsed)).await;
            }
        }
        *guard = Some(Instant::now());
        drop(guard);
        next.run(request).await
    }
}
