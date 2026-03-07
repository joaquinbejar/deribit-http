//! Unit tests for common types

use deribit_http::model::types::{ApiError, AuthToken, RequestParams, TimeInForce};

#[test]
fn test_time_in_force_as_str() {
    assert_eq!(TimeInForce::GoodTilCancelled.as_str(), "good_til_cancelled");
    assert_eq!(TimeInForce::GoodTilDay.as_str(), "good_til_day");
    assert_eq!(TimeInForce::FillOrKill.as_str(), "fill_or_kill");
    assert_eq!(
        TimeInForce::ImmediateOrCancel.as_str(),
        "immediate_or_cancel"
    );
}

#[test]
fn test_time_in_force_serialization() {
    assert_eq!(
        serde_json::to_string(&TimeInForce::GoodTilCancelled).unwrap(),
        "\"good_til_cancelled\""
    );
    assert_eq!(
        serde_json::to_string(&TimeInForce::ImmediateOrCancel).unwrap(),
        "\"immediate_or_cancel\""
    );
    assert_eq!(
        serde_json::to_string(&TimeInForce::FillOrKill).unwrap(),
        "\"fill_or_kill\""
    );
}

#[test]
fn test_time_in_force_deserialization() {
    let gtc: TimeInForce = serde_json::from_str("\"good_til_cancelled\"").unwrap();
    let ioc: TimeInForce = serde_json::from_str("\"immediate_or_cancel\"").unwrap();
    let fok: TimeInForce = serde_json::from_str("\"fill_or_kill\"").unwrap();

    assert_eq!(gtc, TimeInForce::GoodTilCancelled);
    assert_eq!(ioc, TimeInForce::ImmediateOrCancel);
    assert_eq!(fok, TimeInForce::FillOrKill);
}

#[test]
fn test_time_in_force_equality() {
    assert_eq!(TimeInForce::GoodTilCancelled, TimeInForce::GoodTilCancelled);
    assert_ne!(TimeInForce::GoodTilCancelled, TimeInForce::FillOrKill);
}

#[test]
fn test_time_in_force_clone() {
    let tif = TimeInForce::GoodTilCancelled;
    let cloned = tif;
    assert_eq!(tif, cloned);
}

#[test]
fn test_time_in_force_copy() {
    let tif = TimeInForce::ImmediateOrCancel;
    let copied = tif;
    assert_eq!(tif, copied);
}

#[test]
fn test_api_error_deserialization() {
    let json = r#"{
        "code": 10001,
        "message": "Invalid request"
    }"#;

    let error: ApiError = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(error.code, 10001);
    assert_eq!(error.message, "Invalid request");
    assert!(error.data.is_none());
}

#[test]
fn test_api_error_with_data() {
    let json = r#"{
        "code": 10002,
        "message": "Missing parameter",
        "data": {"param": "instrument_name"}
    }"#;

    let error: ApiError = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(error.code, 10002);
    assert!(error.data.is_some());
}

#[test]
fn test_auth_token_deserialization() {
    let json = r#"{
        "access_token": "abc123",
        "token_type": "bearer",
        "expires_in": 3600,
        "scope": "account:read trade:read_write"
    }"#;

    let token: AuthToken = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(token.access_token, "abc123");
    assert_eq!(token.token_type, "bearer");
    assert_eq!(token.expires_in, 3600);
    assert_eq!(token.scope, "account:read trade:read_write");
    assert!(token.refresh_token.is_none());
}

#[test]
fn test_auth_token_with_refresh() {
    let json = r#"{
        "access_token": "abc123",
        "token_type": "bearer",
        "expires_in": 3600,
        "refresh_token": "refresh456",
        "scope": "account:read"
    }"#;

    let token: AuthToken = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(token.refresh_token, Some("refresh456".to_string()));
}

#[test]
fn test_request_params_new() {
    let params = RequestParams::new();
    assert!(params.is_empty());
}

#[test]
fn test_request_params_add() {
    let params = RequestParams::new()
        .add("instrument_name", "BTC-PERPETUAL")
        .add("amount", 100);

    assert!(!params.is_empty());
    let json = params.to_json();
    assert!(json.is_object());
}

#[test]
fn test_request_params_to_json() {
    let params = RequestParams::new().add("currency", "BTC");

    let json = params.to_json();
    assert!(json.get("currency").is_some());
}
