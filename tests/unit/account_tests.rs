//! Unit tests for account models

use deribit_http::model::account::{
    CurrencyPortfolio, Portfolio, Subaccount, TradingProductDetail,
};

#[test]
fn test_portfolio_new() {
    let portfolio = Portfolio::new("BTC".to_string());

    assert_eq!(portfolio.currency, "BTC");
    assert!(portfolio.accounts.is_empty());
    assert_eq!(portfolio.total_usd_value, None);
    assert!(!portfolio.cross_margin_enabled);
}

#[test]
fn test_subaccount_deserialization() {
    let json = r#"{
        "email": "test@example.com",
        "id": 12345,
        "login_enabled": true,
        "receive_notifications": false,
        "system_name": "test_system",
        "type": "subaccount",
        "username": "test_user"
    }"#;

    let subaccount: Subaccount = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(subaccount.email, "test@example.com");
    assert_eq!(subaccount.id, 12345);
    assert!(subaccount.login_enabled);
    assert!(!subaccount.receive_notifications);
    assert_eq!(subaccount.username, "test_user");
}

#[test]
fn test_subaccount_with_portfolio() {
    let json = r#"{
        "email": "test@example.com",
        "id": 12345,
        "login_enabled": true,
        "receive_notifications": true,
        "system_name": "main",
        "type": "subaccount",
        "username": "trader1",
        "portfolio": {
            "BTC": {
                "available_funds": 1.5,
                "available_withdrawal_funds": 1.0,
                "balance": 2.0,
                "currency": "BTC",
                "equity": 2.0,
                "initial_margin": 0.5,
                "locked_balance": 0.0,
                "maintenance_margin": 0.3,
                "margin_balance": 2.0,
                "spot_reserve": 0.0,
                "additional_reserve": 0.0
            }
        }
    }"#;

    let subaccount: Subaccount = serde_json::from_str(json).expect("Failed to parse");

    assert!(subaccount.portfolio.is_some());
    let portfolio = subaccount.portfolio.unwrap();
    assert!(portfolio.contains_key("BTC"));
    let btc_portfolio = portfolio.get("BTC").unwrap();
    assert_eq!(btc_portfolio.currency, "BTC");
    assert_eq!(btc_portfolio.balance, 2.0);
}

#[test]
fn test_currency_portfolio_deserialization() {
    let json = r#"{
        "available_funds": 5.0,
        "available_withdrawal_funds": 4.5,
        "balance": 6.0,
        "currency": "ETH",
        "equity": 6.0,
        "initial_margin": 1.0,
        "locked_balance": 0.5,
        "maintenance_margin": 0.8,
        "margin_balance": 6.0,
        "spot_reserve": 0.1,
        "additional_reserve": 0.0
    }"#;

    let portfolio: CurrencyPortfolio = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(portfolio.currency, "ETH");
    assert_eq!(portfolio.balance, 6.0);
    assert_eq!(portfolio.available_funds, 5.0);
    assert_eq!(portfolio.initial_margin, 1.0);
}

#[test]
fn test_trading_product_detail_deserialization() {
    let json = r#"{
        "enabled": true,
        "product": "futures",
        "overwriteable": false
    }"#;

    let detail: TradingProductDetail = serde_json::from_str(json).expect("Failed to parse");

    assert!(detail.enabled);
    assert_eq!(detail.product, "futures");
    assert!(!detail.overwriteable);
}
