//! HTTP-specific types and models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP request structure
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// HTTP method (GET, POST, PUT, DELETE, etc.)
    pub method: String,
    /// API endpoint path
    pub endpoint: String,
    /// HTTP headers as key-value pairs
    pub headers: HashMap<String, String>,
    /// Optional request body content
    pub body: Option<String>,
}

/// HTTP response structure
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// HTTP status code
    pub status: u16,
    /// Response headers as key-value pairs
    pub headers: HashMap<String, String>,
    /// Response body content
    pub body: String,
}

/// Generic API response wrapper
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Successful result data
    pub result: Option<T>,
    /// Error information if request failed
    pub error: Option<ApiError>,
    /// Request ID for tracking
    pub id: Option<u64>,
    /// Server processing start time in microseconds
    #[serde(rename = "usIn")]
    pub us_in: Option<u64>,
    /// JSON-RPC version (typically "2.0")
    pub jsonrpc: Option<String>,
    /// Server processing end time in microseconds
    #[serde(rename = "usOut")]
    pub us_out: Option<u64>,
    /// Processing time difference in microseconds
    #[serde(rename = "usDiff")]
    pub us_diff: Option<u64>,
    /// Whether this is a testnet response
    pub testnet: Option<bool>,
}

/// API error structure
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code number
    pub code: i32,
    /// Human-readable error message
    pub message: String,
    /// Additional error data
    pub data: Option<serde_json::Value>,
}

/// Authentication token structure
#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Clone, Default, Serialize, Deserialize)]
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

// Implement Display and Debug traits using macros from deribit-base
deribit_base::impl_json_display!(ApiError);
deribit_base::impl_json_debug_pretty!(ApiError);

deribit_base::impl_json_display!(AuthToken);
deribit_base::impl_json_debug_pretty!(AuthToken);

deribit_base::impl_json_display!(RequestParams);
deribit_base::impl_json_debug_pretty!(RequestParams);

// ApiResponse<T> is generic, so we cannot use the macros directly
// HttpRequest and HttpResponse contain HashMap which is not serializable by default
// so we keep the derived Debug trait for those
