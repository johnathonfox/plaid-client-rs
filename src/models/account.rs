//! Models for bank accounts, shared across products (Auth, Balance, …).

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A bank account associated with an item.
// Field names mirror Plaid's JSON schema (`account_id`, `type`, …).
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
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
    /// The last 2-4 alphanumeric characters of the account number, when
    /// the institution provides them.
    #[serde(default)]
    pub mask: Option<String>,
    /// The current balance information.
    pub balances: Balances,
}

/// Balance information for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balances {
    /// The amount of funds available, if provided by the institution.
    #[serde(default, with = "crate::models::common::decimal_option_json")]
    pub available: Option<Decimal>,
    /// The total amount of funds in the account.
    #[serde(default, with = "crate::models::common::decimal_option_json")]
    pub current: Option<Decimal>,
    /// The credit limit for credit accounts, or the overdraft limit for
    /// depository accounts, when the institution reports one.
    #[serde(default, with = "crate::models::common::decimal_option_json")]
    pub limit: Option<Decimal>,
    /// The ISO-4217 currency code (e.g. `USD`).
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code (e.g. a cryptocurrency) when
    /// `iso_currency_code` is `None`.
    #[serde(default)]
    pub unofficial_currency_code: Option<String>,
}
