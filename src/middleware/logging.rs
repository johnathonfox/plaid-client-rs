//! Request/response logging middleware built on `tracing`.

use super::{Middleware, Next, Request, ResponseBody};
use crate::PlaidError;

/// Logs each request and its outcome at `DEBUG` level via `tracing`.
///
/// Never logs the request body, because bodies contain credentials
/// (`client_id`/`secret`) and access tokens.
#[derive(Debug, Default, Clone, Copy)]
pub struct TracingLogger;

#[async_trait::async_trait]
impl Middleware for TracingLogger {
    async fn handle(&self, request: &Request, next: &Next<'_>) -> Result<ResponseBody, PlaidError> {
        tracing::debug!(path = %request.path, "plaid request");
        let result = next.run(request).await;
        match &result {
            Ok(_) => tracing::debug!(path = %request.path, "plaid response ok"),
            Err(error) => {
                tracing::debug!(path = %request.path, %error, "plaid response error");
            }
        }
        result
    }
}
