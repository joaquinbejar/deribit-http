//! Wallet Endpoints Example
//!
//! This example demonstrates the wallet management endpoints added in v0.6.0:
//! - `/private/get_current_deposit_address` - Get current deposit address
//! - `/private/create_deposit_address` - Create a new deposit address
//! - `/private/get_address_book` - Get address book entries
//! - `/private/add_to_address_book` - Add address to address book
//! - `/private/remove_from_address_book` - Remove address from address book
//! - `/private/withdraw` - Create withdrawal request
//! - `/private/cancel_withdrawal` - Cancel pending withdrawal
//!
//! ⚠️ WARNING: Some endpoints involve real assets. Use testnet only!
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin wallet_endpoints

use deribit_http::model::wallet::AddressBookType;
use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("🔐 Wallet Endpoints Example");
    info!("============================");
    info!("⚠️  Running on testnet for safety");
    println!();

    // =================================================================
    // 1. GET CURRENT DEPOSIT ADDRESS
    // =================================================================
    info!("📥 1. GET CURRENT DEPOSIT ADDRESS");
    info!("----------------------------------");

    match client.get_current_deposit_address("BTC").await {
        Ok(address) => {
            info!("✅ Retrieved current BTC deposit address");
            info!("   📍 Address: {}", address.address);
            info!("   💱 Currency: {}", address.currency);
            if let Some(addr_type) = &address.address_type {
                info!("   📋 Type: {}", addr_type);
            }
            if let Some(ts) = address.creation_timestamp {
                info!("   🕐 Created: {}", ts);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get deposit address: {}", e);
            info!("   This may be normal if no address exists yet");
        }
    }
    println!();

    // =================================================================
    // 2. CREATE NEW DEPOSIT ADDRESS
    // =================================================================
    info!("📥 2. CREATE NEW DEPOSIT ADDRESS");
    info!("---------------------------------");

    match client.create_deposit_address("BTC").await {
        Ok(address) => {
            info!("✅ Created new BTC deposit address");
            info!("   📍 Address: {}", address.address);
            info!("   💱 Currency: {}", address.currency);
        }
        Err(e) => {
            warn!("⚠️  Could not create deposit address: {}", e);
            info!("   Some accounts may have address creation limits");
        }
    }
    println!();

    // =================================================================
    // 3. GET ADDRESS BOOK (WITHDRAWAL ADDRESSES)
    // =================================================================
    info!("📖 3. GET ADDRESS BOOK (WITHDRAWAL)");
    info!("------------------------------------");

    match client
        .get_address_book("BTC", AddressBookType::Withdrawal)
        .await
    {
        Ok(entries) => {
            info!("✅ Retrieved address book entries");
            info!("   📋 Total entries: {}", entries.len());

            for (i, entry) in entries.iter().take(5).enumerate() {
                info!("   📍 Entry {}: {}", i + 1, entry.address);
                if let Some(label) = &entry.label {
                    info!("      🏷️  Label: {}", label);
                }
                info!("      💱 Currency: {}", entry.currency);
            }

            if entries.len() > 5 {
                info!("   ... and {} more entries", entries.len() - 5);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get address book: {}", e);
        }
    }
    println!();

    // =================================================================
    // 4. ADD TO ADDRESS BOOK (DEMONSTRATION)
    // =================================================================
    info!("➕ 4. ADD TO ADDRESS BOOK");
    info!("-------------------------");
    info!("⚠️  Skipping actual add - requires valid external address");
    info!("   Example usage:");
    info!("   client.add_to_address_book(");
    info!("       \"BTC\",");
    info!("       AddressBookType::Withdrawal,");
    info!("       \"bc1q...\",  // Valid BTC address");
    info!("       Some(\"My Wallet\"),");
    info!("       None");
    info!("   ).await?;");
    println!();

    // =================================================================
    // 5. GET TRANSFER ADDRESSES
    // =================================================================
    info!("🔄 5. GET ADDRESS BOOK (TRANSFER)");
    info!("----------------------------------");

    match client
        .get_address_book("BTC", AddressBookType::Transfer)
        .await
    {
        Ok(entries) => {
            info!("✅ Retrieved transfer address book");
            info!("   📋 Total entries: {}", entries.len());

            for (i, entry) in entries.iter().take(3).enumerate() {
                info!("   📍 Entry {}: {}", i + 1, entry.address);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get transfer addresses: {}", e);
        }
    }
    println!();

    // =================================================================
    // 6. WITHDRAW (DEMONSTRATION ONLY)
    // =================================================================
    info!("💸 6. WITHDRAW");
    info!("--------------");
    info!("⚠️  Skipping actual withdrawal - involves real assets!");
    info!("   Example usage:");
    info!("   client.withdraw(");
    info!("       \"BTC\",");
    info!("       \"bc1q...\",  // Must be in address book");
    info!("       0.001,       // Amount");
    info!("       Some(WithdrawalPriorityLevel::High)");
    info!("   ).await?;");
    println!();

    // =================================================================
    // 7. CANCEL WITHDRAWAL (DEMONSTRATION ONLY)
    // =================================================================
    info!("❌ 7. CANCEL WITHDRAWAL");
    info!("-----------------------");
    info!("⚠️  Skipping - requires pending withdrawal ID");
    info!("   Example usage:");
    info!("   client.cancel_withdrawal(\"BTC\", withdrawal_id).await?;");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated wallet endpoint usage");
    info!("   - Deposit address retrieval and creation");
    info!("   - Address book management");
    info!("   - Withdrawal/cancellation patterns (not executed)");
    info!("");
    info!("💡 For actual withdrawals, ensure:");
    info!("   1. Address is in your address book");
    info!("   2. Sufficient balance available");
    info!("   3. API key has wallet:read_write scope");

    Ok(())
}
