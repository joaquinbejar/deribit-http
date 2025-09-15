//! Instrument Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_instrument` - Single instrument information
//! - `/public/get_instruments` - Instrument list
//!
//! Usage: cargo run --bin instrument_endpoints

use deribit_http::prelude::*;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();

    info!("🚀 Deribit HTTP Client - Instrument Endpoints Example");
    info!("====================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. GET SINGLE INSTRUMENT (/public/get_instrument)
    // =================================================================
    info!("🎯 1. GET SINGLE INSTRUMENT INFORMATION");
    info!("---------------------------------------");

    // Test with BTC-PERPETUAL
    match client.get_instrument("BTC-PERPETUAL").await {
        Ok(instrument) => {
            info!("✅ BTC-PERPETUAL instrument retrieved successfully");
            info!("📊 Instrument Details:");
            info!("   📝 Name: {}", instrument.instrument_name);
            if let Some(contract_size) = instrument.contract_size {
                info!("   📏 Contract Size: {}", contract_size);
            } else {
                info!("   📏 Contract Size: Not available");
            }
            if let Some(creation) = instrument.creation_timestamp {
                info!("   📅 Creation: {}", creation);
            } else {
                info!("   📅 Creation: Not available");
            }

            if let Some(expiration) = instrument.expiration_timestamp {
                info!("   ⏰ Expiration: {}", expiration);
            } else {
                info!("   ♾️ Expiration: Perpetual (no expiration)");
            }
        }
        Err(e) => {
            error!("❌ Get BTC-PERPETUAL instrument error: {}", e);
        }
    }

    // Test with a BTC future (if available)
    match client.get_instrument("BTC-29MAR24").await {
        Ok(instrument) => {
            info!("✅ BTC future instrument retrieved successfully");
            info!("📊 BTC Future Details:");
            info!("   📝 Name: {}", instrument.instrument_name);
            if let Some(contract_size) = instrument.contract_size {
                info!("   📏 Contract Size: {}", contract_size);
            } else {
                info!("   📏 Contract Size: Not available");
            }
            if let Some(creation) = instrument.creation_timestamp {
                info!("   📅 Creation: {}", creation);
            } else {
                info!("   📅 Creation: Not available");
            }

            if let Some(expiration) = instrument.expiration_timestamp {
                info!("   ⏰ Expiration: {}", expiration);
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC future instrument error: {}", e);
            info!("💡 This is expected if the specific future is not available");
        }
    }

    // Test with a BTC option (if available)
    match client.get_instrument("BTC-29MAR24-60000-C").await {
        Ok(instrument) => {
            info!("✅ BTC option instrument retrieved successfully");
            info!("📊 BTC Option Details:");
            info!("   📝 Name: {}", instrument.instrument_name);
            if let Some(contract_size) = instrument.contract_size {
                info!("   📏 Contract Size: {}", contract_size);
            } else {
                info!("   📏 Contract Size: Not available");
            }
            if let Some(creation) = instrument.creation_timestamp {
                info!("   📅 Creation: {}", creation);
            } else {
                info!("   📅 Creation: Not available");
            }

            if let Some(expiration) = instrument.expiration_timestamp {
                info!("   ⏰ Expiration: {}", expiration);
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC option instrument error: {}", e);
            info!("💡 This is expected if the specific option is not available");
        }
    }

    // Test with ETH-PERPETUAL
    match client.get_instrument("ETH-PERPETUAL").await {
        Ok(instrument) => {
            info!("✅ ETH-PERPETUAL instrument retrieved successfully");
            info!("📊 ETH Perpetual Details:");
            info!("   📝 Name: {}", instrument.instrument_name);
            if let Some(contract_size) = instrument.contract_size {
                info!("   📏 Contract Size: {}", contract_size);
            } else {
                info!("   📏 Contract Size: Not available");
            }
            if let Some(creation) = instrument.creation_timestamp {
                info!("   📅 Creation: {}", creation);
            } else {
                info!("   📅 Creation: Not available");
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH-PERPETUAL instrument error: {}", e);
            info!("💡 This may be expected if ETH perpetual is not available on testnet");
        }
    }

    // Test with non-existent instrument to demonstrate error handling
    match client.get_instrument("NON-EXISTENT-INSTRUMENT").await {
        Ok(instrument) => {
            warn!(
                "⚠️ Unexpected success for non-existent instrument: {}",
                instrument.instrument_name
            );
        }
        Err(e) => {
            info!("✅ Expected error for non-existent instrument: {}", e);
            info!("💡 This demonstrates proper error handling for invalid instrument names");
        }
    }
    println!();

    // =================================================================
    // 2. GET INSTRUMENTS LIST (/public/get_instruments)
    // =================================================================
    info!("📋 2. GET INSTRUMENTS LIST");
    info!("--------------------------");

    // Test with BTC instruments (all kinds)
    match client.get_instruments("BTC", None, Some(false)).await {
        Ok(instruments) => {
            info!("✅ BTC instruments retrieved successfully");
            info!("📊 Found {} BTC instruments:", instruments.len());

            // Group instruments by kind for better display
            let mut futures_count = 0;
            let mut options_count = 0;
            let mut perpetuals_count = 0;
            let mut other_count = 0;

            for instrument in &instruments {
                if instrument.instrument_name.contains("PERPETUAL") {
                    perpetuals_count += 1;
                } else if instrument.instrument_name.contains("-C")
                    || instrument.instrument_name.contains("-P")
                {
                    options_count += 1;
                } else if instrument.instrument_name.len() > 10
                    && !instrument.instrument_name.contains("-")
                {
                    futures_count += 1;
                } else {
                    other_count += 1;
                }
            }

            info!("   🔄 Perpetuals: {}", perpetuals_count);
            info!("   📅 Futures: {}", futures_count);
            info!("   📊 Options: {}", options_count);
            info!("   🔧 Other: {}", other_count);

            // Show first few instruments as examples
            for (i, instrument) in instruments.iter().take(5).enumerate() {
                let contract_size_display = instrument
                    .contract_size
                    .map(|size| size.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                info!(
                    "   {}. {} (Contract Size: {})",
                    i + 1,
                    instrument.instrument_name,
                    contract_size_display
                );
            }

            if instruments.len() > 5 {
                info!(
                    "💡 Showing first 5 of {} BTC instruments",
                    instruments.len()
                );
            }
        }
        Err(e) => {
            error!("❌ Get BTC instruments error: {}", e);
        }
    }

    // Test with BTC futures only
    match client
        .get_instruments("BTC", Some("future"), Some(false))
        .await
    {
        Ok(instruments) => {
            info!("✅ BTC futures retrieved successfully");
            info!("📈 Found {} BTC future instruments:", instruments.len());

            for (i, instrument) in instruments.iter().take(3).enumerate() {
                let creation_display = instrument
                    .creation_timestamp
                    .map(|ts| ts.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                info!(
                    "   {}. {} (Created: {})",
                    i + 1,
                    instrument.instrument_name,
                    creation_display
                );
            }

            if instruments.len() > 3 {
                info!("💡 Showing first 3 of {} BTC futures", instruments.len());
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC futures error: {}", e);
            info!("💡 This may be expected if BTC futures are limited on testnet");
        }
    }

    // Test with BTC options only
    match client
        .get_instruments("BTC", Some("option"), Some(false))
        .await
    {
        Ok(instruments) => {
            info!("✅ BTC options retrieved successfully");
            info!("📊 Found {} BTC option instruments:", instruments.len());

            for (i, instrument) in instruments.iter().take(3).enumerate() {
                let option_type = if instrument.instrument_name.contains("-C") {
                    "Call"
                } else if instrument.instrument_name.contains("-P") {
                    "Put"
                } else {
                    "Unknown"
                };

                let contract_size_display = instrument
                    .contract_size
                    .map(|size| size.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                info!(
                    "   {}. {} ({} Option, Size: {})",
                    i + 1,
                    instrument.instrument_name,
                    option_type,
                    contract_size_display
                );
            }

            if instruments.len() > 3 {
                info!("💡 Showing first 3 of {} BTC options", instruments.len());
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC options error: {}", e);
            info!("💡 This may be expected if BTC options are limited on testnet");
        }
    }

    // Test with ETH instruments
    match client.get_instruments("ETH", None, Some(false)).await {
        Ok(instruments) => {
            info!("✅ ETH instruments retrieved successfully");
            info!("📊 Found {} ETH instruments:", instruments.len());

            // Show summary and first few instruments
            for (i, instrument) in instruments.iter().take(3).enumerate() {
                let contract_size_display = instrument
                    .contract_size
                    .map(|size| size.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                info!(
                    "   {}. {} (Contract Size: {})",
                    i + 1,
                    instrument.instrument_name,
                    contract_size_display
                );
            }

            if instruments.len() > 3 {
                info!(
                    "💡 Showing first 3 of {} ETH instruments",
                    instruments.len()
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH instruments error: {}", e);
            info!("💡 This may be expected if ETH instruments are limited on testnet");
        }
    }

    // Test with USDC instruments (if available)
    match client.get_instruments("USDC", None, Some(false)).await {
        Ok(instruments) => {
            info!("✅ USDC instruments retrieved successfully");
            info!("📊 Found {} USDC instruments", instruments.len());

            if !instruments.is_empty() {
                for (i, instrument) in instruments.iter().take(3).enumerate() {
                    info!("   {}. {}", i + 1, instrument.instrument_name);
                }
            }
        }
        Err(e) => {
            warn!("⚠️ Get USDC instruments error: {}", e);
            info!("💡 This may be expected if USDC instruments are not available");
        }
    }

    // Test with expired instruments
    match client
        .get_instruments("BTC", Some("future"), Some(true))
        .await
    {
        Ok(instruments) => {
            info!("✅ BTC expired futures retrieved successfully");
            info!("📅 Found {} expired BTC futures", instruments.len());

            if !instruments.is_empty() {
                info!("💡 Including expired instruments in the results");
                for (i, instrument) in instruments.iter().take(2).enumerate() {
                    let creation_display = instrument
                        .creation_timestamp
                        .map(|ts| ts.to_string())
                        .unwrap_or_else(|| "N/A".to_string());
                    info!(
                        "   {}. {} (Creation: {})",
                        i + 1,
                        instrument.instrument_name,
                        creation_display
                    );
                }
            } else {
                info!("💡 No expired BTC futures found");
            }
        }
        Err(e) => {
            warn!("⚠️ Get expired BTC futures error: {}", e);
        }
    }

    // Test with invalid currency to demonstrate error handling
    match client.get_instruments("INVALID", None, Some(false)).await {
        Ok(instruments) => {
            if instruments.is_empty() {
                info!("✅ Empty result for invalid currency (expected behavior)");
            } else {
                warn!(
                    "⚠️ Unexpected instruments found for invalid currency: {}",
                    instruments.len()
                );
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid currency: {}", e);
            info!("💡 This demonstrates proper error handling for invalid currency");
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED INSTRUMENT ENDPOINTS");
    info!("==========================================");
    info!("🎯 /public/get_instrument - Single instrument information");
    info!("📋 /public/get_instruments - Instrument list with filters");
    println!();

    info!("🎉 Instrument endpoints example completed successfully!");
    info!(
        "💡 Tip: Use get_instruments() to discover available instruments, then get_instrument() for detailed info"
    );
    info!("📊 These endpoints are essential for understanding available trading instruments");
    info!(
        "🔗 Filter by currency, kind (future/option/spot), and expired status for targeted results"
    );

    Ok(())
}
