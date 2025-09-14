use std::env;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use tracing::warn;
use crate::HttpError;

/// API credentials for authentication
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ApiCredentials {
    /// Client ID for OAuth2
    pub client_id: Option<String>,
    /// Client secret for OAuth2
    pub client_secret: Option<String>,
}

impl ApiCredentials {
    pub fn is_valid(&self) -> bool {
        self.client_id.is_some() && self.client_secret.is_some()
    }
    
    pub fn new() -> Result<Self, HttpError> {
        let creds = Self::default();
        if creds.is_valid() {
            Ok(creds)
        } else {
            warn!("API credentials are provided in environment variables, only public endpoints will be available.");
            Err(HttpError::ConfigError("API credentials are not properly set in environment variables".into()))
        }
    }
    
    pub fn get_client_credentials(&self) -> Result<(String, String), HttpError> {
        if self.client_id.is_some() && self.client_secret.is_some() {
            Ok((self.client_id.clone().unwrap(), self.client_secret.clone().unwrap()))
        } else {
            Err(HttpError::ConfigError("Client ID and Client Secret must be set for OAuth2 authentication".into()))
        }
    }
}

impl Default for ApiCredentials {
    fn default() -> Self {
        dotenv::dotenv().ok();
        let client_id = env::var("DERIBIT_CLIENT_ID").ok();
        let client_secret = env::var("DERIBIT_CLIENT_SECRET").ok();
        Self {
            client_id,
            client_secret,

        }
    }
}