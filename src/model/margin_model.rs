//! Margin model types for Deribit API
//!
//! This module contains types for margin model configuration.

use serde::{Deserialize, Serialize};

/// Available margin models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarginModel {
    /// Cross Portfolio Margin
    CrossPm,
    /// Segregated Portfolio Margin
    SegregatedPm,
    /// Cross Standard Margin
    CrossSm,
    /// Segregated Standard Margin
    SegregatedSm,
}

impl MarginModel {
    /// Returns the margin model as a string for API requests
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CrossPm => "cross_pm",
            Self::SegregatedPm => "segregated_pm",
            Self::CrossSm => "cross_sm",
            Self::SegregatedSm => "segregated_sm",
        }
    }
}

impl std::fmt::Display for MarginModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Response for change_margin_model endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeMarginModelResponse {
    /// The new margin model
    pub margin_model: String,
    /// Whether the change was successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_model_serialization() {
        let model = MarginModel::CrossPm;
        let json = serde_json::to_string(&model).expect("Failed to serialize");
        assert_eq!(json, "\"cross_pm\"");
    }

    #[test]
    fn test_margin_model_deserialization() {
        let json = "\"segregated_pm\"";
        let model: MarginModel = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(model, MarginModel::SegregatedPm);
    }

    #[test]
    fn test_margin_model_as_str() {
        assert_eq!(MarginModel::CrossSm.as_str(), "cross_sm");
        assert_eq!(MarginModel::SegregatedSm.as_str(), "segregated_sm");
    }

    #[test]
    fn test_change_margin_model_response() {
        let json = r#"{
            "margin_model": "cross_pm",
            "success": true
        }"#;

        let response: ChangeMarginModelResponse =
            serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(response.margin_model, "cross_pm");
        assert_eq!(response.success, Some(true));
    }
}
