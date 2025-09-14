use crate::HttpError;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::warn;

/// API credentials for authentication
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ApiCredentials {
    /// Client ID for OAuth2
    pub client_id: Option<String>,
    /// Client secret for OAuth2
    pub client_secret: Option<String>,
}

impl ApiCredentials {
    /// Validates whether the required fields `client_id` and `client_secret` are present.
    ///
    /// # Returns
    ///
    /// * `true` - If both `client_id` and `client_secret` are `Some` (i.e., not `None`).
    /// * `false` - If either `client_id` or `client_secret` is `None`.
    ///
    pub fn is_valid(&self) -> bool {
        self.client_id.is_some() && self.client_secret.is_some()
    }

    /// Creates a new instance of the struct with credentials initialized from environment variables.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the credentials are properly configured and valid.
    /// - `Err(HttpError::ConfigError)`: If the credentials are not properly set or invalid, with an appropriate
    ///   error message indicating the configuration issue.
    ///
    /// # Note
    /// - If the credentials are invalid or not set, a warning will be logged, and only public API endpoints
    ///   will be accessible.
    ///
    pub fn new() -> Result<Self, HttpError> {
        let creds = Self::default();
        if creds.is_valid() {
            Ok(creds)
        } else {
            warn!(
                "API credentials are provided in environment variables, only public endpoints will be available."
            );
            Err(HttpError::ConfigError(
                "API credentials are not properly set in environment variables".into(),
            ))
        }
    }

    /// Retrieves the client credentials (Client ID and Client Secret) required for OAuth2 authentication.
    ///
    /// # Returns
    /// - `Ok((String, String))`: A tuple containing the `client_id` and `client_secret` if they are both set.
    /// - `Err(HttpError)`: An error of type `HttpError::ConfigError` if either `client_id` or `client_secret` is not set.
    ///
    /// # Errors
    /// Returns an `HttpError::ConfigError` with a message indicating that both `Client ID`
    /// and `Client Secret` must be set for OAuth2 authentication when either or both are absent.
    ///
    /// # Note
    /// This function assumes that `client_id` and `client_secret` are optional fields.
    /// Ensure they are properly configured before invoking this function.
    pub fn get_client_credentials(&self) -> Result<(String, String), HttpError> {
        if self.client_id.is_some() && self.client_secret.is_some() {
            Ok((
                self.client_id.clone().unwrap(),
                self.client_secret.clone().unwrap(),
            ))
        } else {
            Err(HttpError::ConfigError(
                "Client ID and Client Secret must be set for OAuth2 authentication".into(),
            ))
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
