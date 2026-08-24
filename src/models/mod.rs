//! Request and response models for the Plaid API.
//!
//! Models are organized by Plaid product. All types implement
//! `Serialize` and `Deserialize` via `serde`.

pub mod account;
pub mod accounts;
pub mod auth;
pub mod balance;
pub mod common;
pub mod identity;
pub mod institutions;
pub mod investments;
pub mod item;
pub mod liabilities;
pub mod link;
pub mod payment_initiation;
pub mod sandbox;
pub mod transactions;
pub mod transfer;
pub mod webhook;
