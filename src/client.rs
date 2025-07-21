//! HTTP client implementation for Deribit REST API

/// HTTP client for Deribit REST API
#[derive(Debug)]
pub struct DeribitHttpClient {
    /// Base URL for API requests
    pub base_url: String,
}

impl DeribitHttpClient {
    /// Create a new HTTP client
    pub fn new(test_net: bool) -> Self {
        let base_url = if test_net {
            "https://test.deribit.com/api/v2".to_string()
        } else {
            "https://www.deribit.com/api/v2".to_string()
        };

        Self { base_url }
    }
}
