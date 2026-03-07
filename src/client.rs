//! HTTP client implementation for Deribit REST API

use crate::auth::AuthManager;
use crate::config::HttpConfig;
use crate::error::HttpError;
use crate::model::response::api_response::ApiResponse;
use crate::model::types::AuthToken;
use crate::rate_limit::{RateLimiter, categorize_endpoint};
use crate::sync_compat::Mutex;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::sync::Arc;

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
        Self::with_config(config)
    }

    /// Create a new HTTP client with custom configuration
    pub fn with_config(config: HttpConfig) -> Self {
        let builder = Client::builder();

        #[cfg(not(target_arch = "wasm32"))]
        let builder = builder
            .timeout(config.timeout)
            .user_agent(&config.user_agent);

        let client = builder.build().expect("Failed to create HTTP client");

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
        let auth_header = {
            let mut auth_manager = self.auth_manager.lock().await;
            auth_manager
                .get_authorization_header()
                .await
                .ok_or_else(|| {
                    HttpError::AuthenticationFailed(
                        "No valid authentication token available.".to_string(),
                    )
                })?
        };

        // Debug: log the authorization header being used
        tracing::debug!("Using authorization header: {}", auth_header);

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
        let auth_header = {
            let mut auth_manager = self.auth_manager.lock().await;
            auth_manager
                .get_authorization_header()
                .await
                .ok_or_else(|| {
                    HttpError::AuthenticationFailed(
                        "No valid authentication token available.".to_string(),
                    )
                })?
        };

        // Debug: log the authorization header being used
        tracing::debug!("Using authorization header: {}", auth_header);

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

    /// Generic helper for public GET endpoints.
    ///
    /// Performs a rate-limited GET request to a public endpoint, parses the
    /// API response, and extracts the result. Handles all standard error cases:
    /// network errors, HTTP errors, API errors, and missing results.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint path (e.g., "/public/get_currencies")
    /// * `query` - Query string including leading "?" if non-empty, or empty string
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected result type, must implement `DeserializeOwned`
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails at any stage.
    pub async fn public_get<T>(&self, endpoint: &str, query: &str) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}{}", self.base_url(), endpoint, query);

        let response = self.make_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(error_text));
        }

        let api_response: ApiResponse<T> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
    }

    /// Generic helper for private GET endpoints.
    ///
    /// Performs a rate-limited, authenticated GET request to a private endpoint,
    /// parses the API response, and extracts the result. Handles all standard
    /// error cases: authentication errors, network errors, HTTP errors, API errors,
    /// and missing results.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint path (e.g., "/private/get_account_summary")
    /// * `query` - Query string including leading "?" if non-empty, or empty string
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected result type, must implement `DeserializeOwned`
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails at any stage.
    pub async fn private_get<T>(&self, endpoint: &str, query: &str) -> Result<T, HttpError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}{}", self.base_url(), endpoint, query);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(error_text));
        }

        let api_response: ApiResponse<T> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
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
