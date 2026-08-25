//! Error types for the Plaid client.

use thiserror::Error;

/// Errors that can occur when using the Plaid client.
#[derive(Debug, Error)]
pub enum PlaidError {
    /// An HTTP request failed before a response was received
    /// (connection errors, timeouts, TLS failures).
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Failed to build the HTTP client.
    #[error("failed to build HTTP client: {0}")]
    HttpClient(reqwest::Error),

    /// A Plaid API error.
    #[error("Plaid API error (HTTP {status}, {error_code}): {error_message}")]
    Api {
        /// The HTTP status code of the response.
        status: u16,
        /// The Plaid error type.
        error_type: String,
        /// The Plaid error code.
        error_code: String,
        /// A human-readable error message.
        error_message: String,
        /// The Plaid request ID for support.
        request_id: String,
    },

    /// A non-2xx response whose body was not a Plaid error
    /// (e.g. a load-balancer 502 page).
    #[error("unexpected HTTP status {status}: {body}")]
    UnexpectedStatus {
        /// The HTTP status code of the response.
        status: u16,
        /// The raw response body.
        body: String,
    },

    /// The response was 2xx but its body could not be decoded.
    #[error("failed to decode response body: {0}")]
    Decode(reqwest::Error),

    /// Serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Webhook signature verification failed (see ADR-0004).
    #[error("webhook verification failed: {0}")]
    WebhookVerification(String),

    /// An unknown or unexpected error occurred.
    #[error("unknown error: {0}")]
    Unknown(String),
}
