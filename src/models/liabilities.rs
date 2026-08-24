//! Models for the Liabilities product endpoints.

use crate::models::account::Account;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Request for `/liabilities/get`.
#[derive(Debug, Serialize)]
pub struct LiabilitiesGetRequest<'a> {
    /// The access token for the item.
    #[serde(serialize_with = "crate::models::common::serialize_secret_string")]
    pub access_token: &'a SecretString,
}

/// An APR applying to a credit account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditApr {
    /// The APR percentage.
    pub apr_percentage: f64,
    /// The type of balance to which the APR applies (e.g. `purchase_apr`).
    pub apr_type: String,
    /// The amount of money subject to the APR, if available.
    pub balance_subject_to_apr: Option<f64>,
}

/// A credit card liability for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditLiability {
    /// The Plaid account ID.
    pub account_id: String,
    /// The minimum payment due for the next billing cycle, if available.
    pub minimum_payment_amount: Option<f64>,
    /// The amount of the last payment, if available.
    pub last_payment_amount: Option<f64>,
    /// Whether the account is overdue, if available.
    pub is_overdue: Option<bool>,
    /// The due date for the next payment (ISO-8601 date), if available.
    pub next_payment_due_date: Option<String>,
    /// The outstanding balance on the last statement, if available.
    pub last_statement_balance: Option<f64>,
    /// The various interest rates applying to the account.
    #[serde(default)]
    pub aprs: Vec<CreditApr>,
}

/// A student loan liability for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentLiability {
    /// The Plaid account ID.
    pub account_id: String,
    /// The interest rate percentage of the loan.
    pub interest_rate_percentage: f64,
    /// The minimum payment due for the next billing cycle, if available.
    pub minimum_payment_amount: Option<f64>,
    /// The original principal balance of the loan, if available.
    pub origination_principal_amount: Option<f64>,
    /// The total amount of interest outstanding, if available.
    pub outstanding_interest_amount: Option<f64>,
    /// The due date for the next payment (ISO-8601 date), if available.
    pub next_payment_due_date: Option<String>,
    /// The name of the loan, if available.
    pub loan_name: Option<String>,
}

/// The interest rate of a mortgage.
// Field names mirror Plaid's JSON schema (`percentage`, `type`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageInterestRate {
    /// The interest rate percentage.
    pub percentage: f64,
    /// The type of interest rate (e.g. `fixed`, `variable`).
    #[serde(rename = "type")]
    pub rate_type: String,
}

/// A mortgage liability for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MortgageLiability {
    /// The Plaid account ID.
    pub account_id: String,
    /// The interest rate of the mortgage.
    pub interest_rate: MortgageInterestRate,
    /// The type of loan (e.g. `conventional`), if available.
    pub loan_type_description: Option<String>,
    /// The original principal balance of the loan, if available.
    pub origination_principal_amount: Option<f64>,
    /// The amount of the next monthly payment, if available.
    pub next_monthly_payment: Option<f64>,
    /// The year-to-date interest paid, if available.
    pub ytd_interest_paid: Option<f64>,
}

/// Liabilities data for an item, grouped by liability type.
///
/// Plaid omits empty categories, so each list defaults to empty.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Liabilities {
    /// Credit card liabilities.
    #[serde(default)]
    pub credit: Vec<CreditLiability>,
    /// Student loan liabilities.
    #[serde(default)]
    pub student: Vec<StudentLiability>,
    /// Mortgage liabilities.
    #[serde(default)]
    pub mortgage: Vec<MortgageLiability>,
}

/// Response from `/liabilities/get`.
#[derive(Debug, Deserialize)]
pub struct LiabilitiesGetResponse {
    /// The accounts associated with the item.
    pub accounts: Vec<Account>,
    /// The liabilities data for the item.
    pub liabilities: Liabilities,
    /// A unique identifier for the request.
    pub request_id: String,
}
