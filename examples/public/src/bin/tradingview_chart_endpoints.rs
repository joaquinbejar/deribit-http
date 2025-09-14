//! TradingView Chart Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoint:
//! - `/public/get_tradingview_chart_data` - Chart data for TradingView charts
//!
//! Usage: cargo run --bin tradingview_chart_endpoints

use deribit_base::prelude::setup_logger;
use deribit_http::DeribitHttpClient;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. GET TRADINGVIEW CHART DATA (/public/get_tradingview_chart_data)
    // =================================================================
    info!("📈 1. GET TRADINGVIEW CHART DATA");
    info!("--------------------------------");

    // Set up time range (last 24 hours)
    let end_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let start_timestamp = end_timestamp - (24 * 60 * 60 * 1000); // 24 hours ago

    info!(
        "🕐 Time range: {} to {} (last 24 hours)",
        start_timestamp, end_timestamp
    );

    // Test with BTC-PERPETUAL and 1-hour resolution
    match client
        .get_tradingview_chart_data("BTC-PERPETUAL", start_timestamp, end_timestamp, "60")
        .await
    {
        Ok(chart_data) => {
            info!("✅ Chart data for BTC-PERPETUAL (1h) retrieved successfully");
            info!("📊 Chart Status: {}", chart_data.status);
            info!("📈 Data Points: {} candles", chart_data.ticks.len());

            if !chart_data.ticks.is_empty() {
                info!("📊 OHLCV Data Summary:");
                info!(
                    "   🕐 Time Range: {} to {}",
                    chart_data.ticks.first().unwrap(),
                    chart_data.ticks.last().unwrap()
                );

                // Show first few candles
                let candles_to_show = 3.min(chart_data.ticks.len());
                for i in 0..candles_to_show {
                    let timestamp = chart_data.ticks[i];
                    let open = chart_data.open[i];
                    let high = chart_data.high[i];
                    let low = chart_data.low[i];
                    let close = chart_data.close[i];
                    let volume = chart_data.volume[i];

                    info!(
                        "   {}. Candle {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{:.6}",
                        i + 1,
                        timestamp,
                        open,
                        high,
                        low,
                        close,
                        volume
                    );
                }

                if chart_data.ticks.len() > candles_to_show {
                    info!(
                        "   💡 Showing first {} of {} candles",
                        candles_to_show,
                        chart_data.ticks.len()
                    );
                }

                // Calculate price range and movement
                if let (Some(&first_close), Some(&last_close)) =
                    (chart_data.close.first(), chart_data.close.last())
                {
                    let price_change = last_close - first_close;
                    let price_change_pct = (price_change / first_close) * 100.0;
                    let change_symbol = if price_change >= 0.0 { "📈" } else { "📉" };
                    info!(
                        "   {} Price Movement: ${:.2} ({:.2}%)",
                        change_symbol, price_change, price_change_pct
                    );
                }

                // Volume analysis
                let total_volume: f64 = chart_data.volume.iter().sum();
                let avg_volume = total_volume / chart_data.volume.len() as f64;
                info!("   📊 Volume Analysis:");
                info!("      Total Volume: {:.6} BTC", total_volume);
                info!("      Average Volume per Candle: {:.6} BTC", avg_volume);

                // High/Low analysis
                let highest_price = chart_data
                    .high
                    .iter()
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let lowest_price = chart_data.low.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                info!("   💰 Price Range:");
                info!("      Highest: ${:.2}", highest_price);
                info!("      Lowest: ${:.2}", lowest_price);
                info!(
                    "      Range: ${:.2} ({:.2}%)",
                    highest_price - lowest_price,
                    ((highest_price - lowest_price) / lowest_price) * 100.0
                );
            } else {
                info!("💡 No chart data available for this time range");
            }
        }
        Err(e) => {
            warn!("⚠️ Get chart data for BTC-PERPETUAL (1h) error: {}", e);
            info!("💡 This may be expected if no data is available for the specified time range");
        }
    }

    // Test with BTC-PERPETUAL and 15-minute resolution for more granular data
    match client
        .get_tradingview_chart_data("BTC-PERPETUAL", start_timestamp, end_timestamp, "15")
        .await
    {
        Ok(chart_data) => {
            info!("✅ Chart data for BTC-PERPETUAL (15m) retrieved successfully");
            info!("📊 15-minute Chart Status: {}", chart_data.status);
            info!(
                "📈 15-minute Data Points: {} candles",
                chart_data.ticks.len()
            );

            if !chart_data.ticks.is_empty() {
                // Show volatility analysis with more granular data
                let mut price_movements = Vec::new();
                for i in 1..chart_data.close.len() {
                    let prev_close = chart_data.close[i - 1];
                    let curr_close = chart_data.close[i];
                    let movement = ((curr_close - prev_close) / prev_close) * 100.0;
                    price_movements.push(movement.abs());
                }

                if !price_movements.is_empty() {
                    let avg_volatility =
                        price_movements.iter().sum::<f64>() / price_movements.len() as f64;
                    let max_move = price_movements.iter().fold(0.0f64, |a, &b| a.max(b));
                    info!("   📊 Volatility Analysis (15m):");
                    info!("      Average Candle Movement: {:.4}%", avg_volatility);
                    info!("      Maximum Single Candle Move: {:.4}%", max_move);
                }
            }
        }
        Err(e) => {
            warn!("⚠️ Get chart data for BTC-PERPETUAL (15m) error: {}", e);
            info!("💡 This may be expected if 15-minute data is not available");
        }
    }

    // Test with ETH-PERPETUAL and 30-minute resolution
    match client
        .get_tradingview_chart_data("ETH-PERPETUAL", start_timestamp, end_timestamp, "30")
        .await
    {
        Ok(chart_data) => {
            info!("✅ Chart data for ETH-PERPETUAL (30m) retrieved successfully");
            info!("📊 ETH Chart Status: {}", chart_data.status);
            info!("📈 ETH Data Points: {} candles", chart_data.ticks.len());

            if !chart_data.ticks.is_empty() {
                // Compare first and last prices
                if let (Some(&first_close), Some(&last_close)) =
                    (chart_data.close.first(), chart_data.close.last())
                {
                    let eth_change = last_close - first_close;
                    let eth_change_pct = (eth_change / first_close) * 100.0;
                    let eth_symbol = if eth_change >= 0.0 { "📈" } else { "📉" };
                    info!(
                        "   {} ETH Performance: ${:.2} ({:.2}%)",
                        eth_symbol, eth_change, eth_change_pct
                    );
                }

                // ETH volume analysis
                let eth_total_volume: f64 = chart_data.volume.iter().sum();
                info!("   📊 ETH Total Volume: {:.6} ETH", eth_total_volume);
            }
        }
        Err(e) => {
            warn!("⚠️ Get chart data for ETH-PERPETUAL (30m) error: {}", e);
            info!("💡 This may be expected if ETH-PERPETUAL data is limited on testnet");
        }
    }

    // Test with shorter time range (last 4 hours) and higher resolution (5 minutes)
    let short_start_timestamp = end_timestamp - (4 * 60 * 60 * 1000); // 4 hours ago
    info!(
        "🕐 Short time range: {} to {} (last 4 hours)",
        short_start_timestamp, end_timestamp
    );

    match client
        .get_tradingview_chart_data("BTC-PERPETUAL", short_start_timestamp, end_timestamp, "5")
        .await
    {
        Ok(chart_data) => {
            info!("✅ Short-term chart data for BTC-PERPETUAL (5m) retrieved successfully");
            info!("📊 Short-term Chart Status: {}", chart_data.status);
            info!(
                "📈 Short-term Data Points: {} candles",
                chart_data.ticks.len()
            );

            if !chart_data.ticks.is_empty() {
                info!("   💡 High-frequency data useful for scalping and short-term analysis");

                // Show recent trend
                let recent_candles = 5.min(chart_data.close.len());
                if recent_candles > 1 {
                    info!(
                        "   📈 Recent Price Action (last {} candles):",
                        recent_candles
                    );
                    for i in (chart_data.close.len() - recent_candles)..chart_data.close.len() {
                        let timestamp = chart_data.ticks[i];
                        let close = chart_data.close[i];
                        info!("      Candle {} - Close: ${:.2}", timestamp, close);
                    }
                }
            }
        }
        Err(e) => {
            warn!(
                "⚠️ Get short-term chart data for BTC-PERPETUAL (5m) error: {}",
                e
            );
            info!("💡 This may be expected if high-frequency data is rate-limited");
        }
    }

    // Test with different resolutions to demonstrate various timeframes
    let resolutions = vec![
        ("1", "1 minute"),
        ("3", "3 minutes"),
        ("10", "10 minutes"),
        ("60", "1 hour"),
    ];
    let test_start = end_timestamp - (2 * 60 * 60 * 1000); // 2 hours ago

    info!("🔄 Testing different chart resolutions:");
    for (resolution, description) in &resolutions {
        match client
            .get_tradingview_chart_data("BTC-PERPETUAL", test_start, end_timestamp, resolution)
            .await
        {
            Ok(chart_data) => {
                info!(
                    "✅ {} resolution: {} candles (status: {})",
                    description,
                    chart_data.ticks.len(),
                    chart_data.status
                );
            }
            Err(e) => {
                info!("⚠️ {} resolution failed: {}", description, e);
            }
        }
    }

    // Test with a BTC future (if available)
    match client
        .get_tradingview_chart_data("BTC-29MAR24", start_timestamp, end_timestamp, "60")
        .await
    {
        Ok(chart_data) => {
            info!("✅ Chart data for BTC future retrieved successfully");
            info!("📊 Future Chart Status: {}", chart_data.status);
            info!("📈 Future Data Points: {} candles", chart_data.ticks.len());

            if !chart_data.ticks.is_empty() {
                info!("   💡 Future contract chart data useful for basis trading analysis");
            }
        }
        Err(e) => {
            warn!("⚠️ Get chart data for BTC future error: {}", e);
            info!("💡 This is expected if the specific future is not available");
        }
    }

    // Test with invalid instrument to demonstrate error handling
    match client
        .get_tradingview_chart_data("INVALID-INSTRUMENT", start_timestamp, end_timestamp, "60")
        .await
    {
        Ok(chart_data) => {
            if chart_data.ticks.is_empty() {
                info!("✅ Empty chart data for invalid instrument (expected behavior)");
            } else {
                warn!(
                    "⚠️ Unexpected chart data found for invalid instrument: {} candles",
                    chart_data.ticks.len()
                );
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid instrument: {}", e);
            info!("💡 This demonstrates proper error handling for invalid instrument names");
        }
    }

    // Test with invalid resolution
    match client
        .get_tradingview_chart_data("BTC-PERPETUAL", start_timestamp, end_timestamp, "7")
        .await
    {
        Ok(chart_data) => {
            if chart_data.status == "ok" {
                info!(
                    "✅ Unusual resolution accepted: {} candles",
                    chart_data.ticks.len()
                );
            } else {
                info!(
                    "⚠️ Unusual resolution returned status: {}",
                    chart_data.status
                );
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid resolution: {}", e);
            info!("💡 Valid resolutions: 1, 3, 5, 10, 15, 30, 60, 120, 180, 360 minutes");
        }
    }
    println!();

    // =================================================================
    // CHART DATA EXPLANATION
    // =================================================================
    info!("📚 CHART DATA EXPLANATION");
    info!("==========================");
    info!("📊 Status: Indicates if the request was successful ('ok' or error message)");
    info!("🕐 Ticks: Array of timestamps for each candle (milliseconds since epoch)");
    info!("💰 Open: Opening price for each time period");
    info!("⬆️ High: Highest price reached during each time period");
    info!("⬇️ Low: Lowest price reached during each time period");
    info!("🎯 Close: Closing price for each time period");
    info!("📦 Volume: Trading volume during each time period");
    info!(
        "⏰ Resolution: Time interval for each candle (1, 3, 5, 10, 15, 30, 60, 120, 180, 360 minutes)"
    );
    println!();

    // =================================================================
    // TRADING INSIGHTS
    // =================================================================
    info!("🧠 TRADING INSIGHTS FROM CHART DATA");
    info!("====================================");
    info!("📈 OHLCV data is essential for technical analysis and pattern recognition");
    info!("⏰ Different resolutions provide insights at various time scales");
    info!("📊 Volume analysis helps identify strength of price movements");
    info!("🎯 High/low ranges indicate volatility and potential support/resistance levels");
    info!("📉 Price movements between candles show market momentum");
    info!("🔄 Multiple timeframe analysis provides comprehensive market view");
    info!("💡 Chart data enables backtesting of trading strategies");
    println!();

    // =================================================================
    // RESOLUTION GUIDE
    // =================================================================
    info!("⏰ CHART RESOLUTION GUIDE");
    info!("==========================");
    info!("🔥 1m, 3m, 5m - Scalping and high-frequency trading");
    info!("⚡ 15m, 30m - Short-term day trading strategies");
    info!("📊 1h (60m) - Intraday analysis and swing trading setups");
    info!("📈 2h (120m), 3h (180m) - Medium-term trend analysis");
    info!("🎯 6h (360m) - Long-term position analysis");
    info!("💡 Higher resolutions provide more data points but may hit rate limits");
    info!("🔄 Lower resolutions are better for long-term trend identification");
    println!();

    // =================================================================
    // TIME RANGE RECOMMENDATIONS
    // =================================================================
    info!("🕐 TIME RANGE RECOMMENDATIONS");
    info!("==============================");
    info!("⚡ 1-4 hours: Perfect for high-resolution analysis (1m, 3m, 5m)");
    info!("📊 4-24 hours: Ideal for medium resolution (15m, 30m, 1h)");
    info!("📈 1-7 days: Good for hourly and multi-hour analysis");
    info!("🎯 1+ weeks: Best for daily and higher timeframes");
    info!("💡 Longer ranges with high resolution may exceed API limits");
    info!("🔄 Balance time range and resolution based on analysis needs");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED CHART ENDPOINT");
    info!("====================================");
    info!("📈 /public/get_tradingview_chart_data - OHLCV candle data for technical analysis");
    println!();

    info!("🎉 TradingView chart endpoints example completed successfully!");
    info!("💡 Tip: Use chart data for technical analysis, backtesting, and strategy development");
    info!("🔗 OHLCV data is fundamental for algorithmic trading and market analysis");
    info!("📊 Combine multiple timeframes for comprehensive market understanding");
    info!("⏰ Choose appropriate resolution based on your trading or analysis timeframe");

    Ok(())
}
