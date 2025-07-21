//! Authentication module for HTTP client

use serde::{Deserialize, Serialize};

/// Authentication request
#[derive(Debug, Serialize)]
pub struct AuthRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: Option<String>,
}

/// Authentication response
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
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
