//! Unit tests for self-trading models

use deribit_http::model::self_trading::{SelfTradingConfig, SelfTradingMode};

#[test]
fn test_self_trading_mode_as_str_reject_taker() {
    let mode = SelfTradingMode::RejectTaker;
    assert_eq!(mode.as_str(), "reject_taker");
}

#[test]
fn test_self_trading_mode_as_str_cancel_maker() {
    let mode = SelfTradingMode::CancelMaker;
    assert_eq!(mode.as_str(), "cancel_maker");
}

#[test]
fn test_self_trading_mode_display_reject_taker() {
    let mode = SelfTradingMode::RejectTaker;
    assert_eq!(format!("{}", mode), "reject_taker");
}

#[test]
fn test_self_trading_mode_display_cancel_maker() {
    let mode = SelfTradingMode::CancelMaker;
    assert_eq!(format!("{}", mode), "cancel_maker");
}

#[test]
fn test_self_trading_mode_serialization_reject_taker() {
    let mode = SelfTradingMode::RejectTaker;
    let json = serde_json::to_string(&mode).expect("Failed to serialize");
    assert_eq!(json, "\"reject_taker\"");
}

#[test]
fn test_self_trading_mode_serialization_cancel_maker() {
    let mode = SelfTradingMode::CancelMaker;
    let json = serde_json::to_string(&mode).expect("Failed to serialize");
    assert_eq!(json, "\"cancel_maker\"");
}

#[test]
fn test_self_trading_mode_deserialization_reject_taker() {
    let json = "\"reject_taker\"";
    let mode: SelfTradingMode = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(mode, SelfTradingMode::RejectTaker);
}

#[test]
fn test_self_trading_mode_deserialization_cancel_maker() {
    let json = "\"cancel_maker\"";
    let mode: SelfTradingMode = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(mode, SelfTradingMode::CancelMaker);
}

#[test]
fn test_self_trading_config_full_deserialization() {
    let json = r#"{
        "mode": "reject_taker",
        "extended_to_subaccounts": true,
        "block_rfq_self_match_prevention": true
    }"#;

    let config: SelfTradingConfig = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(config.mode, SelfTradingMode::RejectTaker);
    assert!(config.extended_to_subaccounts);
    assert_eq!(config.block_rfq_self_match_prevention, Some(true));
}

#[test]
fn test_self_trading_config_minimal_deserialization() {
    let json = r#"{
        "mode": "cancel_maker",
        "extended_to_subaccounts": false
    }"#;

    let config: SelfTradingConfig = serde_json::from_str(json).expect("Failed to parse");

    assert_eq!(config.mode, SelfTradingMode::CancelMaker);
    assert!(!config.extended_to_subaccounts);
    assert_eq!(config.block_rfq_self_match_prevention, None);
}

#[test]
fn test_self_trading_config_serialization() {
    let config = SelfTradingConfig {
        mode: SelfTradingMode::RejectTaker,
        extended_to_subaccounts: true,
        block_rfq_self_match_prevention: Some(false),
    };

    let json = serde_json::to_string(&config).expect("Failed to serialize");

    assert!(json.contains("\"mode\":\"reject_taker\""));
    assert!(json.contains("\"extended_to_subaccounts\":true"));
    assert!(json.contains("\"block_rfq_self_match_prevention\":false"));
}

#[test]
fn test_self_trading_config_serialization_without_optional() {
    let config = SelfTradingConfig {
        mode: SelfTradingMode::CancelMaker,
        extended_to_subaccounts: false,
        block_rfq_self_match_prevention: None,
    };

    let json = serde_json::to_string(&config).expect("Failed to serialize");

    assert!(json.contains("\"mode\":\"cancel_maker\""));
    assert!(!json.contains("block_rfq_self_match_prevention"));
}

#[test]
fn test_self_trading_mode_equality() {
    assert_eq!(SelfTradingMode::RejectTaker, SelfTradingMode::RejectTaker);
    assert_eq!(SelfTradingMode::CancelMaker, SelfTradingMode::CancelMaker);
    assert_ne!(SelfTradingMode::RejectTaker, SelfTradingMode::CancelMaker);
}

#[test]
fn test_self_trading_mode_clone() {
    let mode = SelfTradingMode::RejectTaker;
    let cloned = mode;
    assert_eq!(mode, cloned);
}

#[test]
fn test_self_trading_mode_copy() {
    let mode = SelfTradingMode::CancelMaker;
    let copied = mode;
    assert_eq!(mode, copied);
}
