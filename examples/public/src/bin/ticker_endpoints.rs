//! Ticker Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoint:
//! - `/public/ticker` - Complete ticker data
//!
//! Usage: cargo run --bin ticker_endpoints

use deribit_http::prelude::*;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();

    info!("🚀 Deribit HTTP Client - Ticker Endpoints Example");
    info!("==================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. GET TICKER DATA (/public/ticker)
    // =================================================================
    info!("📊 1. GET TICKER DATA");
    info!("--------------------");

    // Test with BTC-PERPETUAL
    match client.get_ticker("BTC-PERPETUAL").await {
        Ok(ticker) => {
            info!("✅ Ticker data for BTC-PERPETUAL retrieved successfully");
            info!("📈 Market Data Summary:");

            // Current prices
            if let Some(last_price) = ticker.last_price {
                info!("   💰 Last Price: ${:.2}", last_price);
            } else {
                info!("   💰 Last Price: N/A");
            }

            info!("   🎯 Mark Price: ${:.2}", ticker.mark_price);
            if let Some(open_interest) = ticker.open_interest {
                info!("   📦 Open Interest: {:.6} BTC", open_interest);
            } else {
                info!("   📦 Open Interest: N/A (spot instrument)");
            }

            // Best bid/ask
            match (ticker.best_bid_price, ticker.best_ask_price) {
                (Some(bid_price), Some(ask_price)) => {
                    info!("   📈 Best Bid: ${:.2}", bid_price);
                    info!("   📉 Best Ask: ${:.2}", ask_price);

                    let spread = ask_price - bid_price;
                    let spread_percentage = (spread / bid_price) * 100.0;
                    info!("   📏 Spread: ${:.2} ({:.4}%)", spread, spread_percentage);

                    info!("   📦 Best Bid Amount: {:.6} BTC", ticker.best_bid_amount);
                    info!("   📦 Best Ask Amount: {:.6} BTC", ticker.best_ask_amount);
                }
                _ => {
                    info!("   📈 Best Bid: N/A");
                    info!("   📉 Best Ask: N/A");
                }
            }

            // 24h Statistics
            info!("   📊 24h Statistics:");
            info!("      📈 Volume: {:.6} BTC", ticker.stats.volume);

            if let Some(price_change) = ticker.stats.price_change {
                let change_symbol = if price_change >= 0.0 { "📈" } else { "📉" };
                info!("      {} Price Change: {:.2}%", change_symbol, price_change);
            } else {
                info!("      📊 Price Change: N/A");
            }

            if let Some(high) = ticker.stats.high {
                info!("      ⬆️  24h High: ${:.2}", high);
            } else {
                info!("      ⬆️  24h High: N/A");
            }

            if let Some(low) = ticker.stats.low {
                info!("      ⬇️  24h Low: ${:.2}", low);
            } else {
                info!("      ⬇️  24h Low: N/A");
            }

            // Market analysis
            if let (Some(high), Some(low)) = (ticker.stats.high, ticker.stats.low) {
                let range = high - low;
                let range_percentage = (range / low) * 100.0;
                info!(
                    "      📊 24h Range: ${:.2} ({:.2}%)",
                    range, range_percentage
                );

                if let Some(last_price) = ticker.last_price {
                    let position_in_range = (last_price - low) / range * 100.0;
                    info!(
                        "      🎯 Current Position in Range: {:.1}%",
                        position_in_range
                    );
                }
            }
        }
        Err(e) => {
            warn!("⚠️ Get ticker for BTC-PERPETUAL error: {}", e);
            info!("💡 This may be expected if the instrument is not available");
        }
    }

    // Test with ETH-PERPETUAL
    match client.get_ticker("ETH-PERPETUAL").await {
        Ok(ticker) => {
            info!("✅ Ticker data for ETH-PERPETUAL retrieved successfully");
            info!("📈 ETH Market Summary:");

            if let Some(last_price) = ticker.last_price {
                info!("   💰 ETH Last Price: ${:.2}", last_price);
            }

            info!("   🎯 ETH Mark Price: ${:.2}", ticker.mark_price);
            if let Some(open_interest) = ticker.open_interest {
                info!("   📦 ETH Open Interest: {:.6} ETH", open_interest);
            } else {
                info!("   📦 ETH Open Interest: N/A (spot instrument)");
            }

            if let (Some(bid_price), Some(ask_price)) =
                (ticker.best_bid_price, ticker.best_ask_price)
            {
                info!(
                    "   📈 ETH Best Bid: ${:.2} | 📉 Best Ask: ${:.2}",
                    bid_price, ask_price
                );
            }

            info!("   📊 ETH 24h Volume: {:.6} ETH", ticker.stats.volume);

            if let Some(price_change) = ticker.stats.price_change {
                let change_symbol = if price_change >= 0.0 { "📈" } else { "📉" };
                info!("   {} ETH 24h Change: {:.2}%", change_symbol, price_change);
            }
        }
        Err(e) => {
            warn!("⚠️ Get ticker for ETH-PERPETUAL error: {}", e);
            info!("💡 This may be expected if ETH-PERPETUAL is not available on testnet");
        }
    }

    // Test with available BTC futures (dynamically discovered)
    match client
        .get_instruments("BTC", Some("future"), Some(false))
        .await
    {
        Ok(instruments) => {
            if let Some(future_instrument) = instruments.first() {
                let future_name = &future_instrument.instrument_name;
                match client.get_ticker(future_name).await {
                    Ok(ticker) => {
                        info!(
                            "✅ Ticker data for BTC future {} retrieved successfully",
                            future_name
                        );
                        info!("📈 BTC Future Market:");

                        if let Some(last_price) = ticker.last_price {
                            info!("   💰 Future Last Price: ${:.2}", last_price);
                        }

                        info!("   🎯 Future Mark Price: ${:.2}", ticker.mark_price);
                        if let Some(open_interest) = ticker.open_interest {
                            info!("   📦 Future Open Interest: {:.6} BTC", open_interest);
                        } else {
                            info!("   📦 Future Open Interest: N/A");
                        }

                        if let Some(volume) = Some(ticker.stats.volume) {
                            info!("   📊 Future 24h Volume: {:.6} BTC", volume);
                        }

                        // Compare with perpetual if we had previous data
                        info!(
                            "   💡 Future contracts have expiration dates and may trade at premium/discount to spot"
                        );
                    }
                    Err(e) => {
                        info!("⚠️ Get ticker for BTC future {} error: {}", future_name, e);
                        info!("💡 This may be expected if the future is not currently tradeable");
                    }
                }
            } else {
                info!("💡 No BTC futures available for ticker testing");
            }
        }
        Err(e) => {
            info!("⚠️ Could not fetch BTC futures for ticker testing: {}", e);
            info!("💡 This may be expected on testnet with limited instrument availability");
        }
    }

    // Test with available spot instruments (dynamically discovered)
    match client
        .get_instruments("BTC", Some("spot"), Some(false))
        .await
    {
        Ok(instruments) => {
            if let Some(spot_instrument) = instruments.first() {
                let spot_name = &spot_instrument.instrument_name;
                match client.get_ticker(spot_name).await {
                    Ok(ticker) => {
                        info!(
                            "✅ Ticker data for BTC spot {} retrieved successfully",
                            spot_name
                        );
                        info!("📈 BTC Spot Market:");

                        if let Some(last_price) = ticker.last_price {
                            info!("   💰 Spot Last Price: ${:.2}", last_price);
                        }

                        info!("   🎯 Spot Mark Price: ${:.2}", ticker.mark_price);
                        info!("   📊 Spot 24h Volume: {:.6} BTC", ticker.stats.volume);

                        info!("   💡 Spot instruments represent direct cryptocurrency trading");
                    }
                    Err(e) => {
                        info!("⚠️ Get ticker for BTC spot {} error: {}", spot_name, e);
                        info!(
                            "💡 This may be expected if the spot instrument is not currently active"
                        );
                    }
                }
            } else {
                info!("💡 No BTC spot instruments available for ticker testing");
            }
        }
        Err(e) => {
            info!(
                "⚠️ Could not fetch BTC spot instruments for ticker testing: {}",
                e
            );
            info!("💡 This may be expected on testnet with limited instrument availability");
        }
    }

    // Test with available BTC options (dynamically discovered)
    match client
        .get_instruments("BTC", Some("option"), Some(false))
        .await
    {
        Ok(instruments) => {
            if let Some(option_instrument) = instruments.first() {
                let option_name = &option_instrument.instrument_name;
                match client.get_ticker(option_name).await {
                    Ok(ticker) => {
                        info!(
                            "✅ Ticker data for BTC option {} retrieved successfully",
                            option_name
                        );
                        info!("📈 BTC Option Market:");

                        if let Some(last_price) = ticker.last_price {
                            info!("   💰 Option Last Price: ${:.2}", last_price);
                        }

                        info!("   🎯 Option Mark Price: ${:.2}", ticker.mark_price);
                        if let Some(open_interest) = ticker.open_interest {
                            info!("   📦 Option Open Interest: {:.6}", open_interest);
                        } else {
                            info!("   📦 Option Open Interest: N/A");
                        }

                        if let Some(strike) = option_instrument.strike {
                            info!("   🎯 Strike Price: ${:.2}", strike);
                        }

                        if let Some(option_type) = &option_instrument.option_type {
                            info!("   📋 Option Type: {:?}", option_type);
                        }

                        info!("   💡 Options provide leverage and hedging opportunities");
                    }
                    Err(e) => {
                        info!("⚠️ Get ticker for BTC option {} error: {}", option_name, e);
                        info!("💡 This may be expected if the option is not currently active");
                    }
                }
            } else {
                info!("💡 No BTC options available for ticker testing");
            }
        }
        Err(e) => {
            info!("⚠️ Could not fetch BTC options for ticker testing: {}", e);
            info!("💡 This may be expected on testnet with limited options availability");
        }
    }

    // Test with invalid instrument to demonstrate error handling
    match client.get_ticker("INVALID-INSTRUMENT").await {
        Ok(ticker) => {
            warn!("⚠️ Unexpected ticker data found for invalid instrument");
            if let Some(last_price) = ticker.last_price {
                info!("   Received price: ${:.2}", last_price);
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid instrument: {}", e);
            info!("💡 This demonstrates proper error handling for invalid instrument names");
        }
    }
    println!();

    // =================================================================
    // TICKER DATA EXPLANATION
    // =================================================================
    info!("📚 TICKER DATA EXPLANATION");
    info!("===========================");
    info!("💰 Last Price: Most recent trade execution price");
    info!("🎯 Mark Price: Fair value price used for margin calculations");
    info!("📦 Open Interest: Total number of outstanding contracts");
    info!("📈 Best Bid: Highest price someone is willing to pay");
    info!("📉 Best Ask: Lowest price someone is willing to sell");
    info!("📏 Spread: Difference between best ask and best bid");
    info!("📊 Volume: Total trading volume in the last 24 hours");
    info!("📈📉 Price Change: Percentage change from 24 hours ago");
    info!("⬆️⬇️ High/Low: Highest and lowest prices in the last 24 hours");
    println!();

    // =================================================================
    // TRADING INSIGHTS
    // =================================================================
    info!("🧠 TRADING INSIGHTS FROM TICKER DATA");
    info!("=====================================");
    info!("💡 Mark price is used for liquidation and margin calculations");
    info!("📊 High volume indicates active trading and good liquidity");
    info!("📏 Tight spreads suggest liquid markets with low slippage");
    info!("📦 High open interest shows market participant engagement");
    info!("📈📉 Price change helps identify trending markets");
    info!("🎯 Last price vs mark price differences may indicate funding rates");
    info!("⚖️ Position in 24h range shows momentum and potential support/resistance");
    println!();

    // =================================================================
    // MARKET COMPARISON TIPS
    // =================================================================
    info!("🔄 MARKET COMPARISON TIPS");
    info!("==========================");
    info!("🏦 Compare perpetual vs future prices for basis trading opportunities");
    info!("💱 Monitor cross-currency pairs (BTC-USD vs BTC-USDC) for arbitrage");
    info!("📈 Use 24h statistics to identify volatile vs stable instruments");
    info!("🎯 Mark price deviations can indicate funding rate changes");
    info!("📊 Volume patterns help identify institutional vs retail activity");
    info!("⏰ Ticker data updates in real-time for active trading decisions");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED TICKER ENDPOINT");
    info!("=====================================");
    info!("📊 /public/ticker - Complete ticker data for any instrument");
    println!();

    info!("🎉 Ticker endpoints example completed successfully!");
    info!("💡 Tip: Use ticker data for market analysis, price discovery, and trading decisions");
    info!("🔗 Ticker data is essential for understanding current market conditions");
    info!("📊 Monitor multiple instruments to identify trading opportunities and market trends");
    info!("⚡ Real-time ticker data enables responsive trading strategies");

    Ok(())
}
