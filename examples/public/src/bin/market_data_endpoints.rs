//! Market Data Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public market data endpoints:
//! - `/public/get_apr_history` - APR history for yield tokens (USDE, STETH)
//! - `/public/get_book_summary_by_currency` - Book summary for all instruments by currency
//! - `/public/get_book_summary_by_instrument` - Book summary for specific instrument
//! - `/public/get_contract_size` - Contract size for instrument
//! - `/public/get_currencies` - All supported cryptocurrencies
//!
//! Usage: cargo run --bin market_data_endpoints

use deribit_http::prelude::*;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();

    info!("🚀 Deribit HTTP Client - Market Data Endpoints Example");
    info!("=====================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. GET ALL CURRENCIES (/public/get_currencies)
    // =================================================================
    info!("💰 1. GET ALL SUPPORTED CURRENCIES");
    info!("----------------------------------");

    match client.get_currencies().await {
        Ok(currencies) => {
            info!("✅ Currencies retrieved successfully");
            info!("📊 Found {} supported currencies:", currencies.len());

            for currency in &currencies {
                info!(
                    "   • {} ({}) - Fee precision: {}",
                    currency.currency,
                    currency.currency_long,
                    currency
                        .fee_precision
                        .map_or("N/A".to_string(), |v| v.to_string())
                );

                if let Some(apr) = currency.apr {
                    info!("     📈 APR: {}% (yield-generating token)", apr);
                }
            }

            if currencies.len() > 10 {
                info!(
                    "💡 Showing details for first {} currencies, {} total found",
                    std::cmp::min(currencies.len(), 5),
                    currencies.len()
                );
            }
        }
        Err(e) => {
            error!("❌ Get currencies error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 2. GET APR HISTORY (/public/get_apr_history)
    // =================================================================
    info!("📈 2. GET APR HISTORY FOR YIELD TOKENS");
    info!("--------------------------------------");

    // Test with STETH (Staked Ethereum)
    match client.get_apr_history("steth", Some(5), None).await {
        Ok(apr_history) => {
            info!("✅ APR history for STETH retrieved successfully");
            info!("📊 Found {} data points:", apr_history.data.len());

            for data_point in &apr_history.data {
                info!("   • Day {}: APR {}%", data_point.day, data_point.apr);
            }

            if let Some(continuation) = apr_history.continuation {
                info!("🔗 Continuation token available: {}", continuation);
            }
        }
        Err(e) => {
            warn!("⚠️ Get APR history for STETH error: {}", e);
            info!("💡 This may be expected if STETH is not available on testnet");
        }
    }

    // Also test with USDE if available
    match client.get_apr_history("usde", Some(3), None).await {
        Ok(apr_history) => {
            info!("✅ APR history for USDE retrieved successfully");
            info!("📊 Found {} data points for USDE", apr_history.data.len());
        }
        Err(e) => {
            warn!("⚠️ Get APR history for USDE error: {}", e);
            info!("💡 This may be expected if USDE is not available on testnet");
        }
    }
    println!();

    // =================================================================
    // 3. GET BOOK SUMMARY BY CURRENCY (/public/get_book_summary_by_currency)
    // =================================================================
    info!("📊 3. GET BOOK SUMMARY BY CURRENCY");
    info!("----------------------------------");

    match client
        .get_book_summary_by_currency("BTC", Some("future"))
        .await
    {
        Ok(book_summaries) => {
            info!("✅ Book summary for BTC futures retrieved successfully");
            info!("📊 Found {} BTC future instruments:", book_summaries.len());

            for (i, summary) in book_summaries.iter().take(3).enumerate() {
                info!(
                    "   {}. {} - Volume: {} {}",
                    i + 1,
                    summary.instrument_name,
                    summary.volume,
                    summary.base_currency
                );
                info!(
                    "      Mark Price: {} | Open Interest: {}",
                    summary.mark_price, summary.open_interest
                );

                info!("      Volume USD: ${:.2}", summary.volume_usd);
            }

            if book_summaries.len() > 3 {
                info!("💡 Showing first 3 of {} BTC futures", book_summaries.len());
            }
        }
        Err(e) => {
            error!("❌ Get book summary by currency error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 4. GET BOOK SUMMARY BY INSTRUMENT (/public/get_book_summary_by_instrument)
    // =================================================================
    info!("🎯 4. GET BOOK SUMMARY BY INSTRUMENT");
    info!("-----------------------------------");

    match client.get_book_summary_by_instrument("BTC-PERPETUAL").await {
        Ok(summary) => {
            info!("✅ Book summary for BTC-PERPETUAL retrieved successfully");
            info!("📊 Instrument: {}", summary.instrument_name);
            info!("   💰 Base Currency: {}", summary.base_currency);
            info!("   💱 Quote Currency: {}", summary.quote_currency);
            info!("   📈 Mark Price: {}", summary.mark_price);
            info!("   📊 Volume: {}", summary.volume);
            info!("   🏦 Open Interest: {}", summary.open_interest);

            info!("      Volume USD: ${:.2}", summary.volume_usd);

            if let Some(funding_8h) = summary.funding_8h {
                info!("   📈 8h Funding Rate: {:.6}", funding_8h);
            }

            if let Some(current_funding) = summary.current_funding {
                info!("   📈 Current Funding: {:.6}", current_funding);
            }

            if let Some(price_change) = summary.price_change {
                info!("   📊 Price Change: {:.2}%", price_change);
            }
        }
        Err(e) => {
            error!("❌ Get book summary by instrument error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 5. GET CONTRACT SIZE (/public/get_contract_size)
    // =================================================================
    info!("📏 5. GET CONTRACT SIZE");
    info!("----------------------");

    // Test with BTC-PERPETUAL
    match client.get_contract_size("BTC-PERPETUAL").await {
        Ok(contract_size) => {
            info!("✅ Contract size for BTC-PERPETUAL retrieved successfully");
            info!("📏 Contract size: {} USD", contract_size);
            info!("💡 For futures, contract size is in USD");
        }
        Err(e) => {
            error!("❌ Get contract size for BTC-PERPETUAL error: {}", e);
        }
    }

    // Also test with an option if available
    match client.get_contract_size("BTC-29MAR24-60000-C").await {
        Ok(contract_size) => {
            info!("✅ Contract size for BTC option retrieved successfully");
            info!("📏 BTC Option contract size: {} BTC", contract_size);
            info!("💡 For options, contract size is in base currency (BTC)");
        }
        Err(e) => {
            warn!("⚠️ Get contract size for BTC option error: {}", e);
            info!("💡 This is expected if the specific option is not available");
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED MARKET DATA ENDPOINTS");
    info!("==========================================");
    info!("✅ /public/get_currencies - All supported cryptocurrencies");
    info!("📈 /public/get_apr_history - APR history for yield tokens (USDE, STETH)");
    info!("📊 /public/get_book_summary_by_currency - Book summary by currency");
    info!("🎯 /public/get_book_summary_by_instrument - Book summary by instrument");
    info!("📏 /public/get_contract_size - Contract size for instruments");
    println!();

    info!("🎉 Market data endpoints example completed successfully!");
    info!("💡 Tip: All endpoints provide comprehensive market information");
    info!("🔗 For real-time updates, consider using WebSocket subscriptions");

    Ok(())
}
