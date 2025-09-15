//! HTTP response message handling

use crate::error::HttpError;
use crate::model::response::api::{ApiResponse, HttpResponse};
use crate::model::types::ApiError;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Response handler for HTTP messages
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HttpResponseHandler;

impl HttpResponseHandler {
    /// Create a new response handler
    pub fn new() -> Self {
        Self
    }

    /// Parse HTTP response
    pub fn parse_response<T>(&self, response: &HttpResponse) -> Result<ApiResponse<T>, HttpError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if response.status >= 400 {
            return Err(HttpError::RequestFailed(format!(
                "HTTP {} - {}",
                response.status, response.body
            )));
        }

        serde_json::from_str(&response.body).map_err(|e| HttpError::InvalidResponse(e.to_string()))
    }

    /// Check if response is successful
    pub fn is_success(&self, response: &HttpResponse) -> bool {
        response.status >= 200 && response.status < 300
    }

    /// Extract error from API response
    pub fn extract_error<'a, T>(&self, api_response: &'a ApiResponse<T>) -> Option<&'a ApiError> {
        api_response.error.as_ref()
    }

    /// Extract result from API response
    pub fn extract_result<'a, T>(&self, api_response: &'a ApiResponse<T>) -> Option<&'a T> {
        api_response.result.as_ref()
    }

    /// Handle rate limiting
    pub fn handle_rate_limit(&self, response: &HttpResponse) -> Result<(), HttpError> {
        if response.status == 429 {
            return Err(HttpError::RateLimitExceeded);
        }
        Ok(())
    }

    /// Handle authentication errors
    pub fn handle_auth_error(&self, response: &HttpResponse) -> Result<(), HttpError> {
        if response.status == 401 || response.status == 403 {
            return Err(HttpError::AuthenticationFailed(
                "Authentication failed or expired".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for HttpResponseHandler {
    fn default() -> Self {
        Self::new()
    }
}
