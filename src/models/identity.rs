//! Models for the Identity product endpoints.

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/identity/get`.
#[derive(Debug, Serialize)]
pub struct IdentityGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// Structured data for an identity address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAddressData {
    /// The street address.
    pub street: String,
    /// The city.
    pub city: String,
    /// The region or state, if available.
    pub region: Option<String>,
    /// The postal code, if available.
    pub postal_code: Option<String>,
    /// The country, if available.
    pub country: Option<String>,
}

/// An address associated with an identity owner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAddress {
    /// The structured address data.
    pub data: IdentityAddressData,
    /// Whether this is the owner's primary address.
    pub primary: bool,
}

/// An email address associated with an identity owner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityEmail {
    /// The email address.
    pub data: String,
    /// Whether this is the owner's primary email address.
    pub primary: bool,
    /// The email type (e.g. `primary`, `secondary`).
    #[serde(rename = "type")]
    pub email_type: String,
}

/// A phone number associated with an identity owner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityPhoneNumber {
    /// The phone number.
    pub data: String,
    /// Whether this is the owner's primary phone number.
    pub primary: bool,
    /// The phone number type (e.g. `mobile`, `home`).
    #[serde(rename = "type")]
    pub phone_type: String,
}

/// An owner (account holder) of an identity account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityOwner {
    /// The full names associated with the owner.
    #[serde(default)]
    pub names: Vec<String>,
    /// The addresses associated with the owner.
    #[serde(default)]
    pub addresses: Vec<IdentityAddress>,
    /// The email addresses associated with the owner.
    #[serde(default)]
    pub emails: Vec<IdentityEmail>,
    /// The phone numbers associated with the owner.
    #[serde(default)]
    pub phone_numbers: Vec<IdentityPhoneNumber>,
}

/// An account with identity (account holder) information.
// Field names mirror Plaid's JSON schema (`account_id`, `type`, …).
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAccount {
    /// The Plaid account ID.
    pub account_id: String,
    /// The account name.
    pub name: String,
    /// The official account name, if provided by the institution.
    pub official_name: Option<String>,
    /// The account type (e.g. `depository`, `credit`).
    #[serde(rename = "type")]
    pub account_type: String,
    /// The account subtype (e.g. `checking`, `savings`).
    pub subtype: Option<String>,
    /// The owners (account holders) of the account.
    #[serde(default)]
    pub owners: Vec<IdentityOwner>,
}

/// Response from `/identity/get`.
#[derive(Debug, Deserialize)]
pub struct IdentityGetResponse {
    /// The accounts associated with the item, including identity data.
    pub accounts: Vec<IdentityAccount>,
    /// A unique identifier for the request.
    pub request_id: String,
}
