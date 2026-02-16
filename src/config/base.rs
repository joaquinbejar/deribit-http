//! Base configuration for HTTP client

use crate::config::credentials::ApiCredentials;
use crate::constants::{DEFAULT_TIMEOUT, MAX_RETRIES, PRODUCTION_BASE_URL, TESTNET_BASE_URL};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(not(target_arch = "wasm32"))]
    fn default() -> Self {
        let testnet = env::var("DERIBIT_TESTNET")
            .map(|val| val.to_lowercase() == "true")
            .unwrap_or(true); // Default to testnet for safety

        let base_url = if testnet {
            Url::parse(TESTNET_BASE_URL).expect("Invalid testnet URL")
        } else {
            Url::parse(PRODUCTION_BASE_URL).expect("Invalid base URL")
        };

        Self::from_env(base_url, testnet)
    }

    #[cfg(target_arch = "wasm32")]
    fn default() -> Self {
        Self::testnet()
    }
}

impl HttpConfig {
    /// Read shared configuration from environment variables.
    #[cfg(not(target_arch = "wasm32"))]
    fn from_env(base_url: Url, testnet: bool) -> Self {
        dotenv::dotenv().ok();
        let credentials = ApiCredentials::new().ok();

        let max_retries = env::var("DERIBIT_HTTP_MAX_RETRIES")
            .map(|val| val.parse::<u32>().unwrap_or(MAX_RETRIES))
            .unwrap_or(MAX_RETRIES);

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

    /// Create testnet configuration
    pub fn testnet() -> Self {
        Self::create(
            Url::parse(TESTNET_BASE_URL).expect("Invalid testnet URL"),
            true,
        )
    }

    /// Create production configuration
    pub fn production() -> Self {
        Self::create(
            Url::parse(PRODUCTION_BASE_URL).expect("Invalid production URL"),
            false,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create(base_url: Url, testnet: bool) -> Self {
        Self::from_env(base_url, testnet)
    }

    #[cfg(target_arch = "wasm32")]
    fn create(base_url: Url, testnet: bool) -> Self {
        Self {
            base_url,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
            max_retries: MAX_RETRIES,
            user_agent: format!("deribit-http/{}", env!("CARGO_PKG_VERSION")),
            testnet,
            credentials: None,
        }
    }

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
