//! Subaccount Management Endpoints Example
//!
//! This example demonstrates the subaccount management endpoints added in v0.6.0:
//! - `/private/get_subaccounts` - List all subaccounts
//! - `/private/get_subaccounts_details` - Get subaccount details with positions
//! - `/private/create_subaccount` - Create a new subaccount
//! - `/private/change_subaccount_name` - Rename a subaccount
//! - `/private/set_email_for_subaccount` - Set email for subaccount
//! - `/private/toggle_subaccount_login` - Enable/disable subaccount login
//! - `/private/toggle_notifications_from_subaccount` - Toggle notifications
//! - `/private/remove_subaccount` - Remove a subaccount
//!
//! ⚠️ WARNING: Creating/removing subaccounts has permanent effects!
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin subaccount_endpoints

use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("👥 Subaccount Management Endpoints Example");
    info!("==========================================");
    info!("⚠️  Running on testnet for safety");
    println!();

    // =================================================================
    // 1. GET ALL SUBACCOUNTS
    // =================================================================
    info!("📋 1. GET ALL SUBACCOUNTS");
    info!("--------------------------");

    let subaccounts = match client.get_subaccounts(Some(true)).await {
        Ok(subs) => {
            info!("✅ Retrieved subaccounts successfully");
            info!("   👥 Total subaccounts: {}", subs.len());

            for (i, sub) in subs.iter().enumerate() {
                info!("   📋 Subaccount {}: ID={}", i + 1, sub.id);
                info!("      👤 Username: {}", sub.username);
                info!("      📧 Email: {:?}", sub.email);
                info!("      🔐 Login enabled: {}", sub.login_enabled);
                if let Some(ref portfolio) = sub.portfolio {
                    info!(
                        "      💰 Has portfolio data with {} currencies",
                        portfolio.len()
                    );
                }
                println!();
            }
            subs
        }
        Err(e) => {
            error!("❌ Failed to get subaccounts: {}", e);
            return Err(e);
        }
    };

    // =================================================================
    // 2. GET SUBACCOUNTS DETAILS (WITH POSITIONS)
    // =================================================================
    info!("📊 2. GET SUBACCOUNTS DETAILS");
    info!("------------------------------");

    match client.get_subaccounts_details("BTC", Some(true)).await {
        Ok(details) => {
            info!("✅ Retrieved BTC subaccount details");
            info!("   📋 Subaccounts with details: {}", details.len());

            for detail in details.iter().take(3) {
                info!("   👤 Subaccount UID: {}", detail.uid);
                info!("      📈 Positions: {}", detail.positions.len());
                if let Some(ref orders) = detail.open_orders {
                    info!("      📝 Open orders: {}", orders.len());
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get subaccount details: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. CREATE SUBACCOUNT (DEMONSTRATION)
    // =================================================================
    info!("➕ 3. CREATE SUBACCOUNT");
    info!("------------------------");
    info!("⚠️  Skipping actual creation - permanent action!");
    info!("   Example usage:");
    info!("   let new_sub = client.create_subaccount().await?;");
    info!("   println!(\"Created subaccount ID: {{}}\", new_sub.id);");
    println!();

    // =================================================================
    // 4. CHANGE SUBACCOUNT NAME (DEMONSTRATION)
    // =================================================================
    info!("✏️  4. CHANGE SUBACCOUNT NAME");
    info!("-----------------------------");

    if let Some(first_sub) = subaccounts.first() {
        info!("   Would rename subaccount ID: {}", first_sub.id);
        info!("   Current username: {}", first_sub.username);
        info!("   ⚠️  Skipping actual rename");
        info!("   Example usage:");
        info!(
            "   client.change_subaccount_name({}, \"new_name\").await?;",
            first_sub.id
        );
    } else {
        info!("   No subaccounts available to demonstrate");
    }
    println!();

    // =================================================================
    // 5. SET EMAIL FOR SUBACCOUNT (DEMONSTRATION)
    // =================================================================
    info!("📧 5. SET EMAIL FOR SUBACCOUNT");
    info!("-------------------------------");
    info!("⚠️  Skipping - requires valid email");
    info!("   Example usage:");
    info!("   client.set_email_for_subaccount(subaccount_id, \"user@example.com\").await?;");
    println!();

    // =================================================================
    // 6. TOGGLE SUBACCOUNT LOGIN (DEMONSTRATION)
    // =================================================================
    info!("🔐 6. TOGGLE SUBACCOUNT LOGIN");
    info!("------------------------------");
    info!("⚠️  Skipping - affects account access");
    info!("   Example usage:");
    info!("   // Enable login");
    info!("   client.toggle_subaccount_login(subaccount_id, \"enable\").await?;");
    info!("   // Disable login");
    info!("   client.toggle_subaccount_login(subaccount_id, \"disable\").await?;");
    println!();

    // =================================================================
    // 7. TOGGLE NOTIFICATIONS (DEMONSTRATION)
    // =================================================================
    info!("🔔 7. TOGGLE NOTIFICATIONS FROM SUBACCOUNT");
    info!("-------------------------------------------");
    info!("⚠️  Skipping - affects notification settings");
    info!("   Example usage:");
    info!("   client.toggle_notifications_from_subaccount(subaccount_id, true).await?;");
    println!();

    // =================================================================
    // 8. REMOVE SUBACCOUNT (DEMONSTRATION)
    // =================================================================
    info!("🗑️  8. REMOVE SUBACCOUNT");
    info!("------------------------");
    info!("⚠️  Skipping - PERMANENT and IRREVERSIBLE!");
    info!("   Example usage:");
    info!("   client.remove_subaccount(subaccount_id).await?;");
    info!("   ⚠️  Only works if subaccount has no positions/orders");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated subaccount management endpoints");
    info!("   - Listed {} subaccounts", subaccounts.len());
    info!("   - Retrieved subaccount details");
    info!("   - Showed patterns for create/modify/remove");
    info!("");
    info!("💡 API Key Requirements:");
    info!("   - account:read - for listing subaccounts");
    info!("   - account:read_write - for creating/modifying");

    Ok(())
}
