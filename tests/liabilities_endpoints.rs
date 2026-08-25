#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::{Config, Environment, PlaidClient};
use rust_decimal::dec;
use secrecy::SecretString;
use serde_json::json;
use server::PlaidMockServer;

fn test_config(base_url: &str) -> Config {
    Config {
        client_id: SecretString::from("test-client-id"),
        secret: SecretString::from("test-secret"),
        environment: Environment::Sandbox,
        base_url: Some(base_url.parse().unwrap()),
        ..Config::default()
    }
}

fn test_access_token() -> SecretString {
    SecretString::from("access-sandbox-xxx")
}

#[tokio::test]
async fn liabilities_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/liabilities/get",
        json!({
            "accounts": [PlaidMockServer::sample_account()],
            "liabilities": {
                "credit": [{
                    "account_id": "acc-1",
                    "minimum_payment_amount": 35.0,
                    "last_payment_amount": 100.0,
                    "is_overdue": false,
                    "next_payment_due_date": "2026-09-15",
                    "last_statement_balance": 1250.75,
                    "aprs": [{
                        "apr_percentage": 24.99,
                        "apr_type": "purchase_apr",
                        "balance_subject_to_apr": 1250.75
                    }]
                }],
                "student": [{
                    "account_id": "acc-2",
                    "interest_rate_percentage": 5.25,
                    "minimum_payment_amount": 150.0,
                    "origination_principal_amount": 30000.0,
                    "outstanding_interest_amount": 120.50,
                    "next_payment_due_date": "2026-09-01",
                    "loan_name": "Direct Subsidized Loan"
                }],
                "mortgage": [{
                    "account_id": "acc-3",
                    "interest_rate": {
                        "percentage": 3.875,
                        "type": "fixed"
                    },
                    "loan_type_description": "conventional",
                    "origination_principal_amount": 350000.0,
                    "next_monthly_payment": 1645.32,
                    "ytd_interest_paid": 8100.0
                }]
            },
            "request_id": "req-liabilities"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.liabilities_get(&test_access_token()).await.unwrap();

    assert_eq!(response.accounts.len(), 1);
    assert_eq!(response.accounts[0].account_id, "acc-1");

    let credit = &response.liabilities.credit[0];
    assert_eq!(credit.account_id, "acc-1");
    assert_eq!(credit.minimum_payment_amount, Some(dec!(35.0)));
    assert_eq!(credit.is_overdue, Some(false));
    assert_eq!(credit.aprs[0].apr_percentage, 24.99);
    assert_eq!(credit.aprs[0].apr_type, "purchase_apr");
    assert_eq!(credit.aprs[0].balance_subject_to_apr, Some(dec!(1250.75)));

    let student = &response.liabilities.student[0];
    assert_eq!(student.account_id, "acc-2");
    assert_eq!(student.interest_rate_percentage, 5.25);
    assert_eq!(student.loan_name.as_deref(), Some("Direct Subsidized Loan"));

    let mortgage = &response.liabilities.mortgage[0];
    assert_eq!(mortgage.account_id, "acc-3");
    assert_eq!(mortgage.interest_rate.percentage, 3.875);
    assert_eq!(mortgage.interest_rate.rate_type, "fixed");
    assert_eq!(mortgage.ytd_interest_paid, Some(dec!(8100.0)));

    assert_eq!(response.request_id, "req-liabilities");
}

#[tokio::test]
async fn liabilities_get_defaults_missing_categories() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/liabilities/get",
        json!({
            "accounts": [PlaidMockServer::sample_account()],
            "liabilities": {
                "mortgage": [{
                    "account_id": "acc-3",
                    "interest_rate": {
                        "percentage": 3.875,
                        "type": "fixed"
                    }
                }]
            },
            "request_id": "req-liabilities"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client.liabilities_get(&test_access_token()).await.unwrap();

    assert!(response.liabilities.credit.is_empty());
    assert!(response.liabilities.student.is_empty());
    assert_eq!(response.liabilities.mortgage.len(), 1);
}
