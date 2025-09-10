//! Base configuration for HTTP client

use crate::constants::{DEFAULT_TIMEOUT, MAX_RETRIES, PRODUCTION_BASE_URL, TESTNET_BASE_URL};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
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

/// API credentials for authentication
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ApiCredentials {
    /// Client ID for OAuth2
    pub client_id: String,
    /// Client secret for OAuth2
    pub client_secret: String,
    /// API key (alternative to OAuth2)
    pub api_key: Option<String>,
    /// API secret (for API key authentication)
    pub api_secret: Option<String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse(PRODUCTION_BASE_URL).expect("Invalid base URL"),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
            max_retries: MAX_RETRIES,
            user_agent: format!("deribit-http/{}", env!("CARGO_PKG_VERSION")),
            testnet: false,
            credentials: None,
        }
    }
}

impl HttpConfig {
    /// Create a new configuration for testnet
    pub fn testnet() -> Self {
        Self {
            base_url: Url::parse(TESTNET_BASE_URL).expect("Invalid testnet URL"),
            testnet: true,
            ..Default::default()
        }
    }

    /// Create a new configuration for production
    pub fn production() -> Self {
        Self::default()
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
            client_id,
            client_secret,
            api_key: None,
            api_secret: None,
        });
        self
    }

    /// Set API key credentials
    pub fn with_api_key(mut self, api_key: String, api_secret: String) -> Self {
        self.credentials = Some(ApiCredentials {
            client_id: String::new(),
            client_secret: String::new(),
            api_key: Some(api_key),
            api_secret: Some(api_secret),
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

// HttpConfig cannot use JSON macros because it contains non-serializable fields (Url)
// so we keep the derived Debug trait
