//! Request/response middleware for the Plaid client.
//!
//! Middleware wraps the client's request pipeline in an onion-style chain
//! (see ADR-0003). Each middleware receives the [`Request`] and a [`Next`]
//! handle for the remainder of the chain; the terminal sender performs the
//! actual HTTP POST. Built-ins: [`TracingLogger`], [`RetryPolicy`],
//! [`RateLimiter`].

pub mod logging;
pub mod rate_limit;
pub mod retry;

pub use logging::TracingLogger;
pub use rate_limit::RateLimiter;
pub use retry::RetryPolicy;

use crate::client::PlaidClientInner;
use crate::PlaidError;
use serde_json::value::RawValue;
use serde_json::Value;
use std::fmt::Debug;

/// A successful response body, kept as the raw JSON text.
///
/// Typed models are decoded from this text directly, so money fields read
/// their exact decimal digits. A [`serde_json::Value`] would already have
/// decoded every number to `f64` (ADR-0006).
pub type ResponseBody = Box<RawValue>;
use std::sync::Arc;

/// A single API call passed through the middleware chain.
///
/// The body contains injected credentials (`client_id`/`secret`) and may
/// contain access tokens, so `Debug` redacts it and the type is not
/// `Clone`. Middleware must never log or persist the body.
pub struct Request {
    /// The API path being called (e.g. `/auth/get`).
    pub path: String,
    /// The JSON request body, including injected credentials.
    pub body: Value,
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("path", &self.path)
            .field("body", &"<redacted>")
            .finish()
    }
}

/// The remainder of the middleware chain.
///
/// Middleware calls [`Next::run`] to pass the request down the chain and
/// may inspect, delay, or retry around that call.
pub struct Next<'a> {
    pub(crate) chain: &'a [Arc<dyn Middleware>],
    pub(crate) client: &'a PlaidClientInner,
}

impl Debug for Next<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Next")
            .field("remaining", &self.chain.len())
            .finish_non_exhaustive()
    }
}

impl Next<'_> {
    /// Run the rest of the chain for this request.
    ///
    /// # Errors
    ///
    /// Propagates any error from downstream middleware or the terminal
    /// sender.
    pub async fn run(&self, request: &Request) -> Result<ResponseBody, PlaidError> {
        match self.chain.split_first() {
            Some((head, tail)) => {
                head.handle(
                    request,
                    &Next {
                        chain: tail,
                        client: self.client,
                    },
                )
                .await
            }
            None => crate::client::send(self.client, request).await,
        }
    }
}

/// Cross-cutting behavior that wraps request execution.
///
/// Implementations must be `Send + Sync` and are shared via `Arc` on
/// [`crate::Config::middleware`]. Execution order is registration order:
/// the first middleware in the list is the outermost wrapper.
#[async_trait::async_trait]
pub trait Middleware: Debug + Send + Sync {
    /// Handle a request, usually by calling `next.run(request)` zero or
    /// more times and post-processing the result.
    ///
    /// # Errors
    ///
    /// Propagates downstream errors; middleware may also return its own
    /// [`PlaidError`] (e.g. after exhausting retries).
    async fn handle(&self, request: &Request, next: &Next<'_>) -> Result<ResponseBody, PlaidError>;
}
