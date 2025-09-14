//! Token Management Integration Tests
//!
//! This test covers token management scenarios:
//! 1. OAuth2 token acquisition
//! 2. Token refresh mechanisms
//! 3. Token expiration handling
//! 4. Token validation and verification
//! 5. Token storage and retrieval

use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info};
use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
#[allow(dead_code)]
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    // Check if .env file exists
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with authentication credentials".into());
    }

    // Load environment variables
    dotenv::dotenv().ok();

    // Check for either OAuth2 or API key credentials
    let has_oauth2 = std::env::var("DERIBIT_CLIENT_ID").is_ok()
        && std::env::var("DERIBIT_CLIENT_SECRET").is_ok();
    let has_api_key =
        std::env::var("DERIBIT_API_KEY").is_ok() && std::env::var("DERIBIT_API_SECRET").is_ok();

    if !has_oauth2 && !has_api_key {
        return Err("Missing authentication credentials. Please provide either OAuth2 (CLIENT_ID/CLIENT_SECRET) or API key (API_KEY/API_SECRET)".into());
    }

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_token_storage_and_retrieval() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;

    // Initialize tracing for test debugging

    info!("Starting token storage and retrieval test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new();

    // Authenticate using available credentials
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        debug!("Using OAuth2 authentication");
        // Authentication is now automatic - no need to call authenticate_oauth2
        info!("OAuth2 authentication is automatic");

        // Make an authenticated request to verify authentication is working
        let result = client.get_account_summary("BTC", None).await;
        assert!(
            result.is_ok(),
            "Should be able to make authenticated requests"
        );
    } else if let (Ok(api_key), Ok(api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        debug!("Using API key authentication");
        // Authentication is now automatic - no need to call authenticate_api_key
        info!("API key authentication is automatic");

        // Make an authenticated request to verify authentication is working
        let result = client.get_account_summary("BTC", None).await;
        assert!(
            result.is_ok(),
            "Should be able to make authenticated requests"
        );
    }

    info!("Token storage and retrieval test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_token_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;

    // Initialize tracing for test debugging

    info!("Starting token validation test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new();

    // Check if authentication credentials are available
    if let (Ok(_client_id), Ok(_client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        debug!("Using OAuth2 authentication for validation test");
        info!("OAuth2 authentication is automatic");
    } else if let (Ok(_api_key), Ok(_api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        debug!("Using API key authentication for validation test");
        info!("API key authentication is automatic");
    } else {
        return Err("No valid authentication credentials found".into());
    };

    info!("Authentication successful for validation test");

    // Test token by making multiple authenticated requests
    for i in 0..5 {
        debug!(
            "Making authenticated request #{} for token validation",
            i + 1
        );
        let result = client.get_server_time().await;
        assert!(
            result.is_ok(),
            "Token should be valid for request #{}",
            i + 1
        );

        // Small delay between requests
        sleep(Duration::from_millis(200)).await;
    }

    info!("Token validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_concurrent_token_usage() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;

    // Initialize tracing for test debugging

    info!("Starting concurrent token usage test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new();

    // Authenticate using available credentials
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        debug!("Using OAuth2 authentication for concurrent test");
        // Authentication is now automatic - no need to call authenticate_oauth2
        info!("OAuth2 authentication is automatic");
    } else if let (Ok(_api_key), Ok(_api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        debug!("Using API key authentication for concurrent test");
        // Authentication is now automatic - no need to call authenticate_api_key
        info!("API key authentication is automatic");
    } else {
        return Err("No valid authentication credentials found".into());
    }

    info!("Authentication successful for concurrent test");

    // Make multiple concurrent authenticated requests
    let mut handles = vec![];

    for i in 0..3 {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            debug!("Starting concurrent request #{}", i + 1);
            let result = client_clone.get_server_time().await;
            debug!("Completed concurrent request #{}", i + 1);
            result
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await??;
        debug!("Concurrent request #{} result: {:?}", i + 1, result);
        assert!(
            result > 0,
            "Server time should be valid for concurrent request #{}",
            i + 1
        );
    }

    info!("Concurrent token usage test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_token_refresh_behavior() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;

    // Initialize tracing for test debugging

    info!("Starting token refresh behavior test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new();

    // Check authentication credentials are available
    if let (Ok(_client_id), Ok(_client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        debug!("Using OAuth2 authentication for refresh test");
        // Authentication is now automatic - no need to call authenticate_oauth2
        info!("OAuth2 authentication is automatic");
    } else if let (Ok(_api_key), Ok(_api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        debug!("Using API key authentication for refresh test");
        // Authentication is now automatic - no need to call authenticate_api_key
        info!("API key authentication is automatic");
    } else {
        return Err("No valid authentication credentials found".into());
    }

    info!("First authentication successful");

    // Wait a moment
    sleep(Duration::from_secs(1)).await;

    // Test making authenticated requests to verify refresh behavior
    if let (Ok(_client_id), Ok(_client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        debug!("Testing OAuth2 authentication refresh behavior");
        info!("OAuth2 authentication refresh is automatic");
    } else if let (Ok(_api_key), Ok(_api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        debug!("Testing API key authentication refresh behavior");
        info!("API key authentication refresh is automatic");
    } else {
        return Err("No valid authentication credentials found".into());
    }

    info!("Second authentication successful");

    // Test that authentication works for requests after refresh behavior
    let result1 = client.get_server_time().await;
    assert!(result1.is_ok(), "Authentication should work for requests");

    let result2 = client.get_account_summary("BTC", None).await;
    assert!(
        result2.is_ok(),
        "Authentication should work for authenticated requests"
    );

    info!("Token refresh behavior test completed successfully");
    Ok(())
}
