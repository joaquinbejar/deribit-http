//! Authentication module for Deribit HTTP API
//!
//! This module provides OAuth2 and API key authentication mechanisms
//! for the Deribit REST API. It handles token management, refresh,
//! and secure credential storage.

use crate::config::HttpConfig;
use crate::error::HttpError;
use crate::model::http_types::AuthToken;
use base64::Engine;
use hmac::{Hmac, Mac};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, error};
use urlencoding;

type HmacSha256 = Hmac<Sha256>;

/// OAuth2 authentication request
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Grant type (always "client_credentials" for Deribit)
    pub grant_type: String,
    /// Client ID from Deribit account
    pub client_id: String,
    /// Client secret from Deribit account
    pub client_secret: String,
    /// Optional scope for the token
    pub scope: Option<String>,
}

/// API key authentication parameters
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ApiKeyAuth {
    /// API key
    pub key: String,
    /// API secret
    pub secret: String,
    /// Timestamp for the request
    pub timestamp: u64,
    /// Nonce for the request
    pub nonce: String,
}

/// Authentication manager for HTTP client
#[derive(Debug, Clone)]
pub struct AuthManager {
    client: Client,
    config: HttpConfig,
    token: Option<AuthToken>,
    token_expires_at: Option<SystemTime>,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(client: Client, config: HttpConfig) -> Self {
        Self {
            client,
            config,
            token: None,
            token_expires_at: None,
        }
    }

    /// Authenticate using OAuth2 client credentials
    pub async fn authenticate_oauth2(&mut self) -> Result<AuthToken, HttpError> {
        let credentials = match self.config.credentials.clone() {
            Some(creds) => match creds.is_valid() {
                true => creds,
                false => {
                    return Err(HttpError::AuthenticationFailed(
                        "Invalid credentials for OAuth2".to_string(),
                    ));
                }
            },
            None => {
                return Err(HttpError::AuthenticationFailed(
                    "No credentials configured".to_string(),
                ));
            }
        };
        let (client_id, client_secret) = credentials.get_client_credentials()?;
        // Build URL with query parameters as per Deribit API documentation
        let url = format!(
            "{}/public/auth?grant_type=client_credentials&client_id={}&client_secret={}",
            self.config.base_url,
            urlencoding::encode(client_id.as_str()),
            urlencoding::encode(client_secret.as_str())
        );

        // Debug: log the URL being used
        debug!("Authentication URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::AuthenticationFailed(format!(
                "OAuth2 authentication failed: {}",
                error_text
            )));
        }

        // Parse the JSON-RPC response directly
        let json_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        // Check for JSON-RPC error
        if let Some(error) = json_response.get("error") {
            let _code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(-1);
            let _message = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            return Err(HttpError::AuthenticationFailed(format!(
                "OAuth2 authentication failed: {}",
                json_response
            )));
        }

        // Extract the result and parse as AuthToken
        let result = json_response
            .get("result")
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))?;

        let token: AuthToken = serde_json::from_value(result.clone())
            .map_err(|e| HttpError::InvalidResponse(format!("Failed to parse token: {}", e)))?;

        // Calculate token expiration time
        let expires_at = SystemTime::now() + Duration::from_secs(token.expires_in);

        self.token = Some(token.clone());
        self.token_expires_at = Some(expires_at);

        Ok(token)
    }

    /// Generate API key signature for request
    pub fn generate_api_key_signature(
        &self,
        api_secret: &str,
        timestamp: u64,
        nonce: &str,
        method: &str,
        uri: &str,
        body: &str,
    ) -> Result<String, HttpError> {
        let data = format!(
            "{}{}{}{}{}",
            timestamp,
            nonce,
            method.to_uppercase(),
            uri,
            body
        );

        let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())
            .map_err(|e| HttpError::AuthenticationFailed(format!("Invalid API secret: {}", e)))?;

        mac.update(data.as_bytes());
        let result = mac.finalize();

        Ok(base64::engine::general_purpose::STANDARD.encode(result.into_bytes()))
    }

    /// Get current authentication token
    pub fn get_token(&self) -> Option<&AuthToken> {
        if self.is_token_expired() {
            self.token.as_ref()
        } else {
            None
        }
    }

    /// Check if token is expired or about to expire
    fn is_token_expired(&self) -> bool {
        match self.token_expires_at {
            Some(expires_at) => {
                // Consider token expired if it expires within the next 60 seconds
                let buffer = Duration::from_secs(60);
                SystemTime::now() + buffer >= expires_at
            }
            None => true,
        }
    }

    ///
    /// Checks whether the token is valid.
    ///
    /// The function determines the validity of a token based on two conditions:
    /// 1. The token must exist (i.e., it is `Some`).
    /// 2. The token must not be expired, as determined by the `is_token_expired` function.
    ///
    /// # Returns
    /// * `true` - if the token exists and is not expired.
    /// * `false` - if the token does not exist or is expired.
    ///
    fn is_token_valid(&self) -> bool {
        match self.token {
            Some(_) => !self.is_token_expired(),
            None => false,
        }
    }

    /// Get authorization header value
    pub async fn get_authorization_header(&mut self) -> Option<String> {
        match self.is_token_valid() {
            true => {
                let token = self.token.as_ref().unwrap();
                Some(format!("{} {}", token.token_type, token.access_token))
            }
            false => match self.config.credentials.as_ref() {
                Some(credentials) => match credentials.is_valid() {
                    true => match self.authenticate_oauth2().await {
                        Ok(token) => Some(format!("{} {}", token.token_type, token.access_token)),
                        Err(e) => {
                            error!("Failed to authenticate: {}", e);
                            None
                        }
                    },
                    false => None,
                },
                None => None,
            },
        }
    }

    /// Generate nonce for API key authentication
    pub fn generate_nonce() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let chars: String = (0..16)
            .map(|_| {
                let idx = rng.random_range(0..62);
                match idx {
                    0..=25 => (b'a' + idx) as char,
                    26..=51 => (b'A' + (idx - 26)) as char,
                    _ => (b'0' + (idx - 52)) as char,
                }
            })
            .collect();
        chars
    }

    /// Get current timestamp in milliseconds
    pub fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Updates the authentication token and its expiration time for the current instance.
    ///
    /// This method updates the internal state of the object by setting a new authentication token
    /// and calculating its expiration time based on the current system time and the token's
    /// `expires_in` duration.
    ///
    /// # Arguments
    ///
    /// * `token` - An instance of `AuthToken` containing the new authentication token and its
    ///   metadata (e.g., expiration duration).
    ///
    /// # Effects
    ///
    /// * The `self.token` field is set to the provided `token`.
    /// * The `self.token_expires_at` field is set to the current system time plus the `expires_in`
    ///   duration from the provided `token`.
    ///
    /// # Note
    ///
    /// Ensure that the provided `AuthToken` is valid and its `expires_in` duration is correctly
    /// defined in seconds, as it will determine the computed expiration time.
    ///
    /// # Panics
    ///
    /// This function does not explicitly panic, but unexpected behavior could occur if the
    /// system time manipulation or `Duration` calculations fail (e.g., overflow).
    pub fn update_token(&mut self, token: AuthToken) {
        self.token_expires_at = Some(SystemTime::now() + Duration::from_secs(token.expires_in));
        self.token = Some(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_request_creation() {
        let auth_request = AuthRequest {
            grant_type: "client_credentials".to_string(),
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            scope: Some("read write".to_string()),
        };

        assert_eq!(auth_request.grant_type, "client_credentials");
        assert_eq!(auth_request.client_id, "test_client");
    }

    #[test]
    fn test_nonce_generation() {
        let nonce1 = AuthManager::generate_nonce();
        let nonce2 = AuthManager::generate_nonce();

        assert_eq!(nonce1.len(), 16);
        assert_eq!(nonce2.len(), 16);
        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_timestamp_generation() {
        let timestamp1 = AuthManager::get_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let timestamp2 = AuthManager::get_timestamp();

        assert!(timestamp2 > timestamp1);
    }
}
