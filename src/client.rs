//! HTTP client implementation for Deribit REST API

use crate::config::{HttpConfig, validate_config};
use crate::error::HttpError;
use crate::rate_limit::{RateLimiter, categorize_endpoint};
use reqwest::Client;
use std::sync::Arc;

/// HTTP client for Deribit REST API
#[derive(Debug, Clone)]
pub struct DeribitHttpClient {
    /// HTTP client instance
    client: Client,
    /// Configuration
    config: Arc<HttpConfig>,
    /// Rate limiter
    rate_limiter: RateLimiter,
}

impl DeribitHttpClient {
    /// Create a new HTTP client
    pub fn new(test_net: bool) -> Self {
        let config = if test_net {
            HttpConfig::testnet()
        } else {
            HttpConfig::production()
        };

        Self::with_config(config).expect("Failed to create client with default config")
    }

    /// Create a new HTTP client with custom configuration
    pub fn with_config(config: HttpConfig) -> Result<Self, HttpError> {
        // Validate configuration
        validate_config(&config)?;

        // Build reqwest client
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        Ok(Self {
            client,
            config: Arc::new(config),
            rate_limiter: RateLimiter::new(),
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &HttpConfig {
        &self.config
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        self.config.base_url.as_str()
    }

    /// Get the HTTP client
    pub fn http_client(&self) -> &Client {
        &self.client
    }

    /// Make a rate-limited HTTP request
    pub async fn make_request(&self, url: &str) -> Result<reqwest::Response, HttpError> {
        // Determine rate limit category from URL
        let category = categorize_endpoint(url);

        // Wait for rate limit permission
        self.rate_limiter.wait_for_permission(category).await;

        // Make the request
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))
    }

    /// Get rate limiter for advanced usage
    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }
}
