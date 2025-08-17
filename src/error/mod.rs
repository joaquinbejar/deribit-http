//! Error handling module for HTTP client

/// HTTP client error types
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    /// Config error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Request failed with HTTP error
    #[error("Request failed: {0}")]
    RequestFailed(String),

    /// Authentication failed with the API
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// API rate limit has been exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Invalid response format received from API
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// Network connection error occurred
    #[error("Network error: {0}")]
    NetworkError(String),
}
