//! Endpoint-specific implementations for the Plaid API.
//!
//! Each module corresponds to a Plaid product area.

pub mod accounts;
pub mod auth;
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
pub mod webhooks;
