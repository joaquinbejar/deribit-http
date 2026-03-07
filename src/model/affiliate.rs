//! Affiliate program models for Deribit API
//!
//! This module contains types for affiliate program information.

use serde::{Deserialize, Serialize};

/// Affiliate program information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffiliateProgramInfo {
    /// Whether the affiliate program is enabled
    pub is_enabled: bool,
    /// Affiliate referral link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Number of referred users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_affiliates: Option<u64>,
    /// Total received commission amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affiliate_info_deserialization() {
        let json = r#"{
            "is_enabled": true,
            "link": "https://deribit.com/?ref=ABC123",
            "number_of_affiliates": 5,
            "received": 0.001
        }"#;

        let info: AffiliateProgramInfo = serde_json::from_str(json).expect("Failed to parse");
        assert!(info.is_enabled);
        assert!(info.link.is_some());
        assert_eq!(info.number_of_affiliates, Some(5));
    }

    #[test]
    fn test_affiliate_info_disabled() {
        let json = r#"{
            "is_enabled": false
        }"#;

        let info: AffiliateProgramInfo = serde_json::from_str(json).expect("Failed to parse");
        assert!(!info.is_enabled);
        assert!(info.link.is_none());
    }
}
