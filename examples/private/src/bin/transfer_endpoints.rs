//! Transfer Endpoints Example
//!
//! This example demonstrates the transfer management endpoints added in v0.6.0:
//! - `/private/get_transfers` - Get transfer history
//! - `/private/submit_transfer_to_subaccount` - Transfer to subaccount
//! - `/private/submit_transfer_to_user` - Transfer to another user
//! - `/private/submit_transfer_between_subaccounts` - Transfer between subaccounts
//! - `/private/cancel_transfer_by_id` - Cancel a pending transfer
//!
//! ⚠️ WARNING: Transfer operations involve real assets!
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin transfer_endpoints

use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("💸 Transfer Endpoints Example");
    info!("==============================");
    info!("⚠️  Running on testnet for safety");
    println!();

    // =================================================================
    // 1. GET TRANSFERS HISTORY
    // =================================================================
    info!("📋 1. GET TRANSFERS HISTORY");
    info!("----------------------------");

    match client.get_transfers("BTC", Some(10), None).await {
        Ok(response) => {
            info!("✅ Retrieved transfer history");
            info!("   📊 Total transfers: {}", response.count);
            info!("   📋 Retrieved: {}", response.data.len());

            for (i, transfer) in response.data.iter().take(5).enumerate() {
                info!("   💸 Transfer {}:", i + 1);
                info!("      🆔 ID: {}", transfer.id);
                info!("      💱 Currency: {}", transfer.currency);
                info!("      💰 Amount: {}", transfer.amount);
                info!("      📊 State: {:?}", transfer.state);
                info!("      🔄 Direction: {:?}", transfer.direction);
                info!("      👤 Other side: {}", transfer.other_side);
                println!();
            }

            if response.data.len() > 5 {
                info!("   ... and {} more transfers", response.data.len() - 5);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get transfers: {}", e);
            info!("   This may be normal if no transfers exist");
        }
    }
    println!();

    // =================================================================
    // 2. GET ETH TRANSFERS
    // =================================================================
    info!("📋 2. GET ETH TRANSFERS");
    info!("------------------------");

    match client.get_transfers("ETH", Some(5), None).await {
        Ok(response) => {
            info!("✅ Retrieved ETH transfer history");
            info!("   📊 Total ETH transfers: {}", response.count);
        }
        Err(e) => {
            warn!("⚠️  Could not get ETH transfers: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. SUBMIT TRANSFER TO SUBACCOUNT (DEMONSTRATION)
    // =================================================================
    info!("➡️  3. SUBMIT TRANSFER TO SUBACCOUNT");
    info!("-------------------------------------");
    info!("⚠️  Skipping - involves real asset transfer!");
    info!("   Example usage:");
    info!("   let transfer = client.submit_transfer_to_subaccount(");
    info!("       \"BTC\",");
    info!("       0.001,  // Amount");
    info!("       12345   // Destination subaccount ID");
    info!("   ).await?;");
    info!("   println!(\"Transfer ID: {{}}\", transfer.id);");
    println!();

    // =================================================================
    // 4. SUBMIT TRANSFER TO USER (DEMONSTRATION)
    // =================================================================
    info!("👤 4. SUBMIT TRANSFER TO USER");
    info!("------------------------------");
    info!("⚠️  Skipping - involves real asset transfer!");
    info!("   Example usage:");
    info!("   let transfer = client.submit_transfer_to_user(");
    info!("       \"ETH\",");
    info!("       0.1,          // Amount");
    info!("       \"user@dest\"   // Destination user identifier");
    info!("   ).await?;");
    println!();

    // =================================================================
    // 5. SUBMIT TRANSFER BETWEEN SUBACCOUNTS (DEMONSTRATION)
    // =================================================================
    info!("🔄 5. SUBMIT TRANSFER BETWEEN SUBACCOUNTS");
    info!("------------------------------------------");
    info!("⚠️  Skipping - involves real asset transfer!");
    info!("   Example usage:");
    info!("   let transfer = client.submit_transfer_between_subaccounts(");
    info!("       \"USDC\",");
    info!("       100.0,    // Amount");
    info!("       12345,    // Destination subaccount ID");
    info!("       Some(67890)  // Source subaccount ID (optional)");
    info!("   ).await?;");
    println!();

    // =================================================================
    // 6. CANCEL TRANSFER (DEMONSTRATION)
    // =================================================================
    info!("❌ 6. CANCEL TRANSFER");
    info!("---------------------");
    info!("⚠️  Skipping - requires pending transfer ID");
    info!("   Example usage:");
    info!("   let cancelled = client.cancel_transfer_by_id(");
    info!("       \"BTC\",");
    info!("       transfer_id");
    info!("   ).await?;");
    info!("   println!(\"Cancelled transfer state: {{}}\", cancelled.state);");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated transfer endpoints");
    info!("   - Retrieved transfer history");
    info!("   - Showed transfer patterns (not executed)");
    info!("");
    info!("💡 Transfer Types:");
    info!("   - To Subaccount: Within your account structure");
    info!("   - To User: External user transfer");
    info!("   - Between Subaccounts: Move between your subaccounts");
    info!("");
    info!("🔐 API Key Requirements:");
    info!("   - wallet:read - for listing transfers");
    info!("   - wallet:read_write - for submitting transfers");

    Ok(())
}
