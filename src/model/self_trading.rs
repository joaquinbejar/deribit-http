//! Self-trading configuration models for Deribit API
//!
//! This module contains types for self-trading prevention configuration.

use serde::{Deserialize, Serialize};

/// Self-trading prevention mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfTradingMode {
    /// Reject the taker order
    RejectTaker,
    /// Cancel the maker order
    CancelMaker,
}

impl SelfTradingMode {
    /// Returns the mode as a string for API requests
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RejectTaker => "reject_taker",
            Self::CancelMaker => "cancel_maker",
        }
    }
}

impl std::fmt::Display for SelfTradingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Self-trading configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfTradingConfig {
    /// The self-trading prevention mode
    pub mode: SelfTradingMode,
    /// Whether the config extends to subaccounts
    pub extended_to_subaccounts: bool,
    /// Whether to block RFQ self-match prevention
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_self_match_prevention: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_trading_mode_serialization() {
        let mode = SelfTradingMode::CancelMaker;
        let json = serde_json::to_string(&mode).expect("Failed to serialize");
        assert_eq!(json, "\"cancel_maker\"");
    }

    #[test]
    fn test_self_trading_mode_deserialization() {
        let json = "\"reject_taker\"";
        let mode: SelfTradingMode = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(mode, SelfTradingMode::RejectTaker);
    }

    #[test]
    fn test_self_trading_config_deserialization() {
        let json = r#"{
            "mode": "cancel_maker",
            "extended_to_subaccounts": true,
            "block_rfq_self_match_prevention": false
        }"#;

        let config: SelfTradingConfig = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(config.mode, SelfTradingMode::CancelMaker);
        assert!(config.extended_to_subaccounts);
        assert_eq!(config.block_rfq_self_match_prevention, Some(false));
    }
}
