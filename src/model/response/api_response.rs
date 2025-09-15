/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::types::ApiError;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// HTTP response structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    /// HTTP status code
    pub status: u16,
    /// Response headers as key-value pairs
    pub headers: HashMap<String, String>,
    /// Response body content
    pub body: String,
}

/// Generic API response wrapper
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
