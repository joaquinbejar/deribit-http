//! Unit tests for trading products models

use deribit_http::model::trading_products::TradingProduct;

#[test]
fn test_trading_product_as_str_futures() {
    let product = TradingProduct::Futures;
    assert_eq!(product.as_str(), "futures");
}

#[test]
fn test_trading_product_as_str_options() {
    let product = TradingProduct::Options;
    assert_eq!(product.as_str(), "options");
}

#[test]
fn test_trading_product_as_str_spots() {
    let product = TradingProduct::Spots;
    assert_eq!(product.as_str(), "spots");
}

#[test]
fn test_trading_product_as_str_future_combos() {
    let product = TradingProduct::FutureCombos;
    assert_eq!(product.as_str(), "future_combos");
}

#[test]
fn test_trading_product_as_str_option_combos() {
    let product = TradingProduct::OptionCombos;
    assert_eq!(product.as_str(), "option_combos");
}

#[test]
fn test_trading_product_display() {
    assert_eq!(format!("{}", TradingProduct::Futures), "futures");
    assert_eq!(format!("{}", TradingProduct::Options), "options");
    assert_eq!(format!("{}", TradingProduct::Spots), "spots");
    assert_eq!(format!("{}", TradingProduct::FutureCombos), "future_combos");
    assert_eq!(format!("{}", TradingProduct::OptionCombos), "option_combos");
}

#[test]
fn test_trading_product_serialization() {
    let product = TradingProduct::Futures;
    let json = serde_json::to_string(&product).expect("Failed to serialize");
    assert_eq!(json, "\"futures\"");
}

#[test]
fn test_trading_product_serialization_combos() {
    let product = TradingProduct::FutureCombos;
    let json = serde_json::to_string(&product).expect("Failed to serialize");
    assert_eq!(json, "\"future_combos\"");
}

#[test]
fn test_trading_product_deserialization() {
    let json = "\"options\"";
    let product: TradingProduct = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(product, TradingProduct::Options);
}

#[test]
fn test_trading_product_deserialization_combos() {
    let json = "\"option_combos\"";
    let product: TradingProduct = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(product, TradingProduct::OptionCombos);
}

#[test]
fn test_trading_product_clone() {
    let product = TradingProduct::Futures;
    let cloned = product;
    assert_eq!(product, cloned);
}

#[test]
fn test_trading_product_copy() {
    let product = TradingProduct::Options;
    let copied = product;
    assert_eq!(product, copied);
}

#[test]
fn test_trading_product_equality() {
    assert_eq!(TradingProduct::Futures, TradingProduct::Futures);
    assert_ne!(TradingProduct::Futures, TradingProduct::Options);
    assert_ne!(TradingProduct::Options, TradingProduct::Spots);
}
