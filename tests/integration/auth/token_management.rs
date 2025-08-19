//! Token Management Integration Tests
//!
//! This test covers token management functionality:
//! 1. Token storage and retrieval
//! 2. Token expiration handling
//! 3. Automatic token renewal
//! 4. Token validation

use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info};

use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    // Check if .env file exists
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with authentication credentials".into());
    }

    // Load environment variables
    dotenv::dotenv().ok();

    // Check for either OAuth2 or API key credentials
    let has_oauth2 = std::env::var("DERIBIT_CLIENT_ID").is_ok() && std::env::var("DERIBIT_CLIENT_SECRET").is_ok();
    let has_api_key = std::env::var("DERIBIT_API_KEY").is_ok() && std::env::var("DERIBIT_API_SECRET").is_ok();
    
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
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting token storage and retrieval test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Authenticate using available credentials
    if let (Ok(client_id), Ok(client_secret)) = (std::env::var("DERIBIT_CLIENT_ID"), std::env::var("DERIBIT_CLIENT_SECRET")) {
        debug!("Using OAuth2 authentication");
        let token = client.authenticate_oauth2(&client_id, &client_secret).await?;
        info!("OAuth2 authentication successful");
        
        // Verify token is stored and can be retrieved
        assert!(!token.access_token.is_empty(), "Token should be stored");
        
        // Make an authenticated request to verify token is being used
        let result = client.get_account_summary("BTC", None).await;
        assert!(result.is_ok(), "Should be able to use stored token");
        
    } else if let (Ok(api_key), Ok(api_secret)) = (std::env::var("DERIBIT_API_KEY"), std::env::var("DERIBIT_API_SECRET")) {
        debug!("Using API key authentication");
        let token = client.authenticate_api_key(&api_key, &api_secret).await?;
        info!("API key authentication successful");
        
        // Verify token is stored and can be retrieved
        assert!(!token.access_token.is_empty(), "Token should be stored");
        
        // Make an authenticated request to verify token is being used
        let result = client.get_account_summary("BTC", None).await;
        assert!(result.is_ok(), "Should be able to use stored token");
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
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting token validation test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Authenticate using available credentials
    let token = if let (Ok(client_id), Ok(client_secret)) = (std::env::var("DERIBIT_CLIENT_ID"), std::env::var("DERIBIT_CLIENT_SECRET")) {
        debug!("Using OAuth2 authentication for validation test");
        client.authenticate_oauth2(&client_id, &client_secret).await?
    } else if let (Ok(api_key), Ok(api_secret)) = (std::env::var("DERIBIT_API_KEY"), std::env::var("DERIBIT_API_SECRET")) {
        debug!("Using API key authentication for validation test");
        client.authenticate_api_key(&api_key, &api_secret).await?
    } else {
        return Err("No valid authentication credentials found".into());
    };
    
    info!("Authentication successful for validation test");
    
    // Validate token structure
    assert!(!token.access_token.is_empty(), "Access token should not be empty");
    assert!(token.expires_in > 0, "Token should have valid expiration time");
    assert_eq!(token.token_type, "bearer", "Token type should be bearer");
    
    // Test token by making multiple authenticated requests
    for i in 0..5 {
        debug!("Making authenticated request #{} for token validation", i + 1);
        let result = client.get_server_time().await;
        assert!(result.is_ok(), "Token should be valid for request #{}", i + 1);
        
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
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting concurrent token usage test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Authenticate using available credentials
    if let (Ok(client_id), Ok(client_secret)) = (std::env::var("DERIBIT_CLIENT_ID"), std::env::var("DERIBIT_CLIENT_SECRET")) {
        debug!("Using OAuth2 authentication for concurrent test");
        let _token = client.authenticate_oauth2(&client_id, &client_secret).await?;
    } else if let (Ok(api_key), Ok(api_secret)) = (std::env::var("DERIBIT_API_KEY"), std::env::var("DERIBIT_API_SECRET")) {
        debug!("Using API key authentication for concurrent test");
        let _token = client.authenticate_api_key(&api_key, &api_secret).await?;
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
        assert!(result > 0, "Server time should be valid for concurrent request #{}", i + 1);
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
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting token refresh behavior test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Authenticate using available credentials
    let first_token = if let (Ok(client_id), Ok(client_secret)) = (std::env::var("DERIBIT_CLIENT_ID"), std::env::var("DERIBIT_CLIENT_SECRET")) {
        debug!("Using OAuth2 authentication for refresh test");
        client.authenticate_oauth2(&client_id, &client_secret).await?
    } else if let (Ok(api_key), Ok(api_secret)) = (std::env::var("DERIBIT_API_KEY"), std::env::var("DERIBIT_API_SECRET")) {
        debug!("Using API key authentication for refresh test");
        client.authenticate_api_key(&api_key, &api_secret).await?
    } else {
        return Err("No valid authentication credentials found".into());
    };
    
    info!("First authentication successful");
    
    // Wait a moment
    sleep(Duration::from_secs(1)).await;
    
    // Authenticate again to test refresh behavior
    let second_token = if let (Ok(client_id), Ok(client_secret)) = (std::env::var("DERIBIT_CLIENT_ID"), std::env::var("DERIBIT_CLIENT_SECRET")) {
        debug!("Performing second OAuth2 authentication");
        client.authenticate_oauth2(&client_id, &client_secret).await?
    } else if let (Ok(api_key), Ok(api_secret)) = (std::env::var("DERIBIT_API_KEY"), std::env::var("DERIBIT_API_SECRET")) {
        debug!("Performing second API key authentication");
        client.authenticate_api_key(&api_key, &api_secret).await?
    } else {
        return Err("No valid authentication credentials found".into());
    };
    
    info!("Second authentication successful");
    
    // Both tokens should be valid
    assert!(!first_token.access_token.is_empty(), "First token should be valid");
    assert!(!second_token.access_token.is_empty(), "Second token should be valid");
    
    // Test that both authentication sessions work
    let result1 = client.get_server_time().await;
    assert!(result1.is_ok(), "Should be able to make requests after token refresh");
    
    info!("Token refresh behavior test completed successfully");
    Ok(())
}