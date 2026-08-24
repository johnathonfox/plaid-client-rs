#[path = "mock/server.rs"]
mod server;

use plaid_client_rs::{Config, Environment, PlaidClient};
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

fn sample_security() -> serde_json::Value {
    json!({
        "security_id": "sec-1",
        "name": "Vanguard Total Stock Market ETF",
        "ticker_symbol": "VTI",
        "type": "etf",
        "close_price": 220.5,
        "iso_currency_code": "USD"
    })
}

#[tokio::test]
async fn investments_holdings_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/investments/holdings/get",
        json!({
            "accounts": [{
                "account_id": "acc-inv-1",
                "name": "Plaid Brokerage",
                "official_name": "Plaid Brokerage Account",
                "type": "investment",
                "subtype": "brokerage",
                "balances": {
                    "available": null,
                    "current": 2403.5,
                    "iso_currency_code": "USD"
                }
            }],
            "holdings": [{
                "account_id": "acc-inv-1",
                "security_id": "sec-1",
                "quantity": 10.5,
                "cost_basis": 2100.0,
                "institution_value": 2315.25,
                "institution_price": 220.5,
                "iso_currency_code": "USD"
            }],
            "securities": [sample_security()],
            "request_id": "req-holdings"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .investments_holdings_get(&test_access_token())
        .await
        .unwrap();

    assert_eq!(response.accounts.len(), 1);
    assert_eq!(response.accounts[0].account_id, "acc-inv-1");
    assert_eq!(response.accounts[0].account_type, "investment");

    assert_eq!(response.holdings.len(), 1);
    let holding = &response.holdings[0];
    assert_eq!(holding.security_id, "sec-1");
    assert_eq!(holding.quantity, 10.5);
    assert_eq!(holding.cost_basis, Some(2100.0));
    assert_eq!(holding.institution_price, 220.5);
    assert_eq!(holding.iso_currency_code.as_deref(), Some("USD"));

    assert_eq!(response.securities.len(), 1);
    let security = &response.securities[0];
    assert_eq!(security.ticker_symbol.as_deref(), Some("VTI"));
    assert_eq!(security.security_type.as_deref(), Some("etf"));
    assert_eq!(security.close_price, Some(220.5));

    assert_eq!(response.request_id, "req-holdings");
}

#[tokio::test]
async fn investments_transactions_get_succeeds() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/investments/transactions/get",
        json!({
            "investment_transactions": [{
                "investment_transaction_id": "itxn-1",
                "account_id": "acc-inv-1",
                "security_id": "sec-1",
                "amount": 2205.0,
                "price": 220.5,
                "quantity": 10.0,
                "date": "2026-08-20",
                "name": "BUY Vanguard Total Stock Market ETF",
                "type": "buy",
                "subtype": null,
                "iso_currency_code": "USD"
            }, {
                "investment_transaction_id": "itxn-2",
                "account_id": "acc-inv-1",
                "security_id": null,
                "amount": -5000.0,
                "price": 0.0,
                "quantity": 0.0,
                "date": "2026-08-19",
                "name": "CASH DEPOSIT",
                "type": "cash",
                "subtype": "contribution",
                "iso_currency_code": "USD"
            }],
            "securities": [sample_security()],
            "request_id": "req-inv-txns"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    let response = client
        .investments_transactions_get(&test_access_token(), "2026-08-01", "2026-08-24")
        .await
        .unwrap();

    assert_eq!(response.investment_transactions.len(), 2);
    let buy = &response.investment_transactions[0];
    assert_eq!(buy.investment_transaction_id, "itxn-1");
    assert_eq!(buy.security_id.as_deref(), Some("sec-1"));
    assert_eq!(buy.amount, 2205.0);
    assert_eq!(buy.transaction_type, "buy");
    assert_eq!(buy.subtype, None);

    let cash = &response.investment_transactions[1];
    assert_eq!(cash.security_id, None);
    assert_eq!(cash.subtype.as_deref(), Some("contribution"));

    assert_eq!(response.securities.len(), 1);
    assert_eq!(response.request_id, "req-inv-txns");
}

#[tokio::test]
async fn investments_transactions_get_sends_date_range() {
    let mock = PlaidMockServer::new().await;
    mock.mock_ok(
        "/investments/transactions/get",
        json!({
            "investment_transactions": [],
            "securities": [],
            "request_id": "req-empty"
        }),
    )
    .await;

    let client = PlaidClient::new(test_config(&mock.uri())).unwrap();
    client
        .investments_transactions_get(&test_access_token(), "2026-01-01", "2026-06-30")
        .await
        .unwrap();

    let requests = mock.server.received_requests().await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["start_date"], "2026-01-01");
    assert_eq!(body["end_date"], "2026-06-30");
}
