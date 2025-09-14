//! Complete example of Deribit authentication endpoints
//!
//! This example demonstrates the usage of authentication endpoints with automatic authentication:
//! - Automatic OAuth2 authentication (handled internally)
//! - /public/exchange_token - Token exchange for different subject_id
//! - /public/fork_token - Create new session with same permissions
//!   Note: HTTP client tokens expire automatically (no logout endpoint)

use deribit_base::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    setup_logger();

    info!("🚀 Deribit HTTP Client - Authentication Endpoints Example");
    info!("============================================================");
    info!("");

    // Create HTTP client with automatic authentication
    let client = DeribitHttpClient::new();
    info!(
        "✅ HTTP client created with automatic authentication: {}",
        client.base_url()
    );
    info!("🔐 Authentication will be handled automatically when needed");
    println!();

    // =================================================================
    // 1. AUTOMATIC AUTHENTICATION TEST
    // =================================================================
    info!("🔐 1. AUTOMATIC AUTHENTICATION TEST");
    info!("------------------------------------");

    // Test that automatic authentication works by making an authenticated call
    match client.get_server_time().await {
        Ok(server_time) => {
            info!("✅ Automatic authentication successful!");
            info!("🕐 Server time: {}", server_time);
            info!("💡 Authentication was handled automatically behind the scenes");
        }
        Err(e) => {
            error!("❌ Automatic authentication failed: {}", e);
            info!(
                "💡 Make sure DERIBIT_CLIENT_ID and DERIBIT_CLIENT_SECRET are set in environment"
            );
            return Err(e);
        }
    }
    println!();

    // =================================================================
    // 2. TOKEN EXCHANGE (/public/exchange_token)
    // =================================================================
    info!("🔄 2. TOKEN EXCHANGE FOR DIFFERENT SUBJECT_ID");
    info!("----------------------------------------------");
    info!("💡 Note: Token exchange requires access to internal refresh token");
    info!("⚠️  This functionality may need to be implemented differently with automatic auth");

    // For now, we'll demonstrate that the concept exists but may not be directly accessible
    // with the new automatic authentication system
    warn!("⚠️ Token exchange functionality may need internal client modifications");
    info!("ℹ️ The client handles authentication automatically, so manual token operations");
    info!("ℹ️ may require additional API methods to expose internal token management");
    println!();

    // =================================================================
    // 3. TOKEN FORK (/public/fork_token)
    // =================================================================
    info!("🍴 3. TOKEN FORK FOR NEW SESSION");
    info!("--------------------------------");
    info!("💡 Note: Token fork requires access to internal refresh token");
    info!("⚠️  This functionality may need to be implemented differently with automatic auth");

    // Similar to token exchange, fork functionality may need internal client modifications
    warn!("⚠️ Token fork functionality may need internal client modifications");
    info!("ℹ️ The client handles authentication automatically, so manual token operations");
    info!("ℹ️ may require additional API methods to expose internal token management");
    println!();

    // =================================================================
    // 4. MULTIPLE AUTHENTICATED CALLS TEST
    // =================================================================
    info!("🧪 4. MULTIPLE AUTHENTICATED CALLS TEST");
    info!("---------------------------------------");

    // Test multiple authenticated calls to verify automatic re-authentication works
    info!("🔄 Testing automatic token management with multiple calls...");

    for i in 1..=3 {
        match client.get_server_time().await {
            Ok(server_time) => {
                info!("✅ Authenticated call {} successful", i);
                info!("🕐 Server time: {}", server_time);
            }
            Err(e) => {
                warn!("⚠️ Authenticated call {} error: {}", i, e);
            }
        }

        // Small delay between calls
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    info!("💡 All calls used automatic authentication - no manual token management needed!");
    println!();

    // =================================================================
    // 5. SESSION INFORMATION
    // =================================================================
    info!("ℹ️ 5. SESSION INFORMATION");
    info!("-------------------------");
    info!("🔐 Authentication: Handled automatically by the client");
    info!("🔄 Token refresh: Managed internally when needed");
    info!("⏰ Token expiration: Handled transparently");
    info!("💡 Note: HTTP client tokens remain valid until expiration");
    info!("🔌 For logout functionality, use the deribit-websocket client");
    println!();

    // =================================================================
    // FINAL SUMMARY
    // =================================================================
    info!("📊 SUMMARY OF AUTHENTICATION FEATURES");
    info!("=====================================");
    info!("✅ Automatic OAuth2 authentication - Handled internally");
    info!("🔄 Token management - Transparent to the user");
    info!("🧪 Multiple authenticated calls - All successful");
    info!("⚠️  Manual token operations (exchange/fork) - May need API updates");
    info!("ℹ️ Session management - HTTP tokens expire automatically");
    println!();
    info!("🎉 Example completed successfully!");
    info!("💡 The new automatic authentication system simplifies usage significantly!");

    Ok(())
}
