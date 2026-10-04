//! Models for the Transactions product endpoints.

use rust_decimal::Decimal;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/transactions/sync`.
#[derive(Debug, Serialize)]
pub struct TransactionsSyncRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
    /// The cursor from a previous sync, if any. Omit for the first call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of transactions to fetch per call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// A single transaction.
// Field names mirror Plaid's JSON schema (`transaction_id`, …).
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// The Plaid transaction ID.
    pub transaction_id: String,
    /// The Plaid account ID this transaction belongs to.
    pub account_id: String,
    /// The transaction amount. Positive values move money out of the account.
    #[serde(with = "crate::models::common::decimal_json")]
    pub amount: Decimal,
    /// The date of the transaction (YYYY-MM-DD).
    pub date: String,
    /// The transaction description.
    pub name: String,
    /// The merchant name, if available.
    pub merchant_name: Option<String>,
    /// Whether the transaction is still pending.
    pub pending: bool,
}

/// A removed transaction reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovedTransaction {
    /// The ID of the removed transaction.
    pub transaction_id: String,
}

/// Response from `/transactions/sync`.
#[derive(Debug, Deserialize)]
pub struct TransactionsSyncResponse {
    /// Transactions added since the cursor.
    pub added: Vec<Transaction>,
    /// Transactions modified since the cursor.
    pub modified: Vec<Transaction>,
    /// Transactions removed since the cursor.
    pub removed: Vec<RemovedTransaction>,
    /// The cursor for the next sync call.
    pub next_cursor: String,
    /// Whether more updates are available immediately.
    pub has_more: bool,
    /// A unique identifier for the request.
    pub request_id: String,
}
