//! User lock models for Deribit API
//!
//! This module contains types for user account locks.

use serde::{Deserialize, Serialize};

/// User account lock information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserLock {
    /// Type of lock (e.g., "withdrawal", "trading")
    #[serde(rename = "type")]
    pub lock_type: String,
    /// Currency affected by the lock, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Reason for the lock
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Timestamp when the lock was applied in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    /// When the lock expires in milliseconds, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<u64>,
    /// Whether the lock is currently active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_lock_deserialization() {
        let json = r#"{
            "type": "withdrawal",
            "currency": "BTC",
            "reason": "security_review",
            "timestamp": 1550058362000,
            "locked": true
        }"#;

        let lock: UserLock = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(lock.lock_type, "withdrawal");
        assert_eq!(lock.currency, Some("BTC".to_string()));
        assert_eq!(lock.locked, Some(true));
    }

    #[test]
    fn test_user_lock_minimal() {
        let json = r#"{
            "type": "trading"
        }"#;

        let lock: UserLock = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(lock.lock_type, "trading");
        assert!(lock.currency.is_none());
    }
}
