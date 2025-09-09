//! BTC Option Chain Example
//!
//! This example demonstrates how to download the complete option chain for BTC
//! with a specific expiry date (2025-09-10). It shows:
//! - Fetching all BTC options from Deribit
//! - Filtering by expiry date
//! - Sorting by strike price
//! - Displaying calls and puts in a formatted table
//! - Getting real-time pricing data
//!
//! Usage: cargo run --bin btc_option_chain_example

use deribit_base::prelude::*;
use deribit_http::DeribitHttpClient;
use std::env;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();
    info!("🚀 Deribit HTTP Client - BTC Option Chain Example");
    info!("=================================================");
    info!("📅 Target Expiry: 2025-09-10");
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
    // 1. FETCH ALL BTC OPTIONS WITH TICKER DATA
    // =================================================================
    info!("📋 1. FETCHING ALL BTC OPTIONS WITH TICKER DATA");
    info!("-----------------------------------------------");

    let all_btc_options = match client.get_options("BTC", "10SEP25").await {
        Ok(options) => {
            info!(
                "✅ Successfully fetched {} BTC options with ticker data",
                options.len()
            );
            options
        }
        Err(e) => {
            error!("❌ Failed to fetch BTC options: {}", e);
            return Err(e.into());
        }
    };

    // =================================================================
    // 2. FILTER BY EXPIRY DATE (2025-09-10)
    // =================================================================
    info!("🔍 2. FILTERING BY EXPIRY DATE");
    info!("------------------------------");

    // Convert target date to Deribit format (10SEP25)
    let target_expiry = "10SEP25";
    info!("🎯 Looking for options with expiry: {}", target_expiry);

    let option_chain: Vec<OptionInstrument> = all_btc_options
        .iter()
        .filter(|option| option.instrument.instrument_name.contains(target_expiry))
        .cloned()
        .collect();

    if option_chain.is_empty() {
        warn!("⚠️ No options found for expiry date {}", target_expiry);
        info!("💡 Available expiry dates:");

        // Show available expiry dates
        let mut expiry_dates: Vec<String> = Vec::new();
        for option in &all_btc_options {
            if let Some(expiry) = extract_expiry_from_name(&option.instrument.instrument_name) {
                if !expiry_dates.contains(&expiry) {
                    expiry_dates.push(expiry);
                }
            }
        }
        expiry_dates.sort();
        for (i, expiry) in expiry_dates.iter().take(10).enumerate() {
            info!("   {}. {}", i + 1, expiry);
        }
        if expiry_dates.len() > 10 {
            info!("   ... and {} more", expiry_dates.len() - 10);
        }

        return Ok(());
    }

    info!(
        "✅ Found {} options for expiry {}",
        option_chain.len(),
        target_expiry
    );

    // =================================================================
    // 3. PARSE AND SORT OPTIONS
    // =================================================================
    info!("📊 3. PARSING AND SORTING OPTIONS");
    info!("----------------------------------");

    let mut parsed_options: Vec<ParsedOptionWithTicker> = option_chain
        .iter()
        .filter_map(|option_instrument| parse_option_with_ticker(option_instrument))
        .collect();

    // Sort by strike price
    parsed_options.sort_by(|a, b| {
        a.strike
            .partial_cmp(&b.strike)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    info!("✅ Parsed and sorted {} options", parsed_options.len());

    // Separate calls and puts
    let calls: Vec<&ParsedOptionWithTicker> = parsed_options
        .iter()
        .filter(|opt| opt.option_type == OptionType::Call)
        .collect();

    let puts: Vec<&ParsedOptionWithTicker> = parsed_options
        .iter()
        .filter(|opt| opt.option_type == OptionType::Put)
        .collect();

    info!("📈 Calls: {}, 📉 Puts: {}", calls.len(), puts.len());
    println!();

    // =================================================================
    // 4. DISPLAY OPTION CHAIN TABLE
    // =================================================================
    info!("📋 4. BTC OPTION CHAIN - {}", target_expiry);
    info!("=====================================");
    println!();

    // Table header
    println!("┌─────────────┬──────────┬──────────┬─────────────┐");
    println!("│    CALLS    │  STRIKE  │   PUTS   │ INSTRUMENT  │");
    println!("├─────────────┼──────────┼──────────┼─────────────┤");

    // Get unique strikes
    let mut strikes: Vec<f64> = parsed_options.iter().map(|opt| opt.strike).collect();
    strikes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    strikes.dedup();

    for strike in &strikes {
        let call = calls.iter().find(|c| c.strike == *strike);
        let put = puts.iter().find(|p| p.strike == *strike);

        println!(
            "│{:^13}│{:^10}│{:^10}│{:^13}│",
            if call.is_some() { "✅" } else { "-" },
            format!("{:.0}", strike),
            if put.is_some() { "✅" } else { "-" },
            format!("{:.0}K", strike / 1000.0)
        );

        if call.is_some() || put.is_some() {
            if let Some(c) = call {
                println!(
                    "│ {:11} │          │          │             │",
                    &c.instrument_name[c.instrument_name.len().saturating_sub(11)..]
                );
            }
            if let Some(p) = put {
                println!(
                    "│             │          │ {:8} │             │",
                    &p.instrument_name[p.instrument_name.len().saturating_sub(8)..]
                );
            }
        }
    }

    println!("└─────────────┴──────────┴──────────┴─────────────┘");
    println!();

    // =================================================================
    // 5. DISPLAY OPTION PRICING DATA (ALREADY AVAILABLE)
    // =================================================================
    info!("💰 5. SAMPLE OPTION PRICING (FROM get_options)");
    info!("----------------------------------------------");

    // Display pricing for a few sample options (data already fetched with get_options)
    let sample_options: Vec<&ParsedOptionWithTicker> = parsed_options.iter().take(5).collect();

    for option in sample_options {
        info!(
            "📊 {} ({} {}K):",
            option.instrument_name,
            if option.option_type == OptionType::Call {
                "Call"
            } else {
                "Put"
            },
            option.strike / 1000.0
        );
        info!(
            "   💵 Last Price: {:.4}",
            option.ticker.last_price.unwrap_or(0.0)
        );
        info!(
            "   📈 Bid: {:.4}",
            option.ticker.best_bid_price.unwrap_or(0.0)
        );
        info!(
            "   📉 Ask: {:.4}",
            option.ticker.best_ask_price.unwrap_or(0.0)
        );
        if let Some(iv) = option.ticker.mark_iv {
            info!("   📊 IV: {:.2}%", iv * 100.0);
        }
        println!();
    }

    // =================================================================
    // 6. SUMMARY STATISTICS
    // =================================================================
    info!("📈 6. OPTION CHAIN SUMMARY");
    info!("--------------------------");

    if !strikes.is_empty() {
        let min_strike = strikes.first().unwrap();
        let max_strike = strikes.last().unwrap();
        let strike_range = max_strike - min_strike;

        info!(
            "📊 Strike Range: {:.0} - {:.0} ({:.0} range)",
            min_strike, max_strike, strike_range
        );
        info!("🎯 Total Strikes: {}", strikes.len());
        info!("📞 Call Options: {}", calls.len());
        info!("📉 Put Options: {}", puts.len());
        info!("📅 Expiry Date: {} (2025-09-10)", target_expiry);

        // Calculate days to expiry (approximate)
        let days_to_expiry = calculate_days_to_expiry("2025-09-10");
        info!("⏰ Days to Expiry: ~{}", days_to_expiry);
    }

    println!();
    info!("🎉 BTC Option Chain example completed successfully!");
    info!("💡 Tip: Use different expiry dates by modifying the target_expiry variable");
    info!(
        "📊 This data can be used for options analysis, volatility studies, and trading strategies"
    );

    Ok(())
}

// =================================================================
// HELPER STRUCTURES AND FUNCTIONS
// =================================================================

#[derive(Debug, Clone, PartialEq)]
enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ParsedOptionWithTicker {
    instrument_name: String,
    strike: f64,
    option_type: OptionType,
    expiry: String,
    ticker: TickerData,
}

fn parse_option_with_ticker(
    option_instrument: &OptionInstrument,
) -> Option<ParsedOptionWithTicker> {
    // Parse instrument name format: BTC-10SEP25-60000-C
    let parts: Vec<&str> = option_instrument
        .instrument
        .instrument_name
        .split('-')
        .collect();

    if parts.len() != 4 {
        return None;
    }

    let strike = parts[2].parse::<f64>().ok()?;
    let option_type = match parts[3] {
        "C" => OptionType::Call,
        "P" => OptionType::Put,
        _ => return None,
    };

    Some(ParsedOptionWithTicker {
        instrument_name: option_instrument.instrument.instrument_name.clone(),
        strike,
        option_type,
        expiry: parts[1].to_string(),
        ticker: option_instrument.ticker.clone(),
    })
}

fn extract_expiry_from_name(instrument_name: &str) -> Option<String> {
    let parts: Vec<&str> = instrument_name.split('-').collect();
    if parts.len() >= 2 {
        Some(parts[1].to_string())
    } else {
        None
    }
}

fn calculate_days_to_expiry(_target_date: &str) -> i64 {
    // Simple approximation - in a real implementation you'd use proper date parsing
    // For 2025-09-10, assuming current date is around 2025-01-01
    // This is just for demonstration
    250 // Approximate days to September 2025
}
