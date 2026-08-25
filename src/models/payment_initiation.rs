//! Models for the Payment Initiation product endpoints.
//!
//! These endpoints authenticate with the client's `client_id` and
//! `secret` (injected by [`crate::PlaidClient`]), not an item's
//! `access_token`.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Bacs account numbers for a UK recipient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientBacs {
    /// The account number of the account. Maximum of 10 characters.
    pub account: String,
    /// The 6-character sort code of the account.
    pub sort_code: String,
}

/// The amount and currency of a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAmount {
    /// The amount of the payment, with at most two digits of precision.
    #[serde(with = "rust_decimal::serde::float")]
    pub value: Decimal,
    /// The ISO-4217 currency code of the payment (e.g. `GBP`, `EUR`).
    pub currency: String,
}

/// Request for `/payment_initiation/recipient/create`.
#[derive(Debug, Serialize)]
pub struct PaymentInitiationRecipientCreateRequest<'a> {
    /// The name of the recipient.
    pub name: &'a str,
    /// The IBAN for the recipient. Required if `bacs` is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<&'a str>,
    /// Bacs account number and sort code. Required if `iban` is not
    /// provided, or for domestic GBP-denominated payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
}

/// Request for `/payment_initiation/recipient/get`.
#[derive(Debug, Serialize)]
pub struct PaymentInitiationRecipientGetRequest<'a> {
    /// The ID of the recipient.
    pub recipient_id: &'a str,
}

/// Request for `/payment_initiation/payment/create`.
#[derive(Debug, Serialize)]
pub struct PaymentInitiationPaymentCreateRequest<'a> {
    /// The ID of the recipient the payment is for.
    pub recipient_id: &'a str,
    /// A reference for the payment: alphanumeric, at most 18 characters.
    pub reference: &'a str,
    /// The amount and currency of the payment.
    pub amount: PaymentAmount,
}

/// Request for `/payment_initiation/payment/get`.
#[derive(Debug, Serialize)]
pub struct PaymentInitiationPaymentGetRequest<'a> {
    /// The `payment_id` returned by `/payment_initiation/payment/create`.
    pub payment_id: &'a str,
}

/// Request for `/payment_initiation/payment/list`.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentInitiationPaymentListRequest {
    /// The maximum number of payments to return. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    /// A string in RFC 3339 format. Only payments created before the
    /// cursor are returned. Omit for the first call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Response from `/payment_initiation/recipient/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientCreateResponse {
    /// A unique ID identifying the recipient.
    pub recipient_id: String,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Response from `/payment_initiation/recipient/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientGetResponse {
    /// The ID of the recipient.
    pub recipient_id: String,
    /// The name of the recipient.
    pub name: String,
    /// The IBAN for the recipient, if one was provided at creation.
    pub iban: Option<String>,
    /// The Bacs account number and sort code, if provided at creation.
    pub bacs: Option<RecipientBacs>,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Response from `/payment_initiation/payment/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCreateResponse {
    /// A unique ID identifying the payment.
    pub payment_id: String,
    /// The status of the payment; always
    /// `PAYMENT_STATUS_INPUT_NEEDED` from this endpoint.
    pub status: String,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Response from `/payment_initiation/payment/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGetResponse {
    /// The ID of the payment.
    pub payment_id: String,
    /// The ID of the recipient.
    pub recipient_id: String,
    /// A reference for the payment.
    pub reference: String,
    /// The amount and currency of the payment.
    pub amount: PaymentAmount,
    /// The status of the payment (e.g. `PAYMENT_STATUS_INITIATED`).
    pub status: String,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// A single payment as returned in `/payment_initiation/payment/list`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSummary {
    /// The ID of the payment.
    pub payment_id: String,
    /// A reference for the payment.
    pub reference: String,
    /// The amount and currency of the payment.
    pub amount: PaymentAmount,
    /// The status of the payment (e.g. `PAYMENT_STATUS_EXECUTED`).
    pub status: String,
}

/// Response from `/payment_initiation/payment/list`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentListResponse {
    /// The payments that have been created, most recent first.
    pub payments: Vec<PaymentSummary>,
    /// The cursor to pass as `cursor` to fetch the next page. `None`
    /// when there are no further payments.
    pub next_cursor: Option<String>,
    /// A unique identifier for the request.
    pub request_id: String,
}
