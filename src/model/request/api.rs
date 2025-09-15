/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use std::collections::HashMap;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// HTTP request structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
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
