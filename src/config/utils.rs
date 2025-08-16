//! Configuration utilities for HTTP client

use crate::config::HttpConfig;
use crate::error::HttpError;
use std::env;
use std::time::Duration;

/// Load configuration from environment variables
pub fn load_from_env() -> Result<HttpConfig, HttpError> {
    let mut config = if env::var("DERIBIT_TESTNET").unwrap_or_default() == "true" {
        HttpConfig::testnet()
    } else {
        HttpConfig::production()
    };

    // Set timeout from environment
    if let Ok(timeout_str) = env::var("DERIBIT_HTTP_TIMEOUT") {
        let timeout_secs: u64 = timeout_str
            .parse()
            .map_err(|_| HttpError::InvalidResponse("Invalid timeout value".to_string()))?;
        config = config.with_timeout(Duration::from_secs(timeout_secs));
    }

    // Set max retries from environment
    if let Ok(retries_str) = env::var("DERIBIT_HTTP_MAX_RETRIES") {
        let max_retries: u32 = retries_str
            .parse()
            .map_err(|_| HttpError::InvalidResponse("Invalid max retries value".to_string()))?;
        config = config.with_max_retries(max_retries);
    }

    // Set user agent from environment
    if let Ok(user_agent) = env::var("DERIBIT_HTTP_USER_AGENT") {
        config = config.with_user_agent(user_agent);
    }

    // Set OAuth2 credentials from environment
    if let (Ok(client_id), Ok(client_secret)) = (
        env::var("DERIBIT_CLIENT_ID"),
        env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        config = config.with_oauth2(client_id, client_secret);
    }
    // Alternative: API key credentials
    else if let (Ok(api_key), Ok(api_secret)) =
        (env::var("DERIBIT_API_KEY"), env::var("DERIBIT_API_SECRET"))
    {
        config = config.with_api_key(api_key, api_secret);
    }

    Ok(config)
}

/// Validate configuration
pub fn validate_config(config: &HttpConfig) -> Result<(), HttpError> {
    // Validate base URL
    if config.base_url.as_str().is_empty() {
        return Err(HttpError::InvalidResponse(
            "Base URL cannot be empty".to_string(),
        ));
    }

    // Validate timeout
    if config.timeout.as_secs() == 0 {
        return Err(HttpError::InvalidResponse(
            "Timeout must be greater than 0".to_string(),
        ));
    }

    // Validate credentials if present
    if let Some(credentials) = &config.credentials {
        if credentials.client_id.is_empty() && credentials.api_key.is_none() {
            return Err(HttpError::AuthenticationFailed(
                "Either client_id or api_key must be provided".to_string(),
            ));
        }

        if !credentials.client_id.is_empty() && credentials.client_secret.is_empty() {
            return Err(HttpError::AuthenticationFailed(
                "Client secret is required when using OAuth2".to_string(),
            ));
        }

        if let Some(api_key) = &credentials.api_key {
            if api_key.is_empty() {
                return Err(HttpError::AuthenticationFailed(
                    "API key cannot be empty".to_string(),
                ));
            }
            if credentials.api_secret.is_none() {
                return Err(HttpError::AuthenticationFailed(
                    "API secret is required when using API key authentication".to_string(),
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_validate_config_valid() {
        let config = HttpConfig::default();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_with_oauth2() {
        let config =
            HttpConfig::default().with_oauth2("client_id".to_string(), "client_secret".to_string());
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_with_api_key() {
        let config =
            HttpConfig::default().with_api_key("api_key".to_string(), "api_secret".to_string());
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_config_invalid_timeout() {
        let config = HttpConfig::default().with_timeout(Duration::from_secs(0));
        assert!(validate_config(&config).is_err());
    }
}
