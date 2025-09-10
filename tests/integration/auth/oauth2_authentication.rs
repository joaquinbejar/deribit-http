//! OAuth2 Authentication Integration Tests
//!
//! This test covers OAuth2 authentication flow:
//! 1. Authenticate using client credentials
//! 2. Validate token response
//! 3. Test token expiration and renewal
//! 4. Test invalid credentials handling

use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};

use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    // Check if .env file exists
    if !Path::new(".env").exists() {
        return Err(
            "Missing .env file. Please create one with DERIBIT_CLIENT_ID and DERIBIT_CLIENT_SECRET"
                .into(),
        );
    }

    // Load environment variables
    dotenv::dotenv().ok();

    // Check required variables
    let required_vars = ["DERIBIT_CLIENT_ID", "DERIBIT_CLIENT_SECRET"];

    for var in &required_vars {
        if std::env::var(var).is_err() {
            return Err(format!("Missing required environment variable: {}", var).into());
        }
    }

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_oauth2_authentication_success() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;

    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting OAuth2 authentication test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);

    // Test OAuth2 authentication
    let client_id = std::env::var("DERIBIT_CLIENT_ID")?;
    let client_secret = std::env::var("DERIBIT_CLIENT_SECRET")?;

    debug!(
        "Attempting OAuth2 authentication with client_id: {}",
        client_id
    );

    // Perform authentication
    let auth_result = client.authenticate_oauth2(&client_id, &client_secret).await;

    match auth_result {
        Ok(token) => {
            info!("OAuth2 authentication successful");
            debug!("Token: {:?}", token);

            // Validate token structure
            assert!(
                !token.access_token.is_empty(),
                "Access token should not be empty"
            );
            assert!(
                token.expires_in > 0,
                "Token should have valid expiration time"
            );
            assert_eq!(token.token_type, "bearer", "Token type should be bearer");

            // Test that we can make authenticated requests
            let account_summary = client.get_account_summary("BTC", None).await;
            assert!(
                account_summary.is_ok(),
                "Should be able to make authenticated requests after OAuth2 login"
            );

            info!("OAuth2 authentication test completed successfully");
        }
        Err(e) => {
            warn!("OAuth2 authentication failed: {:?}", e);
            return Err(format!("OAuth2 authentication failed: {}", e).into());
        }
    }

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_oauth2_authentication_invalid_credentials() -> Result<(), Box<dyn std::error::Error>>
{
    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting OAuth2 invalid credentials test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);

    // Test OAuth2 authentication with invalid credentials
    let invalid_client_id = "invalid_client_id";
    let invalid_client_secret = "invalid_client_secret";

    debug!("Attempting OAuth2 authentication with invalid credentials");

    // Perform authentication - should fail
    let auth_result = client
        .authenticate_oauth2(invalid_client_id, invalid_client_secret)
        .await;

    match auth_result {
        Ok(_) => {
            return Err("OAuth2 authentication should have failed with invalid credentials".into());
        }
        Err(e) => {
            info!(
                "OAuth2 authentication correctly failed with invalid credentials: {:?}",
                e
            );
            // Verify it's an authentication error
            assert!(
                e.to_string().contains("authentication")
                    || e.to_string().contains("unauthorized")
                    || e.to_string().contains("invalid")
            );
        }
    }

    info!("OAuth2 invalid credentials test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_oauth2_token_renewal() -> Result<(), Box<dyn std::error::Error>> {
    // Check environment setup
    check_env_file()?;

    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting OAuth2 token renewal test");

    // Create HTTP client for testnet
    let client = DeribitHttpClient::new(true);

    // Test OAuth2 authentication
    let client_id = std::env::var("DERIBIT_CLIENT_ID")?;
    let client_secret = std::env::var("DERIBIT_CLIENT_SECRET")?;

    debug!("Performing initial OAuth2 authentication");

    // Perform initial authentication
    let first_token = client
        .authenticate_oauth2(&client_id, &client_secret)
        .await?;
    info!("First authentication successful");

    // Wait a short time
    sleep(Duration::from_secs(2)).await;

    // Perform second authentication (should get new token)
    debug!("Performing second OAuth2 authentication");
    let second_token = client
        .authenticate_oauth2(&client_id, &client_secret)
        .await?;
    info!("Second authentication successful");

    // Tokens might be the same or different depending on server behavior
    // The important thing is both authentications succeeded
    assert!(!first_token.access_token.is_empty());
    assert!(!second_token.access_token.is_empty());

    info!("OAuth2 token renewal test completed successfully");
    Ok(())
}
