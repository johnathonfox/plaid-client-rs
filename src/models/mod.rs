//! Request and response models for the Plaid API.
//!
//! Models are organized by Plaid product. All types implement
//! `Serialize` and `Deserialize` via `serde`.

pub mod auth;
pub mod common;
pub mod link;
pub mod sandbox;
