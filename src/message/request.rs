//! HTTP request message handling

use crate::model::request::api::HttpRequest;
use crate::model::types::RequestParams;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request builder for HTTP messages
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HttpRequestBuilder {
    base_url: String,
}

impl HttpRequestBuilder {
    /// Create a new request builder
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Build a GET request
    pub fn build_get(&self, endpoint: &str, params: Option<RequestParams>) -> HttpRequest {
        let mut url = format!("{}{}", self.base_url, endpoint);

        if let Some(params) = params
            && !params.is_empty()
        {
            // Convert params to query string
            let query_string = self.params_to_query_string(&params);
            url.push('?');
            url.push_str(&query_string);
        }

        HttpRequest {
            method: "GET".to_string(),
            endpoint: url,
            headers: self.default_headers(),
            body: None,
        }
    }

    /// Build a POST request
    pub fn build_post(&self, endpoint: &str, params: Option<RequestParams>) -> HttpRequest {
        let url = format!("{}{}", self.base_url, endpoint);
        let body = params.map(|p| p.to_json().to_string());

        HttpRequest {
            method: "POST".to_string(),
            endpoint: url,
            headers: self.default_headers(),
            body,
        }
    }

    /// Build authentication request
    pub fn build_auth_request(&self, client_id: &str, client_secret: &str) -> HttpRequest {
        let params = RequestParams::new()
            .add("grant_type", "client_credentials")
            .add("client_id", client_id)
            .add("client_secret", client_secret);

        self.build_post("/public/auth", Some(params))
    }

    /// Build test request
    pub fn build_test_request(&self) -> HttpRequest {
        self.build_get("/public/test", None)
    }

    /// Build get time request
    pub fn build_get_time_request(&self) -> HttpRequest {
        self.build_get("/public/get_time", None)
    }

    /// Get default headers
    fn default_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers
    }

    /// Convert parameters to query string
    fn params_to_query_string(&self, _params: &RequestParams) -> String {
        // TODO: Implement proper query string conversion
        String::new()
    }
}
