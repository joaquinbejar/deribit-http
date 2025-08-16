//! HTTP session management

use crate::config::HttpConfig;
use crate::model::http_types::AuthToken;
use std::sync::Arc;
use tokio::sync::Mutex;

/// HTTP session manager
#[derive(Debug, Clone)]
pub struct HttpSession {
    config: Arc<HttpConfig>,
    auth_token: Arc<Mutex<Option<AuthToken>>>,
}

impl HttpSession {
    /// Create a new HTTP session
    pub fn new(config: HttpConfig) -> Self {
        Self {
            config: Arc::new(config),
            auth_token: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the configuration
    pub fn config(&self) -> &HttpConfig {
        &self.config
    }

    /// Set authentication token
    pub async fn set_auth_token(&self, token: AuthToken) {
        *self.auth_token.lock().await = Some(token);
    }

    /// Get authentication token
    pub async fn auth_token(&self) -> Option<AuthToken> {
        self.auth_token.lock().await.clone()
    }

    /// Check if session is authenticated
    pub async fn is_authenticated(&self) -> bool {
        self.auth_token.lock().await.is_some()
    }

    /// Clear authentication token
    pub async fn clear_auth_token(&self) {
        *self.auth_token.lock().await = None;
    }

    /// Check if token is expired
    pub async fn is_token_expired(&self) -> bool {
        // TODO: Implement token expiration check
        // This would require storing the token creation time
        // and comparing with expires_in value
        false
    }

    /// Get authorization header value
    pub async fn authorization_header(&self) -> Option<String> {
        if let Some(token) = self.auth_token().await {
            Some(format!("{} {}", token.token_type, token.access_token))
        } else {
            None
        }
    }
}
