//! Settlement Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_last_settlements_by_currency` - Settlement history by currency
//! - `/public/get_last_settlements_by_instrument` - Settlement history by instrument
//!
//! Usage: cargo run --bin settlement_endpoints

use deribit_http::DeribitHttpClient;
use std::env;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    info!("🚀 Deribit HTTP Client - Settlement Endpoints Example");
    info!("====================================================");
    println!();

    // Determine if we should use testnet or production
    let use_testnet = env::var("DERIBIT_TESTNET")
        .map(|val| val.to_lowercase() == "true")
        .unwrap_or(true); // Default to testnet for safety

    info!(
        "🌐 Environment: {}",
        if use_testnet { "Testnet" } else { "Production" }
    );

    // Create HTTP client
    let client = DeribitHttpClient::new(use_testnet);
    info!(
        "✅ HTTP client created for {}: {}",
        if use_testnet { "testnet" } else { "production" },
        client.base_url()
    );
    println!();

    // =================================================================
    // 1. GET LAST SETTLEMENTS BY CURRENCY (/public/get_last_settlements_by_currency)
    // =================================================================
    info!("🏦 1. GET LAST SETTLEMENTS BY CURRENCY");
    info!("--------------------------------------");

    // Test with BTC settlements (all types)
    match client
        .get_last_settlements_by_currency("BTC", None, Some(10), None, None)
        .await
    {
        Ok(settlements) => {
            info!("✅ Settlements for BTC retrieved successfully");
            info!(
                "📊 Found {} settlement records:",
                settlements.settlements.len()
            );

            for (i, settlement) in settlements.settlements.iter().take(5).enumerate() {
                info!(
                    "   {}. Type: {} - Timestamp: {}",
                    i + 1,
                    settlement.settlement_type,
                    settlement.timestamp
                );

                if let Some(instrument) = &settlement.instrument_name {
                    info!("      Instrument: {}", instrument);
                }

                if let Some(pnl) = settlement.profit_loss {
                    info!("      P&L: {:.6}", pnl);
                }

                let session_pnl = settlement.session_profit_loss.unwrap_or(0.0);
                info!("      Session P&L: {:.6}", session_pnl);
            }

            if settlements.settlements.len() > 5 {
                info!(
                    "💡 Showing first 5 of {} BTC settlements",
                    settlements.settlements.len()
                );
            }

            if let Some(continuation) = &settlements.continuation {
                info!(
                    "🔗 Continuation token available for pagination: {}...",
                    &continuation[..20.min(continuation.len())]
                );
            }

            if settlements.settlements.is_empty() {
                info!("💡 No settlement data available for BTC");
            }
        }
        Err(e) => {
            warn!("⚠️ Get settlements for BTC error: {}", e);
            info!("💡 This may be expected if no recent settlements are available");
        }
    }

    // Test with BTC delivery settlements only
    match client
        .get_last_settlements_by_currency("BTC", Some("delivery"), Some(5), None, None)
        .await
    {
        Ok(settlements) => {
            info!("✅ BTC delivery settlements retrieved successfully");
            info!(
                "📊 Found {} delivery settlement records",
                settlements.settlements.len()
            );

            for (i, settlement) in settlements.settlements.iter().enumerate() {
                info!(
                    "   {}. Delivery settlement at {}",
                    i + 1,
                    settlement.timestamp
                );

                if let Some(instrument) = &settlement.instrument_name {
                    info!("      Instrument: {}", instrument);
                }

                if let Some(mark_price) = settlement.mark_price {
                    info!("      Mark Price: {:.2}", mark_price);
                }

                if let Some(index_price) = settlement.index_price {
                    info!("      Index Price: {:.2}", index_price);
                }
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC delivery settlements error: {}", e);
            info!("💡 This may be expected if no delivery settlements are available");
        }
    }

    // Test with ETH settlements
    match client
        .get_last_settlements_by_currency("ETH", None, Some(3), None, None)
        .await
    {
        Ok(settlements) => {
            info!("✅ ETH settlements retrieved successfully");
            info!(
                "📊 Found {} ETH settlement records",
                settlements.settlements.len()
            );

            for settlement in &settlements.settlements {
                let session_pnl = settlement.session_profit_loss.unwrap_or(0.0);
                info!(
                    "   ETH settlement: {} - Session P&L: {:.6}",
                    settlement.settlement_type, session_pnl
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH settlements error: {}", e);
            info!("💡 This may be expected if ETH settlements are limited on testnet");
        }
    }
    println!();

    // =================================================================
    // 2. GET LAST SETTLEMENTS BY INSTRUMENT (/public/get_last_settlements_by_instrument)
    // =================================================================
    info!("🎯 2. GET LAST SETTLEMENTS BY INSTRUMENT");
    info!("----------------------------------------");

    // Test with BTC-PERPETUAL
    match client
        .get_last_settlements_by_instrument(
            "BTC-PERPETUAL",
            Some("settlement"),
            Some(5),
            None,
            None,
        )
        .await
    {
        Ok(settlements) => {
            info!("✅ Settlements for BTC-PERPETUAL retrieved successfully");
            info!(
                "📊 Found {} settlement records for BTC-PERPETUAL:",
                settlements.settlements.len()
            );

            for (i, settlement) in settlements.settlements.iter().enumerate() {
                info!(
                    "   {}. Settlement at {} - Type: {}",
                    i + 1,
                    settlement.timestamp,
                    settlement.settlement_type
                );

                if let Some(position_size) = settlement.position_size {
                    info!("      Position Size: {:.6}", position_size);
                }

                if let Some(pnl) = settlement.profit_loss {
                    info!("      P&L: {:.6}", pnl);
                }

                let session_pnl = settlement.session_profit_loss.unwrap_or(0.0);
                info!("      Session P&L: {:.6}", session_pnl);

                if let Some(funding) = settlement.funding {
                    info!("      Funding: {:.6}", funding);
                }
            }

            if settlements.settlements.is_empty() {
                info!("💡 No settlement data available for BTC-PERPETUAL");
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC-PERPETUAL settlements error: {}", e);
            info!(
                "💡 This may be expected if no recent settlements are available for this instrument"
            );
        }
    }

    // Test with ETH-PERPETUAL
    match client
        .get_last_settlements_by_instrument("ETH-PERPETUAL", None, Some(3), None, None)
        .await
    {
        Ok(settlements) => {
            info!("✅ Settlements for ETH-PERPETUAL retrieved successfully");
            info!(
                "📊 Found {} settlement records for ETH-PERPETUAL",
                settlements.settlements.len()
            );

            for (i, settlement) in settlements.settlements.iter().enumerate() {
                let session_pnl = settlement.session_profit_loss.unwrap_or(0.0);
                info!(
                    "   {}. {} settlement - Session P&L: {:.6}",
                    i + 1,
                    settlement.settlement_type,
                    session_pnl
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH-PERPETUAL settlements error: {}", e);
            info!("💡 This may be expected if ETH-PERPETUAL settlements are not available");
        }
    }

    // Test with a BTC future (demonstrating error handling for inactive instruments)
    match client
        .get_last_settlements_by_instrument("BTC-29MAR24", Some("delivery"), Some(2), None, None)
        .await
    {
        Ok(settlements) => {
            info!("✅ Delivery settlements for BTC future retrieved successfully");
            info!(
                "📊 Found {} delivery records for BTC future",
                settlements.settlements.len()
            );

            for settlement in &settlements.settlements {
                info!(
                    "   Future delivery settlement - Timestamp: {}",
                    settlement.timestamp
                );

                if let Some(mark_price) = settlement.mark_price {
                    info!("      Final mark price: {:.2}", mark_price);
                }

                if let Some(index_price) = settlement.index_price {
                    info!("      Index price at delivery: {:.2}", index_price);
                }
            }
        }
        Err(e) => {
            info!("ℹ️ Expected result for inactive BTC future: {}", e);
            info!(
                "💡 This demonstrates proper error handling when instruments are not active in testnet"
            );
        }
    }

    // Test with invalid instrument to demonstrate error handling
    match client
        .get_last_settlements_by_instrument("INVALID-INSTRUMENT", None, Some(1), None, None)
        .await
    {
        Ok(settlements) => {
            if settlements.settlements.is_empty() {
                info!("✅ Empty result for invalid instrument (expected behavior)");
            } else {
                warn!(
                    "⚠️ Unexpected settlements found for invalid instrument: {}",
                    settlements.settlements.len()
                );
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid instrument: {}", e);
            info!("💡 This demonstrates proper error handling for invalid instrument names");
        }
    }
    println!();

    // =================================================================
    // SETTLEMENT TYPES EXPLANATION
    // =================================================================
    info!("📚 SETTLEMENT TYPES EXPLANATION");
    info!("===============================");
    info!("🏦 settlement - Regular periodic settlements for perpetual contracts");
    info!("📦 delivery - Final settlement at expiration for futures and options");
    info!("💸 bankruptcy - Settlements triggered by account liquidation events");
    info!("💡 Each type provides different data fields relevant to the settlement event");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED SETTLEMENT ENDPOINTS");
    info!("==========================================");
    info!("🏦 /public/get_last_settlements_by_currency - Settlement history filtered by currency");
    info!(
        "🎯 /public/get_last_settlements_by_instrument - Settlement history filtered by instrument"
    );
    println!();

    info!("🎉 Settlement endpoints example completed successfully!");
    info!("💡 Tip: Use these endpoints to track settlement history and profit/loss over time");
    info!("🔗 Settlement data is crucial for account reconciliation and performance analysis");
    info!("📊 Filter by settlement type (settlement/delivery/bankruptcy) for specific event types");

    Ok(())
}
