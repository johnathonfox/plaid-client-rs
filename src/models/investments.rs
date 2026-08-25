//! Models for the Investments product endpoints.

use crate::models::account::Account;
use rust_decimal::Decimal;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/investments/holdings/get`.
#[derive(Debug, Serialize)]
pub struct InvestmentsHoldingsGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// Request for `/investments/transactions/get`.
#[derive(Debug, Serialize)]
pub struct InvestmentsTransactionsGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
    /// The earliest date for which to fetch transactions (`YYYY-MM-DD`).
    pub start_date: String,
    /// The latest date for which to fetch transactions (`YYYY-MM-DD`).
    pub end_date: String,
}

/// A security holding for an investment account.
// Field names mirror Plaid's JSON schema (`account_id`, `security_id`, …).
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    /// The Plaid account ID the holding belongs to.
    pub account_id: String,
    /// The Plaid security ID of the held security.
    pub security_id: String,
    /// The quantity of the security held.
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// The total cost basis of the holding, if available.
    #[serde(default, with = "crate::models::common::decimal_option_json")]
    pub cost_basis: Option<Decimal>,
    /// The value of the holding, as reported by the institution.
    #[serde(with = "rust_decimal::serde::float")]
    pub institution_value: Decimal,
    /// The price of the security, as reported by the institution.
    #[serde(with = "rust_decimal::serde::float")]
    pub institution_price: Decimal,
    /// The ISO-4217 currency code of the institution price and value.
    pub iso_currency_code: Option<String>,
}

/// A security (e.g. a stock, ETF, or mutual fund).
// Field names mirror Plaid's JSON schema (`security_id`, `type`, …).
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    /// The Plaid security ID.
    pub security_id: String,
    /// The security name, if available.
    pub name: Option<String>,
    /// The ticker symbol, if available.
    pub ticker_symbol: Option<String>,
    /// The security type (e.g. `equity`, `etf`, `mutual fund`).
    #[serde(rename = "type")]
    pub security_type: Option<String>,
    /// The most recent closing price of the security, if available.
    #[serde(default, with = "crate::models::common::decimal_option_json")]
    pub close_price: Option<Decimal>,
    /// The ISO-4217 currency code of the close price.
    pub iso_currency_code: Option<String>,
}

/// An investment transaction (buy, sell, dividend, …).
// Field names mirror Plaid's JSON schema (`account_id`, `type`, …).
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentTransaction {
    /// The Plaid investment transaction ID.
    pub investment_transaction_id: String,
    /// The Plaid account ID the transaction belongs to.
    pub account_id: String,
    /// The Plaid security ID involved in the transaction, if any.
    pub security_id: Option<String>,
    /// The total value of the transaction (positive for buys, negative
    /// for sells).
    #[serde(with = "rust_decimal::serde::float")]
    pub amount: Decimal,
    /// The price of the security at the time of the transaction.
    #[serde(with = "rust_decimal::serde::float")]
    pub price: Decimal,
    /// The number of units transacted.
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// The date of the transaction (`YYYY-MM-DD`).
    pub date: String,
    /// The transaction description.
    pub name: String,
    /// The transaction type (e.g. `buy`, `sell`, `cash`).
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// The transaction subtype (e.g. `dividend`, `contribution`), if any.
    pub subtype: Option<String>,
    /// The ISO-4217 currency code of the transaction amount.
    pub iso_currency_code: Option<String>,
}

/// Response from `/investments/holdings/get`.
#[derive(Debug, Deserialize)]
pub struct InvestmentsHoldingsGetResponse {
    /// The investment accounts associated with the item.
    pub accounts: Vec<Account>,
    /// The holdings for the accounts.
    pub holdings: Vec<Holding>,
    /// The securities referenced by the holdings.
    pub securities: Vec<Security>,
    /// A unique identifier for the request.
    pub request_id: String,
}

/// Response from `/investments/transactions/get`.
#[derive(Debug, Deserialize)]
pub struct InvestmentsTransactionsGetResponse {
    /// The investment transactions in the requested date range.
    pub investment_transactions: Vec<InvestmentTransaction>,
    /// The securities referenced by the transactions.
    pub securities: Vec<Security>,
    /// A unique identifier for the request.
    pub request_id: String,
}
