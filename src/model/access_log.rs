//! Access log models for Deribit API
//!
//! This module contains types for account access history.

use serde::{Deserialize, Serialize};

/// Access log entry representing a single access event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessLogEntry {
    /// Timestamp of the access event in milliseconds
    pub timestamp: u64,
    /// IP address from which the access occurred
    pub ip: String,
    /// Action performed (e.g., "login", "api_call")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Result of the action (e.g., "success", "failure")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// Country code derived from IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// City derived from IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Log entry ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Additional data associated with the access event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Response for get_access_log endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessLogResponse {
    /// List of access log entries
    pub data: Vec<AccessLogEntry>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_log_entry_deserialization() {
        let json = r#"{
            "timestamp": 1550058362000,
            "ip": "192.168.1.1",
            "action": "login",
            "result": "success",
            "country": "US",
            "city": "New York"
        }"#;

        let entry: AccessLogEntry = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(entry.timestamp, 1550058362000);
        assert_eq!(entry.ip, "192.168.1.1");
        assert_eq!(entry.action, Some("login".to_string()));
    }

    #[test]
    fn test_access_log_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "timestamp": 1000000,
                    "ip": "10.0.0.1"
                }
            ],
            "continuation": "abc123"
        }"#;

        let response: AccessLogResponse = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.continuation, Some("abc123".to_string()));
    }
}
