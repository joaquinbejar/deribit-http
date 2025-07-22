//! Authentication module for HTTP client

use serde::{Deserialize, Serialize};

/// Authentication request
#[derive(Debug, Serialize)]
pub struct AuthRequest {
    /// OAuth2 grant type (typically "client_credentials")
    pub grant_type: String,
    /// Client ID from Deribit API credentials
    pub client_id: String,
    /// Client secret from Deribit API credentials
    pub client_secret: String,
    /// Optional scope for the authentication request
    pub scope: Option<String>,
}

/// Authentication response
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    /// Access token for authenticated API requests
    pub access_token: String,
    /// Token expiration time in seconds
    pub expires_in: i64,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// Granted scope for the access token
    pub scope: String,
    /// Type of the token (typically "Bearer")
    pub token_type: String,
}

impl AuthRequest {
    /// Create a new authentication request
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            grant_type: "client_credentials".to_string(),
            client_id,
            client_secret,
            scope: None,
        }
    }

    /// Set the scope for the authentication request
    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }
}
