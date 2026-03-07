//! Unit tests for option models

use deribit_http::model::option::{BasicGreeks, BasicOptionData, OptionInfo, Spread};
use deribit_http::prelude::OptionType;

#[test]
fn test_option_info_parse_call() {
    let result = OptionInfo::parse_from_string("BTC-28NOV25-108000-C");
    assert!(result.is_ok());

    let option = result.unwrap();
    assert_eq!(option.symbol, "BTC");
    assert_eq!(option.expiration_date, "28NOV25");
    assert_eq!(option.strike_price, 108000.0);
    assert_eq!(option.option_type, OptionType::Call);
}

#[test]
fn test_option_info_parse_put() {
    let result = OptionInfo::parse_from_string("ETH-15DEC25-5000-P");
    assert!(result.is_ok());

    let option = result.unwrap();
    assert_eq!(option.symbol, "ETH");
    assert_eq!(option.expiration_date, "15DEC25");
    assert_eq!(option.strike_price, 5000.0);
    assert_eq!(option.option_type, OptionType::Put);
}

#[test]
fn test_option_info_parse_lowercase_type() {
    let result = OptionInfo::parse_from_string("BTC-28NOV25-100000-p");
    assert!(result.is_ok());

    let option = result.unwrap();
    assert_eq!(option.option_type, OptionType::Put);
}

#[test]
fn test_option_info_parse_invalid_format_too_few_parts() {
    let result = OptionInfo::parse_from_string("BTC-28NOV25-C");
    assert!(result.is_err());
}

#[test]
fn test_option_info_parse_invalid_format_too_many_parts() {
    let result = OptionInfo::parse_from_string("BTC-28NOV25-100000-C-EXTRA");
    assert!(result.is_err());
}

#[test]
fn test_option_info_parse_invalid_strike_price() {
    let result = OptionInfo::parse_from_string("BTC-28NOV25-INVALID-C");
    assert!(result.is_err());
}

#[test]
fn test_option_info_parse_invalid_option_type() {
    let result = OptionInfo::parse_from_string("BTC-28NOV25-100000-X");
    assert!(result.is_err());
}

#[test]
fn test_option_info_parse_invalid_expiration_date() {
    let result = OptionInfo::parse_from_string("BTC-28N-100000-C");
    assert!(result.is_err());
}

#[test]
fn test_option_info_serialization() {
    let option = OptionInfo::parse_from_string("BTC-28NOV25-108000-C").unwrap();
    let json = serde_json::to_string(&option).expect("Failed to serialize");

    assert!(json.contains("BTC"));
    assert!(json.contains("28NOV25"));
    assert!(json.contains("108000"));
}

#[test]
fn test_spread_deserialization() {
    let json = r#"{
        "bid": 0.05,
        "ask": 0.06,
        "mid": 0.055
    }"#;

    let spread: Spread = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(spread.bid, Some(0.05));
    assert_eq!(spread.ask, Some(0.06));
    assert_eq!(spread.mid, Some(0.055));
}

#[test]
fn test_spread_with_none_values() {
    let json = r#"{
        "bid": null,
        "ask": 0.06,
        "mid": null
    }"#;

    let spread: Spread = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(spread.bid, None);
    assert_eq!(spread.ask, Some(0.06));
    assert_eq!(spread.mid, None);
}

#[test]
fn test_basic_greeks_deserialization() {
    let json = r#"{
        "delta_call": 0.65,
        "delta_put": -0.35,
        "gamma": 0.002
    }"#;

    let greeks: BasicGreeks = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(greeks.delta_call, Some(0.65));
    assert_eq!(greeks.delta_put, Some(-0.35));
    assert_eq!(greeks.gamma, Some(0.002));
}

#[test]
fn test_basic_option_data_deserialization() {
    let json = r#"{
        "strike_price": 100000.0,
        "call_bid": 0.05,
        "call_ask": 0.06,
        "put_bid": 0.03,
        "put_ask": 0.04,
        "implied_volatility": [0.65, 0.68],
        "delta_call": 0.55,
        "delta_put": -0.45,
        "gamma": 0.001,
        "volume": 150.5,
        "open_interest": 1000.0,
        "risk_free_rate": 0.05
    }"#;

    let data: BasicOptionData = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(data.strike_price, 100000.0);
    assert_eq!(data.call_bid, Some(0.05));
    assert_eq!(data.put_ask, Some(0.04));
    assert_eq!(data.volume, 150.5);
    assert_eq!(data.open_interest, 1000.0);
}
