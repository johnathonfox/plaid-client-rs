#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(missing_docs, missing_debug_implementations)]

//! A modern, async Rust client for the Plaid API.
//!
//! # Quick Start
//!
//! ```no_run
//! use plaid_client_rs::{PlaidClient, Config, Environment};
//!
//! # async fn run() -> Result<(), plaid_client_rs::PlaidError> {
//! let client = PlaidClient::new(Config {
//!     client_id: "your-client-id".into(),
//!     secret: "your-secret".into(),
//!     environment: Environment::Sandbox,
//!     ..Config::default()
//! });
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod middleware;
pub mod models;
pub mod webhooks;

pub use client::PlaidClient;
pub use config::{Config, Environment};
pub use error::PlaidError;

// Re-export commonly used models at crate root
pub use models::auth;
pub use models::link;
