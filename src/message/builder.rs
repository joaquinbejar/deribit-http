//! Message builder utilities for HTTP client

use crate::message::{HttpRequestBuilder, HttpResponseHandler};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Main message builder for HTTP operations
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HttpMessageBuilder {
    request_builder: HttpRequestBuilder,
    response_handler: HttpResponseHandler,
}

impl HttpMessageBuilder {
    /// Create a new message builder
    pub fn new(base_url: String) -> Self {
        Self {
            request_builder: HttpRequestBuilder::new(base_url),
            response_handler: HttpResponseHandler::new(),
        }
    }

    /// Get reference to request builder
    pub fn request_builder(&self) -> &HttpRequestBuilder {
        &self.request_builder
    }

    /// Get reference to response handler
    pub fn response_handler(&self) -> &HttpResponseHandler {
        &self.response_handler
    }
}
