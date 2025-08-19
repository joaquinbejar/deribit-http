//! API Key Authentication Integration Tests
//!
//! This test covers API key authentication flow:
//! 1. Authenticate using API key and secret
//! 2. Validate authentication response
//! 3. Test signature generation
//! 4. Test invalid API key handling

use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};

use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    // Check if .env file exists
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with DERIBIT_API_KEY and DERIBIT_API_SECRET".into());
    }

    // Load environment variables
    dotenv::dotenv().ok();

    // Check required variables
    let required_vars = [
        "DERIBIT_API_KEY",
        "DERIBIT_API_SECRET",
    ];

    for var in &required_vars {
        if std::env::var(var).is_err() {
            return Err(format!("Missing required environment variable: {}", var).into());
        }
    }

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_api_key_authentication_success() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;
    
    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting API key authentication test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Test API key authentication
    let api_key = std::env::var("DERIBIT_API_KEY")?;
    let api_secret = std::env::var("DERIBIT_API_SECRET")?;
    
    debug!("Attempting API key authentication with key: {}...", &api_key[..8]);
    
    // Perform authentication
    let auth_result = client.authenticate_api_key(&api_key, &api_secret).await;
    
    match auth_result {
        Ok(token) => {
            info!("API key authentication successful");
            debug!("Token: {:?}", token);
            
            // Validate token structure
            assert!(!token.access_token.is_empty(), "Access token should not be empty");
            assert!(token.expires_in > 0, "Token should have valid expiration time");
            assert_eq!(token.token_type, "bearer", "Token type should be bearer");
            
            // Test that we can make authenticated requests
            let account_summary = client.get_account_summary("BTC", None).await;
            assert!(account_summary.is_ok(), "Should be able to make authenticated requests after API key login");
            
            info!("API key authentication test completed successfully");
        }
        Err(e) => {
            warn!("API key authentication failed: {:?}", e);
            return Err(format!("API key authentication failed: {}", e).into());
        }
    }

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_api_key_authentication_invalid_credentials() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting API key invalid credentials test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Test API key authentication with invalid credentials
    let invalid_api_key = "invalid_api_key";
    let invalid_api_secret = "invalid_api_secret";
    
    debug!("Attempting API key authentication with invalid credentials");
    
    // Perform authentication - should fail
    let auth_result = client.authenticate_api_key(invalid_api_key, invalid_api_secret).await;
    
    match auth_result {
        Ok(_) => {
            return Err("API key authentication should have failed with invalid credentials".into());
        }
        Err(e) => {
            info!("API key authentication correctly failed with invalid credentials: {:?}", e);
            // Verify it's an authentication error
            assert!(e.to_string().contains("authentication") || e.to_string().contains("unauthorized") || e.to_string().contains("invalid"));
        }
    }

    info!("API key invalid credentials test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_api_key_signature_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;
    
    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting API key signature validation test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Test API key authentication
    let api_key = std::env::var("DERIBIT_API_KEY")?;
    let api_secret = std::env::var("DERIBIT_API_SECRET")?;
    
    debug!("Testing API key signature validation");
    
    // Perform authentication
    let auth_result = client.authenticate_api_key(&api_key, &api_secret).await?;
    info!("API key authentication successful for signature test");
    
    // Make multiple authenticated requests to test signature consistency
    for i in 0..3 {
        debug!("Making authenticated request #{}", i + 1);
        let result = client.get_server_time().await;
        assert!(result.is_ok(), "Authenticated request #{} should succeed", i + 1);
        
        // Small delay between requests
        sleep(Duration::from_millis(100)).await;
    }
    
    info!("API key signature validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_api_key_token_expiration() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;
    
    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting API key token expiration test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);
    
    // Test API key authentication
    let api_key = std::env::var("DERIBIT_API_KEY")?;
    let api_secret = std::env::var("DERIBIT_API_SECRET")?;
    
    debug!("Performing initial API key authentication");
    
    // Perform initial authentication
    let first_token = client.authenticate_api_key(&api_key, &api_secret).await?;
    info!("First authentication successful");
    
    // Verify token has reasonable expiration time
    assert!(first_token.expires_in > 60, "Token should expire in more than 60 seconds");
    assert!(first_token.expires_in < 86400, "Token should expire in less than 24 hours");
    
    // Wait a short time
    sleep(Duration::from_secs(2)).await;
    
    // Perform second authentication (should get new token or same token)
    debug!("Performing second API key authentication");
    let second_token = client.authenticate_api_key(&api_key, &api_secret).await?;
    info!("Second authentication successful");
    
    // Both tokens should be valid
    assert!(!first_token.access_token.is_empty());
    assert!(!second_token.access_token.is_empty());
    
    info!("API key token expiration test completed successfully");
    Ok(())
}