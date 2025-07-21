//! HTTP client implementation for Deribit REST API

use async_trait::async_trait;
use deribit_base::{DeribitClient, DeribitConfig, DeribitError, DeribitUrls};
use reqwest::Client;
use std::time::Duration;

/// HTTP client for Deribit REST API
#[allow(dead_code)]
pub struct DeribitHttpClient {
    config: DeribitConfig,
    client: Client,
    base_url: String,
    access_token: Option<String>,
    connected: bool,
}

impl DeribitHttpClient {
    /// Create a new HTTP client
    pub fn new(config: DeribitConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        let base_url = format!("{}/api/v2", DeribitUrls::get_base_url(config.test_net));

        Self {
            config,
            client,
            base_url,
            access_token: None,
            connected: false,
        }
    }

    /// Get the access token (if authenticated)
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    /// Set the access token
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }
}

#[async_trait]
impl DeribitClient for DeribitHttpClient {
    type Error = DeribitError;

    async fn connect(&mut self) -> Result<(), Self::Error> {
        // For HTTP client, "connecting" means authenticating
        // This is a placeholder - actual authentication will be implemented
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), Self::Error> {
        self.access_token = None;
        self.connected = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}
