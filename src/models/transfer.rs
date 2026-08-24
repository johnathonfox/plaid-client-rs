//! Models for the Transfer product endpoints.

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// User details supplied with a transfer request.
#[derive(Debug, Serialize)]
pub struct TransferUser<'a> {
    /// The user's legal name.
    pub legal_name: &'a str,
}

/// Request for `/transfer/authorization/create`.
#[derive(Debug, Serialize)]
pub struct TransferAuthorizationCreateRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
    /// The Plaid account ID to transfer funds to or from.
    pub account_id: &'a str,
    /// The type of transfer (`"debit"` or `"credit"`).
    #[serde(rename = "type")]
    pub transfer_type: &'a str,
    /// The transfer network (`"ach"` or `"same-day-ach"`).
    pub network: &'a str,
    /// The transfer amount as a decimal string (e.g. `"12.34"`).
    pub amount: &'a str,
    /// The ACH class (`"ppd"`, `"ccd"`, `"tel"`, or `"web"`).
    pub ach_class: &'a str,
    /// The user initiating the transfer.
    pub user: TransferUser<'a>,
}

/// Request for `/transfer/create`.
#[derive(Debug, Serialize)]
pub struct TransferCreateRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
    /// The Plaid account ID to transfer funds to or from.
    pub account_id: &'a str,
    /// The authorization ID returned by `/transfer/authorization/create`.
    pub authorization_id: &'a str,
    /// The type of transfer (`"debit"` or `"credit"`).
    #[serde(rename = "type")]
    pub transfer_type: &'a str,
    /// The transfer network (`"ach"` or `"same-day-ach"`).
    pub network: &'a str,
    /// The transfer amount as a decimal string (e.g. `"12.34"`).
    pub amount: &'a str,
    /// The transfer description, visible on the bank statement.
    pub description: &'a str,
    /// The ACH class (`"ppd"`, `"ccd"`, `"tel"`, or `"web"`).
    pub ach_class: &'a str,
    /// The user initiating the transfer.
    pub user: TransferUser<'a>,
    /// A random key for idempotent creation; reused keys return the
    /// existing transfer.
    pub idempotency_key: &'a str,
}

/// Request for `/transfer/get`.
#[derive(Debug, Serialize)]
pub struct TransferGetRequest<'a> {
    /// The ID of the transfer to retrieve.
    pub transfer_id: &'a str,
}

/// An authorization decision for a proposed transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorization {
    /// The ID of the transfer authorization.
    pub id: String,
    /// The datetime the authorization was created, in RFC 3339 format.
    pub created: String,
    /// The authorization decision (e.g. `"approved"` or `"declined"`).
    pub authorization_decision: String,
}

/// A transfer of funds between accounts.
// Field names mirror Plaid's JSON schema.
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    /// The ID of the transfer.
    pub id: String,
    /// The Plaid account ID funds are transferred to or from.
    pub account_id: String,
    /// The ID of the authorization that created this transfer, if any.
    pub authorization_id: Option<String>,
    /// The type of transfer (`"debit"` or `"credit"`).
    #[serde(rename = "type")]
    pub transfer_type: String,
    /// The transfer network (`"ach"` or `"same-day-ach"`).
    pub network: String,
    /// The transfer amount as a decimal string.
    pub amount: String,
    /// The transfer description, visible on the bank statement.
    pub description: String,
    /// The status of the transfer (e.g. `"pending"`, `"posted"`).
    pub status: String,
    /// The ACH class (`"ppd"`, `"ccd"`, `"tel"`, or `"web"`).
    pub ach_class: String,
    /// The datetime the transfer was created, in RFC 3339 format.
    pub created: String,
}

/// Response from `/transfer/authorization/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateResponse {
    /// The authorization for the proposed transfer.
    pub authorization: TransferAuthorization,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Response from `/transfer/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCreateResponse {
    /// The created transfer.
    pub transfer: Transfer,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Response from `/transfer/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferGetResponse {
    /// The requested transfer.
    pub transfer: Transfer,
    /// A unique identifier for the request.
    pub request_id: String,
}
