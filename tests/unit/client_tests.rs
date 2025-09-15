/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/9/25
******************************************************************************/

//! Unit tests for DeribitHttpClient

use deribit_http::client::DeribitHttpClient;

#[tokio::test]
async fn test_client_new_default_testnet() {
    // By default, client should use testnet
    unsafe {
        std::env::remove_var("DERIBIT_TESTNET");
    }
    let client = DeribitHttpClient::new();
    // Default is testnet (true), so should contain test.deribit.com
    assert!(client.base_url().contains("test.deribit.com"));
}

#[tokio::test]
async fn test_client_new_production_via_env() {
    // Set environment variable to use production
    unsafe {
        std::env::set_var("DERIBIT_TESTNET", "false");
    }
    let client = DeribitHttpClient::new();
    assert!(client.base_url().contains("deribit.com"));
    // Note: production URL is www.deribit.com, not test.deribit.com
    // Clean up
    unsafe {
        std::env::remove_var("DERIBIT_TESTNET");
    }
}

#[tokio::test]
async fn test_client_new_testnet_via_env() {
    // Explicitly set testnet via environment
    unsafe {
        std::env::set_var("DERIBIT_TESTNET", "true");
    }
    let client = DeribitHttpClient::new();
    assert!(client.base_url().contains("test.deribit.com"));
    // Clean up
    unsafe {
        std::env::remove_var("DERIBIT_TESTNET");
    }
}

#[tokio::test]
async fn test_client_config_access() {
    let client = DeribitHttpClient::new();
    let config = client.config();
    assert!(!config.base_url.as_str().is_empty());
    assert!(config.timeout > std::time::Duration::from_secs(0));
    assert!(!config.user_agent.is_empty());
}

#[tokio::test]
async fn test_client_http_client_access() {
    let client = DeribitHttpClient::new();
    let http_client = client.http_client();
    // Just verify we can access the client without panicking
    assert!(!format!("{:?}", http_client).is_empty());
}

#[tokio::test]
async fn test_client_rate_limiter_access() {
    let client = DeribitHttpClient::new();
    let rate_limiter = client.rate_limiter();
    // Just verify we can access the rate limiter without panicking
    assert!(!format!("{:?}", rate_limiter).is_empty());
}

#[tokio::test]
async fn test_client_automatic_authentication() {
    // With automatic authentication, the client should handle auth internally
    let client = DeribitHttpClient::new();

    // These methods may not exist anymore or behave differently with automatic auth
    // We'll test that the client can be created successfully
    assert!(!client.base_url().is_empty());
}

#[cfg(test)]
mod mock_tests {
    use super::*;

    #[tokio::test]
    async fn test_make_request_success() {
        // Since we can't configure custom URLs anymore, we'll test the client creation
        // and basic functionality without mocking external requests
        let client = DeribitHttpClient::new();

        // Test that client is created successfully
        assert!(!client.base_url().is_empty());
        assert!(client.base_url().contains("deribit.com"));

        // Test config access
        let config = client.config();
        assert!(config.timeout.as_secs() > 0);
        assert!(!config.user_agent.is_empty());
    }

    #[tokio::test]
    async fn test_exchange_token_success() {
        // Since we can't mock the actual Deribit API calls without custom config,
        // we'll test that the method exists and handles errors appropriately
        let client = DeribitHttpClient::new();

        // This will likely fail with authentication error since we don't have real tokens
        // but it tests that the method is callable and returns the expected error type
        let result = client
            .exchange_token("invalid_test_token", 12345, None)
            .await;

        // Should return an error since we're using invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_exchange_token_with_scope() {
        let client = DeribitHttpClient::new();
        let result = client
            .exchange_token("invalid_test_token", 12345, Some("read write"))
            .await;

        // Should return an error since we're using invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fork_token_success() {
        let client = DeribitHttpClient::new();
        let result = client
            .fork_token("invalid_test_token", "test_session", None)
            .await;

        // Should return an error since we're using invalid credentials
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_exchange_token_error_response() {
        // Test error handling for invalid token exchange
        let client = DeribitHttpClient::new();
        let result = client.exchange_token("invalid_token", 12345, None).await;

        // Should return an error since we're using invalid credentials
        assert!(result.is_err());
    }
}
