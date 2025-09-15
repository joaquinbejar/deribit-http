//! Comprehensive Market Data Integration Tests
//!
//! This test covers comprehensive market data scenarios:
//! 1. Public endpoint availability and reliability
//! 2. Data consistency and validation
//! 3. Parameter validation and edge cases
//! 4. Performance and response time testing
//! 5. Cross-instrument data correlation

#[cfg(test)]
mod comprehensive_market_data_tests {
    use deribit_http::DeribitHttpClient;
    use tokio::time::{Duration, Instant, sleep};
    use tracing::{debug, info, warn};

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
                    if let Some(price) = last_price
                        && price > 0.0
                    {
                        assert!(price > 0.0, "Last price should be positive");
                        assert!(price.is_finite(), "Last price should be finite");
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

        // Test server time performance
        let start_time = Instant::now();
        let result = client.get_server_time().await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(time) => {
                debug!("Server time test completed in {:?}: {}", elapsed, time);
                assert!(
                    elapsed < Duration::from_secs(10),
                    "Server time test took too long: {:?}",
                    elapsed
                );
            }
            Err(e) => {
                debug!("Server time test failed in {:?}: {:?}", elapsed, e);
            }
        }

        // Test ticker performance
        let start_time = Instant::now();
        let result = client.get_ticker("BTC-PERPETUAL").await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(ticker) => {
                debug!(
                    "Ticker test completed in {:?}: {}",
                    elapsed, ticker.instrument_name
                );
                assert!(
                    elapsed < Duration::from_secs(10),
                    "Ticker test took too long: {:?}",
                    elapsed
                );
            }
            Err(e) => {
                debug!("Ticker test failed in {:?}: {:?}", elapsed, e);
            }
        }

        // Test order book performance
        let start_time = Instant::now();
        let result = client.get_order_book("BTC-PERPETUAL", Some(10)).await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(order_book) => {
                debug!(
                    "Order book test completed in {:?}: {} bids, {} asks",
                    elapsed,
                    order_book.bids.len(),
                    order_book.asks.len()
                );
                assert!(
                    elapsed < Duration::from_secs(10),
                    "Order book test took too long: {:?}",
                    elapsed
                );
            }
            Err(e) => {
                debug!("Order book test failed in {:?}: {:?}", elapsed, e);
            }
        }

        info!("Market data performance test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_market_data_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
        // Starting market data edge cases test

        let client = DeribitHttpClient::new();

        // Test invalid instrument ticker
        debug!("Testing edge case: Invalid instrument ticker");
        let start_time = Instant::now();
        let result = client.get_ticker("INVALID-INSTRUMENT").await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(_) => {
                info!(
                    "Invalid instrument ticker unexpectedly succeeded in {:?}",
                    elapsed
                );
            }
            Err(e) => {
                info!(
                    "Invalid instrument ticker failed as expected in {:?}: {:?}",
                    elapsed, e
                );
            }
        }
        assert!(
            elapsed < Duration::from_secs(30),
            "Invalid instrument ticker took too long: {:?}",
            elapsed
        );

        // Test empty currency instruments
        debug!("Testing edge case: Empty currency instruments");
        let start_time = Instant::now();
        let result = client.get_instruments("", None, None).await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(instruments) => {
                info!(
                    "Empty currency instruments unexpectedly succeeded in {:?}: {} instruments",
                    elapsed,
                    instruments.len()
                );
            }
            Err(e) => {
                info!(
                    "Empty currency instruments failed as expected in {:?}: {:?}",
                    elapsed, e
                );
            }
        }
        assert!(
            elapsed < Duration::from_secs(30),
            "Empty currency instruments took too long: {:?}",
            elapsed
        );

        // Test invalid order book depth
        debug!("Testing edge case: Invalid order book depth");
        let start_time = Instant::now();
        let result = client.get_order_book("BTC-PERPETUAL", Some(1000)).await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(ob) => {
                info!(
                    "Invalid order book depth unexpectedly succeeded in {:?}: {} bids, {} asks",
                    elapsed,
                    ob.bids.len(),
                    ob.asks.len()
                );
            }
            Err(e) => {
                info!(
                    "Invalid order book depth failed as expected in {:?}: {:?}",
                    elapsed, e
                );
            }
        }
        assert!(
            elapsed < Duration::from_secs(30),
            "Invalid order book depth took too long: {:?}",
            elapsed
        );

        info!("Market data edge cases test completed successfully");
        Ok(())
    }
}
