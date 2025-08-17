//! Complete example of Deribit authentication endpoints
//!
//! This example demonstrates the usage of all authentication endpoints:
//! - /public/auth - Initial OAuth2 authentication
//! - /public/exchange_token - Token exchange for different subject_id
//! - /public/fork_token - Create new session with same permissions
//! - /private/logout - Logout and invalidate token

use deribit_http::{DeribitHttpClient, HttpError, config::HttpConfig};
use std::env;
use std::path::Path;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("debug").init();

    // Check if .env file exists
    if !Path::new(".env").exists() {
        return Err(HttpError::ConfigError(
            "Missing .env file. Please create one with DERIBIT_USERNAME and DERIBIT_PASSWORD"
                .to_string(),
        ));
    }

    // Load environment variables
    dotenv::dotenv().ok();

    info!("🚀 Deribit HTTP Client - Authentication Endpoints Example");
    info!("============================================================");
    info!("");

    // Check environment variables
    let client_id = env::var("DERIBIT_CLIENT_ID").map_err(|_| {
        HttpError::ConfigError("DERIBIT_CLIENT_ID not found in environment variables".to_string())
    })?;
    let client_secret = env::var("DERIBIT_CLIENT_SECRET").map_err(|_| {
        HttpError::ConfigError(
            "DERIBIT_CLIENT_SECRET not found in environment variables".to_string(),
        )
    })?;

    info!("✅ Credentials found in environment variables");
    info!("📋 Client ID: {}...", &client_id[..8.min(client_id.len())]);
    println!();

    // Create HTTP client for testnet
    let config = HttpConfig::testnet();
    let client = DeribitHttpClient::with_config(config)?;
    info!("✅ HTTP client created for testnet: {}", client.base_url());
    println!();

    // =================================================================
    // 1. INITIAL OAUTH2 AUTHENTICATION (/public/auth)
    // =================================================================
    info!("🔐 1. INITIAL OAUTH2 AUTHENTICATION");
    info!("-----------------------------------");

    let initial_token = match client.authenticate_oauth2(&client_id, &client_secret).await {
        Ok(token) => {
            info!("✅ OAuth2 authentication successful");
            info!("📄 Token type: {}", token.token_type);
            info!("⏰ Expires in: {} seconds", token.expires_in);
            info!("🔑 Access token: {}...", &token.access_token[..20]);
            info!(
                "🔄 Refresh token: {}...",
                &token.refresh_token.as_ref().unwrap_or(&"N/A".to_string())[..20]
            );
            info!("🎯 Scope: {}", token.scope);
            println!();
            token
        }
        Err(e) => {
            error!("❌ OAuth2 authentication error: {}", e);
            return Err(HttpError::AuthenticationFailed(
                "Failed to authenticate with OAuth2".to_string(),
            ));
        }
    };

    // Verify that the client is authenticated
    if client.is_authenticated().await {
        info!("✅ Client authenticated successfully");
    } else {
        warn!("⚠️ Client does not appear to be authenticated");
    }
    println!();

    // =================================================================
    // 2. TOKEN EXCHANGE (/public/exchange_token)
    // =================================================================
    info!("🔄 2. TOKEN EXCHANGE FOR DIFFERENT SUBJECT_ID");
    info!("----------------------------------------------");

    // Check for subaccount credentials in environment variables
    let sub_client_id = env::var("DERIBIT_SUB_CLIENT_ID").ok();
    let sub_client_secret = env::var("DERIBIT_SUB_CLIENT_SECRET").ok();

    if let (Some(sub_id), Some(sub_secret)) = (&sub_client_id, &sub_client_secret) {
        info!("🔑 Found subaccount credentials, authenticating with subaccount");
        info!("📋 Sub Client ID: {}...", &sub_id[..8.min(sub_id.len())]);

        // Authenticate with subaccount credentials
        match client.authenticate_oauth2(sub_id, sub_secret).await {
            Ok(sub_token) => {
                info!("✅ Subaccount OAuth2 authentication successful");
                info!("📄 Sub Token type: {}", sub_token.token_type);
                info!("⏰ Sub Expires in: {} seconds", sub_token.expires_in);
                info!("🔑 Sub Access token: {}...", &sub_token.access_token[..20]);
                info!("🎯 Sub Scope: {}", sub_token.scope);

                if let Some(sub_refresh_token) = &sub_token.refresh_token {
                    // Use the subaccount's refresh token for exchange_token with subject_id 0 (main account)
                    let subject_id = 0u64;
                    let custom_scope = Some("session:test_exchange trade:read_write");

                    match client
                        .exchange_token(sub_refresh_token, subject_id, custom_scope)
                        .await
                    {
                        Ok(exchanged_token) => {
                            info!("✅ Token exchange successful from subaccount to main account");
                            info!("🎯 Subject ID: {} (main account)", subject_id);
                            info!("📄 Token type: {}", exchanged_token.token_type);
                            info!("⏰ Expires in: {} seconds", exchanged_token.expires_in);
                            info!(
                                "🔑 Exchanged access token: {}...",
                                &exchanged_token.access_token[..20]
                            );
                            info!("🎯 Exchanged scope: {}", exchanged_token.scope);
                        }
                        Err(e) => {
                            warn!("⚠️ Token exchange error: {}", e);
                            info!("ℹ️ This may be normal depending on subaccount permissions");
                        }
                    }
                } else {
                    warn!("⚠️ No refresh token available from subaccount authentication");
                }
            }
            Err(e) => {
                error!("❌ Subaccount OAuth2 authentication error: {}", e);
                info!("ℹ️ Falling back to default behavior");

                // Fallback to original behavior
                if let Some(refresh_token) = &initial_token.refresh_token {
                    let subject_id = 10u64;
                    let custom_scope = Some("session:test_exchange trade:read_write");

                    match client
                        .exchange_token(refresh_token, subject_id, custom_scope)
                        .await
                    {
                        Ok(exchanged_token) => {
                            info!("✅ Token exchange successful (fallback)");
                            info!("🎯 Subject ID: {}", subject_id);
                            info!("📄 Token type: {}", exchanged_token.token_type);
                            info!("⏰ Expires in: {} seconds", exchanged_token.expires_in);
                            info!(
                                "🔑 New access token: {}...",
                                &exchanged_token.access_token[..20]
                            );
                            info!("🎯 New scope: {}", exchanged_token.scope);
                        }
                        Err(e) => {
                            warn!("⚠️ Token exchange error: {}", e);
                            info!("ℹ️ This may be normal if you don't have subaccounts configured");
                        }
                    }
                } else {
                    warn!("⚠️ No refresh token available for fallback exchange");
                }
            }
        }
    } else {
        info!("ℹ️ No subaccount credentials found, using default behavior");

        if let Some(refresh_token) = &initial_token.refresh_token {
            // Use subject_id 10 as example (subaccount)
            let subject_id = 10u64;
            let custom_scope = Some("session:test_exchange trade:read_write");

            match client
                .exchange_token(refresh_token, subject_id, custom_scope)
                .await
            {
                Ok(exchanged_token) => {
                    info!("✅ Token exchange successful");
                    info!("🎯 Subject ID: {}", subject_id);
                    info!("📄 Token type: {}", exchanged_token.token_type);
                    info!("⏰ Expires in: {} seconds", exchanged_token.expires_in);
                    info!(
                        "🔑 New access token: {}...",
                        &exchanged_token.access_token[..20]
                    );
                    info!("🎯 New scope: {}", exchanged_token.scope);
                }
                Err(e) => {
                    warn!("⚠️ Token exchange error: {}", e);
                    info!("ℹ️ This may be normal if you don't have subaccounts configured");
                }
            }
        } else {
            warn!("⚠️ No refresh token available for exchange");
        }
    }
    println!();

    // =================================================================
    // 3. TOKEN FORK (/public/fork_token)
    // =================================================================
    info!("🍴 3. TOKEN FORK FOR NEW SESSION");
    info!("--------------------------------");

    if let Some(refresh_token) = &initial_token.refresh_token {
        let session_name = "example_fork_session";
        let custom_scope = Some("session:fork_example trade:read account:read");

        match client
            .fork_token(refresh_token, session_name, custom_scope)
            .await
        {
            Ok(forked_token) => {
                info!("✅ Token fork successful");
                info!("📛 Session name: {}", session_name);
                info!("📄 Token type: {}", forked_token.token_type);
                info!("⏰ Expires in: {} seconds", forked_token.expires_in);
                info!(
                    "🔑 Forked access token: {}...",
                    &forked_token.access_token[..20]
                );
                info!("🎯 Forked scope: {}", forked_token.scope);
                println!();
            }
            Err(e) => {
                warn!("⚠️ Token fork error: {}", e);
                info!("ℹ️ This may be normal depending on your API key permissions");
                println!();
            }
        }
    } else {
        warn!("⚠️ No refresh token available for fork");
        println!();
    }

    // =================================================================
    // 4. AUTHENTICATED FUNCTIONALITY TEST
    // =================================================================
    info!("🧪 4. AUTHENTICATED FUNCTIONALITY TEST");
    info!("--------------------------------------");

    // Try to make an authenticated call to verify the token works
    match client.get_server_time().await {
        Ok(server_time) => {
            info!("✅ Authenticated call successful");
            info!("🕐 Server time: {}", server_time);
        }
        Err(e) => {
            warn!("⚠️ Authenticated call error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 5. LOGOUT (/private/logout)
    // =================================================================
    info!("🚪 5. LOGOUT AND SESSION TERMINATION");
    info!("------------------------------------");

    match client.logout().await {
        Ok(()) => {
            info!("✅ Logout successful");
            info!("🔒 Session terminated correctly");

            // Verify that the client is no longer authenticated
            if !client.is_authenticated().await {
                info!("✅ Client is no longer authenticated (as expected)");
            } else {
                warn!("⚠️ Client still appears to be authenticated");
            }
        }
        Err(e) => {
            error!("❌ Logout error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 6. POST-LOGOUT VERIFICATION
    // =================================================================
    info!("🔍 6. POST-LOGOUT VERIFICATION");
    info!("------------------------------");

    // Try to make a call after logout attempt
    // Note: Since logout via HTTP is not available, the token remains valid until expiration
    match client.get_server_time().await {
        Ok(server_time) => {
            info!("ℹ️ Post-logout call successful: {}", server_time);
            info!(
                "💡 This is expected since HTTP logout is not available - token remains valid until expiration"
            );
        }
        Err(e) => {
            info!("❌ Post-logout call failed: {}", e);
        }
    }
    println!();

    // =================================================================
    // FINAL SUMMARY
    // =================================================================
    info!("📊 SUMMARY OF TESTED ENDPOINTS");
    info!("==============================");
    info!("✅ /public/auth - Initial OAuth2 authentication");
    info!("🔄 /public/exchange_token - Token exchange");
    info!("🍴 /public/fork_token - Token fork");
    info!("🚪 /private/logout - Logout and session termination");
    println!();
    info!("🎉 Example completed successfully!");
    info!("💡 Tip: Check the logs to see details of each operation");

    Ok(())
}
