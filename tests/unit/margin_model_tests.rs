//! Unit tests for margin model types

use deribit_http::model::margin_model::{ChangeMarginModelResponse, MarginModel};

#[test]
fn test_margin_model_as_str_cross_pm() {
    let model = MarginModel::CrossPm;
    assert_eq!(model.as_str(), "cross_pm");
}

#[test]
fn test_margin_model_as_str_cross_sm() {
    let model = MarginModel::CrossSm;
    assert_eq!(model.as_str(), "cross_sm");
}

#[test]
fn test_margin_model_as_str_segregated_pm() {
    let model = MarginModel::SegregatedPm;
    assert_eq!(model.as_str(), "segregated_pm");
}

#[test]
fn test_margin_model_as_str_segregated_sm() {
    let model = MarginModel::SegregatedSm;
    assert_eq!(model.as_str(), "segregated_sm");
}

#[test]
fn test_margin_model_display() {
    assert_eq!(format!("{}", MarginModel::CrossPm), "cross_pm");
    assert_eq!(format!("{}", MarginModel::CrossSm), "cross_sm");
    assert_eq!(format!("{}", MarginModel::SegregatedPm), "segregated_pm");
    assert_eq!(format!("{}", MarginModel::SegregatedSm), "segregated_sm");
}

#[test]
fn test_margin_model_serialization() {
    let model = MarginModel::CrossPm;
    let json = serde_json::to_string(&model).expect("Failed to serialize");
    assert_eq!(json, "\"cross_pm\"");
}

#[test]
fn test_margin_model_deserialization() {
    let json = "\"segregated_sm\"";
    let model: MarginModel = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(model, MarginModel::SegregatedSm);
}

#[test]
fn test_change_margin_model_response_deserialization() {
    let json = r#"{
        "margin_model": "cross_pm",
        "success": true
    }"#;

    let response: ChangeMarginModelResponse = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(response.margin_model, "cross_pm");
    assert_eq!(response.success, Some(true));
}

#[test]
fn test_change_margin_model_response_minimal() {
    let json = r#"{
        "margin_model": "segregated_sm"
    }"#;

    let response: ChangeMarginModelResponse = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(response.margin_model, "segregated_sm");
    assert_eq!(response.success, None);
}

#[test]
fn test_margin_model_clone() {
    let model = MarginModel::CrossPm;
    let cloned = model;
    assert_eq!(model, cloned);
}

#[test]
fn test_margin_model_copy() {
    let model = MarginModel::CrossSm;
    let copied = model;
    assert_eq!(model, copied);
}

#[test]
fn test_margin_model_equality() {
    assert_eq!(MarginModel::CrossPm, MarginModel::CrossPm);
    assert_ne!(MarginModel::CrossPm, MarginModel::CrossSm);
}
