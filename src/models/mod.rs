//! Request and response models for the Plaid API.
//!
//! Models are organized by Plaid product. All types implement
//! `Serialize` and `Deserialize` via `serde`.

pub mod account;
pub mod auth;
pub mod balance;
pub mod common;
pub mod item;
pub mod link;
pub mod sandbox;
pub mod transactions;
pub mod webhook;
