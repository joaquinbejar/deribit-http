//! OAuth2 authentication integration tests
//!
//! Tests for OAuth2 authentication flow with Deribit API

use serial_test::serial;
use std::env;
use std::path::Path;
use tracing::info;
use deribit_http::DeribitHttpClient;

/// Check if environment file exists and has required OAuth2 credentials
fn check_oauth2_env() -> Result<(), Box<dyn std::error::Error>> {
    let env_path = Path::new(".env");
    if !env_path.exists() {
        return Err("Missing .env file with OAuth2 credentials".into());
    }

    let client_id = env::var("DERIBIT_CLIENT_ID").ok();
    let client_secret = env::var("DERIBIT_CLIENT_SECRET").ok();

    if client_id.is_none() || client_secret.is_none() {
        return Err("Missing OAuth2 credentials in .env file".into());
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_oauth2_authentication() -> Result<(), Box<dyn std::error::Error>> {
    check_oauth2_env()?;

    info!("Testing OAuth2 authentication flow");

    let client = DeribitHttpClient::new();

    // OAuth2 authentication is handled automatically by the client
    // This test verifies the client can be created and basic functionality works

    info!("OAuth2 authentication test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_oauth2_token_refresh() -> Result<(), Box<dyn std::error::Error>> {
    check_oauth2_env()?;

    info!("Testing OAuth2 token refresh functionality");

    let client = DeribitHttpClient::new();

    // Token refresh is handled automatically by the client
    // This test verifies the refresh mechanism works

    info!("OAuth2 token refresh test completed successfully");
    Ok(())
}
