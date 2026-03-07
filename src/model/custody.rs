//! Custody account models for Deribit API
//!
//! This module contains types for custody accounts.

use serde::{Deserialize, Serialize};

/// Custody account information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustodyAccount {
    /// Custody account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Currency of the custody account
    pub currency: String,
    /// Current balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// Custody provider name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Account status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Account creation timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custody_account_deserialization() {
        let json = r#"{
            "id": "custody_123",
            "currency": "BTC",
            "balance": 1.5,
            "provider": "fireblocks",
            "status": "active"
        }"#;

        let account: CustodyAccount = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(account.currency, "BTC");
        assert_eq!(account.balance, Some(1.5));
        assert_eq!(account.provider, Some("fireblocks".to_string()));
    }

    #[test]
    fn test_custody_account_minimal() {
        let json = r#"{
            "currency": "ETH"
        }"#;

        let account: CustodyAccount = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(account.currency, "ETH");
        assert!(account.id.is_none());
    }
}
