//! Base configuration for HTTP client

use crate::config::credentials::ApiCredentials;
use crate::constants::{DEFAULT_TIMEOUT, MAX_RETRIES, PRODUCTION_BASE_URL, TESTNET_BASE_URL};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use url::Url;

/// Configuration for the HTTP client
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Base URL for API requests
    pub base_url: Url,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum number of retries
    pub max_retries: u32,
    /// User agent string
    pub user_agent: String,
    /// Whether to use testnet
    pub testnet: bool,
    /// API credentials
    pub credentials: Option<ApiCredentials>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        dotenv::dotenv().ok();
        // Credentials
        let credentials = ApiCredentials::new().ok();

        // Testnet flag
        let testnet = env::var("DERIBIT_TESTNET")
            .map(|val| val.to_lowercase() == "true")
            .unwrap_or(true); // Default to testnet for safety

        // Base URL
        let base_url = if testnet {
            Url::parse(TESTNET_BASE_URL).expect("Invalid testnet URL")
        } else {
            Url::parse(PRODUCTION_BASE_URL).expect("Invalid base URL")
        };

        // Maximum number of retries
        let max_retries = env::var("DERIBIT_HTTP_MAX_RETRIES")
            .map(|val| val.parse::<u32>().unwrap_or(MAX_RETRIES))
            .unwrap_or(MAX_RETRIES);

        // Timeout in seconds
        let timeout_u64 = env::var("DERIBIT_HTTP_TIMEOUT")
            .map(|val| val.parse::<u64>().unwrap_or(DEFAULT_TIMEOUT))
            .unwrap_or(DEFAULT_TIMEOUT);
        let timeout = Duration::from_secs(timeout_u64);

        let user_agent = env::var("DERIBIT_HTTP_USER_AGENT")
            .unwrap_or_else(|_| format!("deribit-http/{}", env!("CARGO_PKG_VERSION")));

        Self {
            base_url,
            timeout,
            max_retries,
            user_agent,
            testnet,
            credentials,
        }
    }
}

impl HttpConfig {
    /// Set the timeout for requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the maximum number of retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set the user agent string
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Set OAuth2 credentials
    pub fn with_oauth2(mut self, client_id: String, client_secret: String) -> Self {
        self.credentials = Some(ApiCredentials {
            client_id: Some(client_id),
            client_secret: Some(client_secret),
        });
        self
    }

    /// Check if credentials are configured
    pub fn has_credentials(&self) -> bool {
        self.credentials.is_some()
    }

    /// Get the credentials
    pub fn credentials(&self) -> Option<&ApiCredentials> {
        self.credentials.as_ref()
    }
}
