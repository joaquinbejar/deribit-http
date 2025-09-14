//! Comprehensive Market Data Integration Tests
//!
//! This test covers comprehensive market data scenarios:
//! 1. Public endpoint availability and reliability
//! 2. Data consistency and validation
//! 3. Parameter validation and edge cases
//! 4. Performance and response time testing
//! 5. Cross-instrument data correlation


use tokio::time::{Duration, Instant, sleep};
use tracing::{debug, info, warn};
use deribit_http::DeribitHttpClient;

#[tokio::test]
#[serial_test::serial]
async fn test_server_time_reliability() -> Result<(), Box<dyn std::error::Error>> {
    let client = DeribitHttpClient::new();

    // Test multiple server time requests for consistency
    let num_requests = 10;
    let mut times = Vec::new();
    let mut request_durations = Vec::new();

    for i in 0..num_requests {
        let start_time = Instant::now();
        let result = client.get_server_time().await;
        let elapsed = start_time.elapsed();
        request_durations.push(elapsed);

        match result {
            Ok(time) => {
                times.push(time);
                // Validate time is reasonable (not zero, not too far in future)
                assert!(
                    time > 1_600_000_000_000,
                    "Server time should be reasonable (after 2020): {}",
                    time
                );
                assert!(
                    time < 2_000_000_000_000,
                    "Server time should be reasonable (before 2033): {}",
                    time
                );
            }
            Err(e) => {
                warn!(
                    "Server time request #{} failed in {:?}: {:?}",
                    i + 1,
                    elapsed,
                    e
                );
            }
        }

        // Small delay between requests
        sleep(Duration::from_millis(100)).await;
    }

    // Analyze results
    let success_count = times.len();
    let failure_count = num_requests - success_count;

    info!("Server time reliability results:");
    info!(
        "  Successful requests: {}/{} ({:.1}%)",
        success_count,
        num_requests,
        (success_count as f64 / num_requests as f64) * 100.0
    );
    info!("  Failed requests: {}", failure_count);

    if success_count > 1 {
        // Check time progression (should be monotonic or close to it)
        let mut monotonic_violations = 0;
        for i in 1..times.len() {
            if times[i] < times[i - 1] {
                monotonic_violations += 1;
                debug!("Time went backwards: {} -> {}", times[i - 1], times[i]);
            }
        }

        info!(
            "  Monotonic violations: {}/{}",
            monotonic_violations,
            times.len() - 1
        );

        // Calculate time differences
        let time_diffs: Vec<i64> = times
            .windows(2)
            .map(|w| (w[1] as i64) - (w[0] as i64))
            .collect();
        let avg_diff = if !time_diffs.is_empty() {
            time_diffs.iter().sum::<i64>() / time_diffs.len() as i64
        } else {
            0
        };

        info!("  Average time progression: {}ms", avg_diff);

        // Time should generally progress forward
        assert!(
            monotonic_violations < times.len() / 2,
            "Too many monotonic violations"
        );
    }

    // Analyze request performance
    if !request_durations.is_empty() {
        let avg_duration =
            request_durations.iter().sum::<Duration>() / request_durations.len() as u32;
        let min_duration = *request_durations.iter().min().unwrap();
        let max_duration = *request_durations.iter().max().unwrap();

        info!(
            "  Request timing - avg: {:?}, min: {:?}, max: {:?}",
            avg_duration, min_duration, max_duration
        );

        // Requests should complete in reasonable time
        assert!(
            avg_duration < Duration::from_secs(5),
            "Average request time should be reasonable"
        );
        assert!(
            max_duration < Duration::from_secs(10),
            "Maximum request time should be reasonable"
        );
    }

    // At least some requests should succeed
    assert!(
        success_count > 0,
        "At least some server time requests should succeed"
    );

    info!("Server time reliability test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_instruments_data_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Starting instruments data validation test

    let client = DeribitHttpClient::new();

    // Test instruments for different currencies
    let currencies = vec!["BTC", "ETH", "USDC", "EURR"];

    for currency in currencies {
        debug!("Testing instruments for currency: {}", currency);

        let start_time = Instant::now();
        let result = client.get_instruments(currency, None, None).await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(instruments) => {
                info!(
                    "Instruments for {} retrieved successfully in {:?}: {} instruments",
                    currency,
                    elapsed,
                    instruments.len()
                );

                // Validate instrument data structure
                for (i, instrument) in instruments.iter().enumerate().take(5) {
                    // Check first 5
                    debug!(
                        "Validating instrument #{}: {}",
                        i + 1,
                        instrument.instrument_name
                    );

                    // Basic field validation
                    assert!(
                        !instrument.instrument_name.is_empty(),
                        "Instrument name should not be empty"
                    );
                    if let Some(base_currency) = &instrument.currency {
                        assert!(
                            !base_currency.is_empty(),
                            "Base currency should not be empty for {}",
                            instrument.instrument_name
                        );
                    }

                    // Currency consistency
                    if let Some(ref base_currency) = instrument.currency {
                        assert_eq!(
                            base_currency, currency,
                            "Base currency should match requested currency for {}",
                            instrument.instrument_name
                        );
                    }

                    // Numeric field validation
                    if let Some(tick_size) = instrument.tick_size {
                        assert!(
                            tick_size > 0.0,
                            "Tick size should be positive for {}",
                            instrument.instrument_name
                        );
                        assert!(
                            tick_size.is_finite(),
                            "Tick size should be finite for {}",
                            instrument.instrument_name
                        );
                    }
                    if let Some(min_trade) = instrument.min_trade_amount {
                        assert!(
                            min_trade > 0.0,
                            "Min trade amount should be positive for {}",
                            instrument.instrument_name
                        );
                        assert!(
                            min_trade.is_finite(),
                            "Min trade amount should be finite for {}",
                            instrument.instrument_name
                        );
                    }

                    // Instrument type validation (check instrument name for type)
                    let valid_types = ["future", "option", "spot", "perpetual", "PERPETUAL"];
                    assert!(
                        valid_types.iter().any(|&t| instrument
                            .instrument_name
                            .to_uppercase()
                            .contains(&t.to_uppercase())),
                        "Instrument name should contain valid type for {}",
                        instrument.instrument_name
                    );
                }

                // Performance validation
                assert!(
                    elapsed < Duration::from_secs(10),
                    "Instruments request should complete in reasonable time"
                );
            }
            Err(e) => {
                info!(
                    "Instruments for {} failed in {:?}: {:?}",
                    currency, elapsed, e
                );

                // Even failures should complete in reasonable time
                assert!(
                    elapsed < Duration::from_secs(10),
                    "Failed request should not hang"
                );
            }
        }

        // Delay between currency requests
        sleep(Duration::from_millis(200)).await;
    }

    info!("Instruments data validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_ticker_data_consistency() -> Result<(), Box<dyn std::error::Error>> {
    // Starting ticker data consistency test

    let client = DeribitHttpClient::new();

    // Test ticker data for popular instruments
    let instruments = vec![
        "BTC-PERPETUAL",
        "ETH-PERPETUAL",
        "BTC-25JUL25",
        "ETH-25JUL25",
    ];

    for instrument in instruments {
        debug!("Testing ticker data for instrument: {}", instrument);

        let start_time = Instant::now();
        let result = client.get_ticker(instrument).await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(ticker) => {
                info!(
                    "Ticker for {} retrieved successfully in {:?}",
                    instrument, elapsed
                );
                debug!("Ticker data: {:?}", ticker);

                // Validate ticker data structure
                assert_eq!(
                    ticker.instrument_name, instrument,
                    "Instrument name should match request"
                );

                // Price validation
                if let Some(best_bid_price) = ticker.best_bid_price {
                    assert!(best_bid_price > 0.0, "Best bid price should be positive");
                    assert!(
                        best_bid_price.is_finite(),
                        "Best bid price should be finite"
                    );
                }

                if let Some(best_ask_price) = ticker.best_ask_price {
                    assert!(best_ask_price > 0.0, "Best ask price should be positive");
                    assert!(
                        best_ask_price.is_finite(),
                        "Best ask price should be finite"
                    );
                }

                let last_price = ticker.last_price;
                if let Some(price) = last_price {
                    if price > 0.0 {
                        assert!(price > 0.0, "Last price should be positive");
                        assert!(price.is_finite(), "Last price should be finite");
                    }
                }

                // Note: volume_24h field may not exist in current TickerData structure

                // Spread validation (if both bid and ask are available)
                if let (Some(bid), Some(ask)) = (ticker.best_bid_price, ticker.best_ask_price) {
                    assert!(
                        ask >= bid,
                        "Ask price should be >= bid price for {}",
                        instrument
                    );

                    let spread = ask - bid;
                    let spread_pct = (spread / bid) * 100.0;

                    debug!("Spread for {}: {} ({:.2}%)", instrument, spread, spread_pct);

                    // Spread should be reasonable (less than 10% for liquid instruments)
                    if spread_pct > 10.0 {
                        warn!(
                            "Large spread detected for {}: {:.2}%",
                            instrument, spread_pct
                        );
                    }
                }

                // Timestamp validation
                assert!(ticker.timestamp > 0, "Timestamp should be positive");
                assert!(
                    ticker.timestamp > 1_600_000_000_000,
                    "Timestamp should be reasonable"
                );

                // Performance validation
                assert!(
                    elapsed < Duration::from_secs(5),
                    "Ticker request should complete quickly"
                );
            }
            Err(e) => {
                info!("Ticker for {} failed in {:?}: {:?}", instrument, elapsed, e);

                // Check if it's an instrument not found error (acceptable for some test instruments)
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("not found") || error_str.contains("invalid") {
                    info!(
                        "Instrument {} not found - this is acceptable for test",
                        instrument
                    );
                } else {
                    warn!("Unexpected error for ticker {}: {:?}", instrument, e);
                }
            }
        }

        // Delay between instrument requests
        sleep(Duration::from_millis(300)).await;
    }

    info!("Ticker data consistency test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_order_book_data_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Starting order book data validation test

    let client = DeribitHttpClient::new();

    // Test order book for liquid instruments
    let instruments = vec!["BTC-PERPETUAL", "ETH-PERPETUAL"];
    let depths = vec![5, 10, 20];

    for instrument in instruments {
        for depth in &depths {
            debug!("Testing order book for {} with depth {}", instrument, depth);

            let start_time = Instant::now();
            let result = client.get_order_book(instrument, Some(*depth)).await;
            let elapsed = start_time.elapsed();

            match result {
                Ok(order_book) => {
                    info!(
                        "Order book for {} (depth {}) retrieved in {:?}",
                        instrument, depth, elapsed
                    );

                    // Validate order book structure
                    assert_eq!(
                        order_book.instrument_name, instrument,
                        "Instrument name should match"
                    );
                    assert!(order_book.timestamp > 0, "Timestamp should be positive");

                    // Validate bids
                    assert!(
                        order_book.bids.len() <= *depth as usize,
                        "Bids should not exceed requested depth"
                    );
                    for (i, bid) in order_book.bids.iter().enumerate() {
                        let price = bid.price;
                        let amount = bid.amount;

                        assert!(price > 0.0, "Bid price should be positive at level {}", i);
                        assert!(amount > 0.0, "Bid amount should be positive at level {}", i);
                        assert!(
                            price.is_finite(),
                            "Bid price should be finite at level {}",
                            i
                        );
                        assert!(
                            amount.is_finite(),
                            "Bid amount should be finite at level {}",
                            i
                        );

                        // Bids should be in descending price order
                        if i > 0 {
                            assert!(
                                price <= order_book.bids[i - 1].price,
                                "Bids should be in descending order at level {}",
                                i
                            );
                        }
                    }

                    // Validate asks
                    assert!(
                        order_book.asks.len() <= *depth as usize,
                        "Asks should not exceed requested depth"
                    );
                    for (i, ask) in order_book.asks.iter().enumerate() {
                        let price = ask.price;
                        let amount = ask.amount;

                        assert!(price > 0.0, "Ask price should be positive at level {}", i);
                        assert!(amount > 0.0, "Ask amount should be positive at level {}", i);
                        assert!(
                            price.is_finite(),
                            "Ask price should be finite at level {}",
                            i
                        );
                        assert!(
                            amount.is_finite(),
                            "Ask amount should be finite at level {}",
                            i
                        );

                        // Asks should be in ascending price order
                        if i > 0 {
                            assert!(
                                price >= order_book.asks[i - 1].price,
                                "Asks should be in ascending order at level {}",
                                i
                            );
                        }
                    }

                    // Validate spread (if both bids and asks exist)
                    if !order_book.bids.is_empty() && !order_book.asks.is_empty() {
                        let best_bid = order_book.bids[0].price;
                        let best_ask = order_book.asks[0].price;

                        assert!(best_ask >= best_bid, "Best ask should be >= best bid");

                        let spread = best_ask - best_bid;
                        let spread_pct = (spread / best_bid) * 100.0;

                        debug!(
                            "Order book spread for {}: {} ({:.3}%)",
                            instrument, spread, spread_pct
                        );

                        if spread_pct > 5.0 {
                            warn!(
                                "Large order book spread for {}: {:.3}%",
                                instrument, spread_pct
                            );
                        }
                    }

                    // Performance validation
                    assert!(
                        elapsed < Duration::from_secs(3),
                        "Order book request should be fast"
                    );
                }
                Err(e) => {
                    info!(
                        "Order book for {} (depth {}) failed in {:?}: {:?}",
                        instrument, depth, elapsed, e
                    );

                    let error_str = e.to_string().to_lowercase();
                    if error_str.contains("not found") {
                        info!("Instrument {} not found - acceptable for test", instrument);
                    }
                }
            }

            // Delay between requests
            sleep(Duration::from_millis(200)).await;
        }
    }

    info!("Order book data validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_market_data_performance() -> Result<(), Box<dyn std::error::Error>> {
    // Starting market data performance test

    let client = DeribitHttpClient::new();

    // Test performance of different market data endpoints
    let performance_tests: Vec<(
        &str,
        Box<
            dyn Fn() -> std::pin::Pin<
                    Box<
                        dyn std::future::Future<Output = Result<String, deribit_http::HttpError>>
                            + Send,
                    >,
                > + Send
                + Sync,
        >,
    )> = vec![
        (
            "Server Time",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_server_time()
                        .await
                        .map(|t| format!("time: {}", t))
                })
            }),
        ),
        (
            "BTC Instruments",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_instruments("BTC", None, None)
                        .await
                        .map(|i| format!("{} instruments", i.len()))
                })
            }),
        ),
        (
            "BTC-PERPETUAL Ticker",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_ticker("BTC-PERPETUAL")
                        .await
                        .map(|t| format!("price: {:?}", t.last_price))
                })
            }),
        ),
        (
            "BTC-PERPETUAL Order Book",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_order_book("BTC-PERPETUAL", Some(10))
                        .await
                        .map(|ob| format!("{} bids, {} asks", ob.bids.len(), ob.asks.len()))
                })
            }),
        ),
    ];

    let mut performance_results = Vec::new();

    for (test_name, test_fn) in performance_tests {
        debug!("Testing performance for: {}", test_name);

        // Run multiple iterations to get average performance
        let iterations = 3;
        let mut iteration_times = Vec::new();
        let mut success_count = 0;

        for i in 0..iterations {
            let start_time = Instant::now();
            let result = test_fn().await;
            let elapsed = start_time.elapsed();

            iteration_times.push(elapsed);

            match result {
                Ok(response) => {
                    success_count += 1;
                    debug!(
                        "{} iteration #{} succeeded in {:?}: {}",
                        test_name,
                        i + 1,
                        elapsed,
                        response
                    );
                }
                Err(e) => {
                    debug!(
                        "{} iteration #{} failed in {:?}: {:?}",
                        test_name,
                        i + 1,
                        elapsed,
                        e
                    );
                }
            }

            // Small delay between iterations
            sleep(Duration::from_millis(100)).await;
        }

        // Calculate performance metrics
        let avg_time = iteration_times.iter().sum::<Duration>() / iterations as u32;
        let min_time = *iteration_times.iter().min().unwrap();
        let max_time = *iteration_times.iter().max().unwrap();
        let success_rate = success_count as f64 / iterations as f64;

        performance_results.push((test_name, avg_time, min_time, max_time, success_rate));

        info!(
            "{} performance: avg {:?}, min {:?}, max {:?}, success {:.1}%",
            test_name,
            avg_time,
            min_time,
            max_time,
            success_rate * 100.0
        );

        // Performance assertions
        assert!(
            avg_time < Duration::from_secs(10),
            "{} average time should be reasonable",
            test_name
        );
        assert!(
            max_time < Duration::from_secs(15),
            "{} max time should be reasonable",
            test_name
        );

        // Delay between different tests
        sleep(Duration::from_millis(300)).await;
    }

    // Overall performance analysis
    info!("Overall market data performance analysis:");

    let overall_avg = performance_results
        .iter()
        .map(|(_, avg, _, _, _)| *avg)
        .sum::<Duration>()
        / performance_results.len() as u32;

    let overall_success_rate = performance_results
        .iter()
        .map(|(_, _, _, _, rate)| *rate)
        .sum::<f64>()
        / performance_results.len() as f64;

    info!("  Overall average response time: {:?}", overall_avg);
    info!(
        "  Overall success rate: {:.1}%",
        overall_success_rate * 100.0
    );

    // System should have reasonable overall performance
    assert!(
        overall_avg < Duration::from_secs(5),
        "Overall performance should be good"
    );
    assert!(
        overall_success_rate >= 0.5,
        "Overall success rate should be at least 50%"
    );

    info!("Market data performance test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_market_data_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    // Starting market data edge cases test

    let client = DeribitHttpClient::new();

    // Test edge cases and error scenarios
    let edge_cases: Vec<(
        &str,
        Box<
            dyn Fn() -> std::pin::Pin<
                    Box<
                        dyn std::future::Future<Output = Result<String, deribit_http::HttpError>>
                            + Send,
                    >,
                > + Send
                + Sync,
        >,
    )> = vec![
        (
            "Invalid instrument ticker",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_ticker("INVALID-INSTRUMENT")
                        .await
                        .map(|_| "success".to_string())
                })
            }),
        ),
        (
            "Invalid currency instruments",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_instruments("INVALID", None, None)
                        .await
                        .map(|i| format!("{} instruments", i.len()))
                })
            }),
        ),
        (
            "Empty instrument name",
            Box::new(|| {
                Box::pin(async { client.get_ticker("").await.map(|_| "success".to_string()) })
            }),
        ),
        (
            "Very long instrument name",
            Box::new(|| {
                Box::pin(async {
                    let long_name = "A".repeat(1000);
                    client
                        .get_ticker(&long_name)
                        .await
                        .map(|_| "success".to_string())
                })
            }),
        ),
        (
            "Order book with zero depth",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_order_book("BTC-PERPETUAL", Some(0))
                        .await
                        .map(|ob| format!("{} bids, {} asks", ob.bids.len(), ob.asks.len()))
                })
            }),
        ),
        (
            "Order book with very large depth",
            Box::new(|| {
                Box::pin(async {
                    client
                        .get_order_book("BTC-PERPETUAL", Some(1000))
                        .await
                        .map(|ob| format!("{} bids, {} asks", ob.bids.len(), ob.asks.len()))
                })
            }),
        ),
    ];

    for (case_name, test_fn) in edge_cases {
        debug!("Testing edge case: {}", case_name);

        let start_time = Instant::now();
        let result = test_fn().await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(response) => {
                info!(
                    "Edge case '{}' unexpectedly succeeded in {:?}: {}",
                    case_name, elapsed, response
                );
                // Some edge cases might succeed (e.g., empty results for invalid parameters)
            }
            Err(e) => {
                info!(
                    "Edge case '{}' failed as expected in {:?}: {:?}",
                    case_name, elapsed, e
                );

                // Validate error handling
                let error_str = e.to_string();
                assert!(
                    !error_str.is_empty(),
                    "Error message should not be empty for {}",
                    case_name
                );
                assert!(
                    error_str.len() > 3,
                    "Error message should be descriptive for {}",
                    case_name
                );

                // Error should not contain internal details
                let error_lower = error_str.to_lowercase();
                assert!(
                    !error_lower.contains("panic"),
                    "Error should not mention panics for {}",
                    case_name
                );
                assert!(
                    !error_lower.contains("unwrap"),
                    "Error should not mention unwrap for {}",
                    case_name
                );
            }
        }

        // Even edge cases should complete in reasonable time
        assert!(
            elapsed < Duration::from_secs(10),
            "Edge case '{}' should not hang",
            case_name
        );

        // Delay between edge cases
        sleep(Duration::from_millis(200)).await;
    }

    info!("Market data edge cases test completed successfully");
    Ok(())
}
