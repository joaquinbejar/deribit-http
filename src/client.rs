//! HTTP client implementation for Deribit REST API

use crate::auth::AuthManager;
use crate::config::HttpConfig;
use crate::error::HttpError;
use crate::model::types::AuthToken;
use crate::rate_limit::{RateLimiter, categorize_endpoint};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

/// HTTP client for Deribit REST API
#[derive(Debug, Clone)]
pub struct DeribitHttpClient {
    /// HTTP client instance
    client: Client,
    /// Configuration
    config: Arc<HttpConfig>,
    /// Rate limiter
    rate_limiter: RateLimiter,
    /// Authentication manager
    auth_manager: Arc<Mutex<AuthManager>>,
}

impl DeribitHttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        let config = HttpConfig::default();
        let opt_client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| HttpError::NetworkError(e.to_string()))
            .ok();

        let client = if let Some(opt_client) = &opt_client {
            opt_client.clone()
        } else {
            panic!("Failed to create HTTP client");
        };

        let auth_manager = AuthManager::new(client.clone(), config.clone());

        Self {
            client,
            config: Arc::new(config),
            rate_limiter: RateLimiter::new(),
            auth_manager: Arc::new(Mutex::new(auth_manager)),
        }
    }

    /// Get the configuration
    pub fn config(&self) -> &HttpConfig {
        &self.config
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        self.config.base_url.as_str()
    }

    /// Get the HTTP client
    pub fn http_client(&self) -> &Client {
        &self.client
    }

    /// Make a rate-limited HTTP request
    pub async fn make_request(&self, url: &str) -> Result<reqwest::Response, HttpError> {
        // Determine rate limit category from URL
        let category = categorize_endpoint(url);

        // Wait for rate limit permission
        self.rate_limiter.wait_for_permission(category).await;

        // Make the request
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))
    }

    /// Make an authenticated HTTP GET request for private endpoints
    pub async fn make_authenticated_request(
        &self,
        url: &str,
    ) -> Result<reqwest::Response, HttpError> {
        // Determine rate limit category from URL
        let category = categorize_endpoint(url);

        // Wait for rate limit permission
        self.rate_limiter.wait_for_permission(category).await;

        // Get authorization header
        let mut auth_manager = self.auth_manager.lock().await;
        let auth_header = auth_manager
            .get_authorization_header()
            .await
            .ok_or_else(|| {
                HttpError::AuthenticationFailed(
                    "No valid authentication token available.".to_string(),
                )
            })?;

        // Debug: log the authorization header being used
        tracing::debug!("Using authorization header: {}", auth_header);
        drop(auth_manager);

        // Make the authenticated request
        self.client
            .get(url)
            .header("Authorization", auth_header)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))
    }

    /// Make an authenticated HTTP POST request for private endpoints
    pub async fn make_authenticated_post_request<T: serde::Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<reqwest::Response, HttpError> {
        // Determine rate limit category from URL
        let category = categorize_endpoint(url);

        // Wait for rate limit permission
        self.rate_limiter.wait_for_permission(category).await;

        // Get authorization header
        let mut auth_manager = self.auth_manager.lock().await;
        let auth_header = auth_manager
            .get_authorization_header()
            .await
            .ok_or_else(|| {
                HttpError::AuthenticationFailed(
                    "No valid authentication token available.".to_string(),
                )
            })?;

        // Debug: log the authorization header being used
        tracing::debug!("Using authorization header: {}", auth_header);
        drop(auth_manager);

        // Make the authenticated POST request
        self.client
            .post(url)
            .header("Authorization", auth_header)
            .json(body)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))
    }

    /// Get rate limiter for advanced usage
    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }

    /// Exchange refresh token for a new access token with different subject_id
    pub async fn exchange_token(
        &self,
        refresh_token: &str,
        subject_id: u64,
        scope: Option<&str>,
    ) -> Result<AuthToken, HttpError> {
        let mut url = format!(
            "{}/public/exchange_token?refresh_token={}&subject_id={}",
            self.config.base_url,
            urlencoding::encode(refresh_token),
            subject_id
        );

        if let Some(scope) = scope {
            url.push_str(&format!("&scope={}", urlencoding::encode(scope)));
        }

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
                "Token exchange failed: {}",
                error_text
            )));
        }

        // Parse the JSON-RPC response directly
        let json_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        // Check for JSON-RPC error
        if let Some(_error) = json_response.get("error") {
            return Err(HttpError::AuthenticationFailed(format!(
                "Token exchange failed: {}",
                json_response
            )));
        }

        // Extract the result and parse as AuthToken
        let result = json_response
            .get("result")
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))?;

        let token: AuthToken = serde_json::from_value(result.clone())
            .map_err(|e| HttpError::InvalidResponse(format!("Failed to parse token: {}", e)))?;

        // Update the stored token
        let _auth_manager = self.auth_manager.lock().await;
        let _expires_at =
            std::time::SystemTime::now() + std::time::Duration::from_secs(token.expires_in);

        self.auth_manager.lock().await.update_token(token.clone());

        Ok(token)
    }

    /// Fork a token to create a new session with the same permissions
    pub async fn fork_token(
        &self,
        refresh_token: &str,
        session_name: &str,
        scope: Option<&str>,
    ) -> Result<AuthToken, HttpError> {
        let mut url = format!(
            "{}/public/fork_token?refresh_token={}&session_name={}",
            self.config.base_url,
            urlencoding::encode(refresh_token),
            urlencoding::encode(session_name)
        );

        if let Some(scope) = scope {
            url.push_str(&format!("&scope={}", urlencoding::encode(scope)));
        }

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
                "Token fork failed: {}",
                error_text
            )));
        }

        // Parse the JSON-RPC response directly
        let json_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        // Check for JSON-RPC error
        if let Some(_error) = json_response.get("error") {
            return Err(HttpError::AuthenticationFailed(format!(
                "Token fork failed: {}",
                json_response
            )));
        }

        // Extract the result and parse as AuthToken
        let result = json_response
            .get("result")
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))?;

        let token: AuthToken = serde_json::from_value(result.clone())
            .map_err(|e| HttpError::InvalidResponse(format!("Failed to parse token: {}", e)))?;

        self.auth_manager.lock().await.update_token(token.clone());

        Ok(token)
    }
}

impl Default for DeribitHttpClient {
    fn default() -> Self {
        Self::new()
    }
}
