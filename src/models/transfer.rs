//! Models for the Transfer product endpoints.

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// The direction of a transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransferType {
    /// Money moves into the account.
    Credit,
    /// Money moves out of the account.
    Debit,
    /// A value this version of the client doesn't know.
    #[serde(other)]
    Unknown,
}

/// The network a transfer runs over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferNetwork {
    /// Standard ACH.
    #[serde(rename = "ach")]
    Ach,
    /// Same-day ACH.
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    /// Real-time payments.
    #[serde(rename = "rtp")]
    Rtp,
    /// Domestic wire.
    #[serde(rename = "wire")]
    Wire,
    /// A value this version of the client doesn't know.
    #[serde(other)]
    Unknown,
}

/// The ACH SEC code (class) of a transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransferAchClass {
    /// Prearranged payment and deposit (consumer).
    Ppd,
    /// Corporate credit or debit (business).
    Ccd,
    /// Telephone-initiated entry.
    Tel,
    /// Internet-initiated entry.
    Web,
    /// A value this version of the client doesn't know.
    #[serde(other)]
    Unknown,
}

/// User details supplied with a transfer request.
#[derive(Debug, Serialize)]
pub struct TransferUser<'a> {
    /// The user's legal name.
    pub legal_name: &'a str,
}

/// The shared parameters of transfer authorization and creation.
///
/// Serializes to the common body shape of `/transfer/authorization/create`
/// and `/transfer/create`.
#[derive(Debug, Serialize)]
pub struct TransferParams<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
    /// The Plaid account ID to transfer funds to or from.
    pub account_id: &'a str,
    /// The direction of the transfer.
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    /// The transfer network.
    pub network: TransferNetwork,
    /// The transfer amount as a decimal string (e.g. `"12.34"`).
    pub amount: &'a str,
    /// The ACH class.
    pub ach_class: TransferAchClass,
    /// The user initiating the transfer.
    pub user: TransferUser<'a>,
}

/// Request for `/transfer/create`.
#[derive(Debug, Serialize)]
pub struct TransferCreateRequest<'a> {
    /// The shared transfer parameters.
    #[serde(flatten)]
    pub params: TransferParams<'a>,
    /// The authorization ID returned by `/transfer/authorization/create`.
    pub authorization_id: &'a str,
    /// The transfer description, visible on the bank statement.
    pub description: &'a str,
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
    /// The direction of the transfer.
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    /// The transfer network.
    pub network: TransferNetwork,
    /// The transfer amount as a decimal string.
    pub amount: String,
    /// The transfer description, visible on the bank statement.
    pub description: String,
    /// The status of the transfer (e.g. `"pending"`, `"posted"`).
    pub status: String,
    /// The ACH class.
    pub ach_class: TransferAchClass,
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
