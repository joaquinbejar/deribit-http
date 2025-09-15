//! HTTP-specific types and models

use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API error structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code number
    pub code: i32,
    /// Human-readable error message
    pub message: String,
    /// Additional error data
    pub data: Option<serde_json::Value>,
}

/// Authentication token structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// OAuth2 access token
    pub access_token: String,
    /// Token type (typically "Bearer")
    pub token_type: String,
    /// Token expiration time in seconds
    pub expires_in: u64,
    /// Optional refresh token for renewing access
    pub refresh_token: Option<String>,
    /// Token scope permissions
    pub scope: String,
}

/// Request parameters
#[derive(DebugPretty, DisplaySimple, Clone, Default, Serialize, Deserialize)]
pub struct RequestParams {
    params: HashMap<String, serde_json::Value>,
}

impl RequestParams {
    /// Create new empty parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a parameter
    pub fn add<T: Serialize>(mut self, key: &str, value: T) -> Self {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.params.insert(key.to_string(), json_value);
        }
        self
    }

    /// Convert to JSON value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.params).unwrap_or(serde_json::Value::Null)
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}

/// Time in force enumeration
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Order remains active until explicitly cancelled
    #[serde(rename = "good_til_cancelled")]
    GoodTilCancelled,
    /// Order expires at the end of the trading day
    #[serde(rename = "good_til_day")]
    GoodTilDay,
    /// Order must be filled immediately and completely or cancelled
    #[serde(rename = "fill_or_kill")]
    FillOrKill,
    /// Order must be filled immediately, partial fills allowed, remaining cancelled
    #[serde(rename = "immediate_or_cancel")]
    ImmediateOrCancel,
}

impl TimeInForce {
    /// Returns the string representation of the time in force value
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::GoodTilCancelled => "good_til_cancelled",
            TimeInForce::GoodTilDay => "good_til_day",
            TimeInForce::FillOrKill => "fill_or_kill",
            TimeInForce::ImmediateOrCancel => "immediate_or_cancel",
        }
    }
}

/// Withdrawal information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    /// Withdrawal address
    pub address: String,
    /// Withdrawal amount
    pub amount: f64,
    /// Currency of the withdrawal
    pub currency: String,
    /// Withdrawal fee
    pub fee: f64,
    /// Unique withdrawal identifier
    pub id: u64,
    /// Withdrawal priority level
    pub priority: String,
    /// Current state of the withdrawal
    pub state: String,
    /// Timestamp when withdrawal was created
    pub created_timestamp: u64,
    /// Timestamp when withdrawal was last updated
    pub updated_timestamp: Option<u64>,
    /// Transaction ID on the blockchain
    pub transaction_id: Option<String>,
}

/// Position direction enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Buy direction
    Buy,
    /// Sell direction
    Sell,
}