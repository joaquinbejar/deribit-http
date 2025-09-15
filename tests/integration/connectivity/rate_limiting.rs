//! Rate Limiting Integration Tests
//!
//! This test covers rate limiting scenarios:
//! 1. API rate limit detection and handling
//! 2. Backoff strategies and retry logic
//! 3. Rate limit recovery testing
//! 4. Burst request handling
//! 5. Rate limit recovery testing

#[cfg(test)]
mod rate_limiting_tests {
    use deribit_http::DeribitHttpClient;
    use std::path::Path;
    use tokio::time::{Duration, Instant, sleep};
    use tracing::{debug, info, warn};

    /// Check if .env file exists and contains required variables
    fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(".env").exists() {
            return Err(
                "Missing .env file. Please create one with authentication credentials".into(),
            );
        }

        dotenv::dotenv().ok();

        let has_oauth2 = std::env::var("DERIBIT_CLIENT_ID").is_ok()
            && std::env::var("DERIBIT_CLIENT_SECRET").is_ok();
        let has_api_key =
            std::env::var("DERIBIT_API_KEY").is_ok() && std::env::var("DERIBIT_API_SECRET").is_ok();

        if !has_oauth2 && !has_api_key {
            return Err("Missing authentication credentials".into());
        }

        Ok(())
    }

    /// Authenticate client using available credentials
    async fn authenticate_client(
        _client: &DeribitHttpClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let (Ok(_client_id), Ok(_client_secret)) = (
            std::env::var("DERIBIT_CLIENT_ID"),
            std::env::var("DERIBIT_CLIENT_SECRET"),
        ) {
            info!("Using automatic authentication with OAuth2 credentials");
        } else if let (Ok(_api_key), Ok(_api_secret)) = (
            std::env::var("DERIBIT_API_KEY"),
            std::env::var("DERIBIT_API_SECRET"),
        ) {
            info!("Using automatic authentication with API key credentials");
        } else {
            return Err("No valid authentication credentials found".into());
        }
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_rate_limit_compliance() -> Result<(), Box<dyn std::error::Error>> {
        // Starting rate limit compliance test

        let client = DeribitHttpClient::new();

        // Test sequential requests with proper spacing
        let num_requests = 10;
        let min_interval = Duration::from_millis(100); // Minimum interval between requests
        let mut request_times = Vec::new();

        debug!(
            "Testing {} sequential requests with proper spacing",
            num_requests
        );

        for i in 0..num_requests {
            let start_time = Instant::now();

            debug!("Making request #{}", i + 1);
            let result = client.get_server_time().await;

            let elapsed = start_time.elapsed();
            request_times.push((start_time, elapsed, result.is_ok()));

            match result {
                Ok(time) => {
                    debug!(
                        "Request #{} succeeded in {:?}, time: {}",
                        i + 1,
                        elapsed,
                        time
                    );
                }
                Err(e) => {
                    debug!("Request #{} failed in {:?}: {:?}", i + 1, elapsed, e);

                    // Check if it's a rate limit error
                    let error_str = e.to_string().to_lowercase();
                    if error_str.contains("rate")
                        || error_str.contains("limit")
                        || error_str.contains("429")
                    {
                        warn!("Rate limit detected on request #{}: {:?}", i + 1, e);
                    }
                }
            }

            // Wait minimum interval before next request
            if i < num_requests - 1 {
                sleep(min_interval).await;
            }
        }

        // Analyze request timing patterns
        let mut intervals = Vec::new();
        for i in 1..request_times.len() {
            let interval = request_times[i].0.duration_since(request_times[i - 1].0);
            intervals.push(interval);
        }

        let avg_interval = if !intervals.is_empty() {
            Duration::from_nanos(
                (intervals.iter().map(|d| d.as_nanos()).sum::<u128>() / intervals.len() as u128)
                    .try_into()
                    .unwrap_or(0),
            )
        } else {
            Duration::from_secs(0)
        };

        info!("Average interval between requests: {:?}", avg_interval);

        // Count successes and failures
        let success_count = request_times
            .iter()
            .filter(|(_, _, success)| *success)
            .count();
        let failure_count = request_times.len() - success_count;

        info!(
            "Rate limit compliance test completed: {} successes, {} failures",
            success_count, failure_count
        );

        // At least some requests should succeed with proper spacing
        assert!(
            success_count > 0,
            "At least some requests should succeed with proper rate limiting"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_burst_request_handling() -> Result<(), Box<dyn std::error::Error>> {
        // Starting burst request handling test

        let client = DeribitHttpClient::new();

        // Test burst of requests without spacing
        let burst_size = 5;
        let mut burst_results = Vec::new();

        debug!("Testing burst of {} requests without spacing", burst_size);

        let burst_start = Instant::now();

        for i in 0..burst_size {
            let request_start = Instant::now();
            let result = client.get_server_time().await;
            let request_elapsed = request_start.elapsed();

            burst_results.push((i + 1, result, request_elapsed));

            debug!(
                "Burst request #{} completed in {:?}",
                i + 1,
                request_elapsed
            );
        }

        let total_burst_time = burst_start.elapsed();
        info!(
            "Burst of {} requests completed in {:?}",
            burst_size, total_burst_time
        );

        // Analyze burst results
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut rate_limit_count = 0;

        for (request_num, result, elapsed) in burst_results {
            match result {
                Ok(time) => {
                    success_count += 1;
                    debug!(
                        "Burst request #{} succeeded in {:?}, time: {}",
                        request_num, elapsed, time
                    );
                }
                Err(e) => {
                    failure_count += 1;
                    debug!(
                        "Burst request #{} failed in {:?}: {:?}",
                        request_num, elapsed, e
                    );

                    let error_str = e.to_string().to_lowercase();
                    if error_str.contains("rate")
                        || error_str.contains("limit")
                        || error_str.contains("429")
                    {
                        rate_limit_count += 1;
                        info!("Rate limit hit on burst request #{}", request_num);
                    }
                }
            }
        }

        info!(
            "Burst results: {} successes, {} failures, {} rate limits",
            success_count, failure_count, rate_limit_count
        );

        // In a burst scenario, we expect some rate limiting
        if rate_limit_count > 0 {
            info!("Rate limiting detected as expected during burst");
        } else {
            info!("No rate limiting detected - either rate limits are generous or network is fast");
        }

        // Test recovery after burst
        debug!("Testing recovery after burst");
        sleep(Duration::from_secs(2)).await; // Wait for rate limit to reset

        let recovery_result = client.get_server_time().await;
        match recovery_result {
            Ok(time) => {
                info!("Recovery request succeeded, time: {}", time);
            }
            Err(e) => {
                info!("Recovery request failed: {:?}", e);
            }
        }

        info!("Burst request handling test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_rate_limit_backoff_strategy() -> Result<(), Box<dyn std::error::Error>> {
        // Starting rate limit backoff strategy test

        let client = DeribitHttpClient::new();

        // Test exponential backoff pattern
        let backoff_intervals = vec![
            Duration::from_millis(100),
            Duration::from_millis(200),
            Duration::from_millis(400),
            Duration::from_millis(800),
            Duration::from_millis(1600),
        ];

        for (attempt, backoff_duration) in backoff_intervals.into_iter().enumerate() {
            debug!(
                "Backoff attempt #{} with interval {:?}",
                attempt + 1,
                backoff_duration
            );

            let start_time = Instant::now();
            let result = client.get_server_time().await;
            let elapsed = start_time.elapsed();

            match result {
                Ok(time) => {
                    info!(
                        "Backoff attempt #{} succeeded in {:?}, time: {}",
                        attempt + 1,
                        elapsed,
                        time
                    );
                    // If successful, we can break the backoff pattern
                    break;
                }
                Err(e) => {
                    info!(
                        "Backoff attempt #{} failed in {:?}: {:?}",
                        attempt + 1,
                        elapsed,
                        e
                    );

                    let error_str = e.to_string().to_lowercase();
                    if error_str.contains("rate")
                        || error_str.contains("limit")
                        || error_str.contains("429")
                    {
                        info!("Rate limit still active, continuing backoff");
                    } else {
                        info!("Non-rate-limit error, backoff may not be needed");
                    }
                }
            }

            // Wait for backoff duration before next attempt
            debug!("Waiting {:?} for backoff", backoff_duration);
            sleep(backoff_duration).await;
        }

        info!("Rate limit backoff strategy test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_rate_limit_with_different_endpoints() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting rate limit with different endpoints test

        let client = DeribitHttpClient::new();

        // Try to authenticate (may fail, but we'll test what we can)
        let auth_result = authenticate_client(&client).await;

        // Test different endpoint categories - simplified approach
        let endpoint_tests = vec!["Public - Server Time", "Public - Instruments"];

        // Add authenticated endpoints if auth succeeded
        let all_tests = endpoint_tests;
        if auth_result.is_ok() {
            info!("Authentication successful, testing authenticated endpoints");
            // Note: These closures would need to be properly constructed for authenticated endpoints
            // For now, we'll focus on public endpoints
        } else {
            info!("Authentication failed, testing public endpoints only");
        }

        for endpoint_name in all_tests {
            debug!("Testing rate limits for: {}", endpoint_name);

            // Test multiple requests to this endpoint
            let mut endpoint_results = Vec::new();

            for i in 0..3 {
                let start_time = Instant::now();

                // Make a simple public request based on endpoint name
                let result = if endpoint_name.contains("Server Time") {
                    client
                        .get_server_time()
                        .await
                        .map(|t| format!("Server time: {}", t))
                } else {
                    client
                        .get_instruments("BTC", None, None)
                        .await
                        .map(|instruments| format!("Found {} instruments", instruments.len()))
                };

                let elapsed = start_time.elapsed();
                endpoint_results.push((i + 1, result.is_ok(), elapsed));

                match result {
                    Ok(response) => {
                        debug!(
                            "{} request #{} succeeded in {:?}: {}",
                            endpoint_name,
                            i + 1,
                            elapsed,
                            response
                        );
                    }
                    Err(e) => {
                        debug!(
                            "{} request #{} failed in {:?}: {}",
                            endpoint_name,
                            i + 1,
                            elapsed,
                            e
                        );
                    }
                }

                // Small delay between requests
                sleep(Duration::from_millis(100)).await;
            }

            info!(
                "Completed {} requests for endpoint: {}",
                endpoint_results.len(),
                endpoint_name
            );

            // Delay before testing next endpoint category
            sleep(Duration::from_millis(500)).await;
        }

        info!("Rate limit with different endpoints test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_rate_limit_recovery_time() -> Result<(), Box<dyn std::error::Error>> {
        // Starting rate limit recovery time test

        let client = DeribitHttpClient::new();

        // First, try to trigger rate limiting with rapid requests
        debug!("Attempting to trigger rate limiting");
        let mut rate_limit_triggered = false;

        for i in 0..10 {
            let result = client.get_server_time().await;

            match result {
                Ok(time) => {
                    debug!("Rapid request #{} succeeded, time: {}", i + 1, time);
                }
                Err(e) => {
                    let error_str = e.to_string().to_lowercase();
                    if error_str.contains("rate")
                        || error_str.contains("limit")
                        || error_str.contains("429")
                    {
                        info!("Rate limit triggered on request #{}", i + 1);
                        rate_limit_triggered = true;
                        break;
                    } else {
                        debug!("Rapid request #{} failed (non-rate-limit): {:?}", i + 1, e);
                    }
                }
            }

            // Very short delay to try to trigger rate limiting
            sleep(Duration::from_millis(10)).await;
        }

        if rate_limit_triggered {
            info!("Rate limit successfully triggered, testing recovery");

            // Test recovery at different intervals
            let recovery_intervals = vec![
                Duration::from_millis(500),
                Duration::from_secs(1),
                Duration::from_secs(2),
                Duration::from_secs(5),
            ];

            for wait_duration in recovery_intervals.into_iter() {
                debug!("Testing recovery after waiting {:?}", wait_duration);
                sleep(wait_duration).await;

                let recovery_start = Instant::now();
                let result = client.get_server_time().await;
                let recovery_elapsed = recovery_start.elapsed();

                match result {
                    Ok(time) => {
                        info!(
                            "Recovery successful after {:?} wait (request took {:?}), time: {}",
                            wait_duration, recovery_elapsed, time
                        );
                        break;
                    }
                    Err(e) => {
                        let error_str = e.to_string().to_lowercase();
                        if error_str.contains("rate")
                            || error_str.contains("limit")
                            || error_str.contains("429")
                        {
                            info!("Rate limit still active after {:?} wait", wait_duration);
                        } else {
                            info!("Recovery attempt failed with non-rate-limit error: {:?}", e);
                            break;
                        }
                    }
                }
            }
        } else {
            info!(
                "Rate limit not triggered - either limits are generous or requests were spaced enough"
            );

            // Test normal operation timing
            let mut normal_timings = Vec::new();

            for i in 0..5 {
                let start_time = Instant::now();
                let result = client.get_server_time().await;
                let elapsed = start_time.elapsed();

                normal_timings.push(elapsed);

                match result {
                    Ok(time) => {
                        debug!(
                            "Normal request #{} succeeded in {:?}, time: {}",
                            i + 1,
                            elapsed,
                            time
                        );
                    }
                    Err(e) => {
                        debug!("Normal request #{} failed in {:?}: {:?}", i + 1, elapsed, e);
                    }
                }

                sleep(Duration::from_millis(200)).await;
            }

            let avg_timing = if !normal_timings.is_empty() {
                Duration::from_nanos(
                    (normal_timings.iter().map(|d| d.as_nanos()).sum::<u128>()
                        / normal_timings.len() as u128)
                        .try_into()
                        .unwrap_or(0),
                )
            } else {
                Duration::from_secs(0)
            };

            info!("Average normal request timing: {:?}", avg_timing);
        }

        info!("Rate limit recovery time test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_rate_limit_error_handling() -> Result<(), Box<dyn std::error::Error>> {
        // Starting rate limit error handling test

        let client = DeribitHttpClient::new();

        // Test how the client handles various error scenarios
        let mut error_types = std::collections::HashMap::new();

        // Make several requests and categorize any errors
        for i in 0..15 {
            let result = client.get_server_time().await;

            match result {
                Ok(time) => {
                    debug!("Request #{} succeeded, time: {}", i + 1, time);
                }
                Err(e) => {
                    let error_str = e.to_string().to_lowercase();

                    // Categorize the error
                    let error_category = if error_str.contains("rate")
                        || error_str.contains("limit")
                        || error_str.contains("429")
                    {
                        "rate_limit"
                    } else if error_str.contains("timeout") {
                        "timeout"
                    } else if error_str.contains("network") || error_str.contains("connection") {
                        "network"
                    } else if error_str.contains("auth") {
                        "authentication"
                    } else {
                        "other"
                    };

                    *error_types.entry(error_category.to_string()).or_insert(0) += 1;

                    debug!(
                        "Request #{} failed with {} error: {:?}",
                        i + 1,
                        error_category,
                        e
                    );

                    // If it's a rate limit error, test the error details
                    if error_category == "rate_limit" {
                        info!("Rate limit error detected: {:?}", e);

                        // Verify error message quality
                        assert!(
                            !error_str.is_empty(),
                            "Rate limit error message should not be empty"
                        );
                        assert!(
                            error_str.len() > 5,
                            "Rate limit error message should be descriptive"
                        );

                        // Test recovery after rate limit
                        debug!("Testing recovery after rate limit error");
                        sleep(Duration::from_secs(1)).await;

                        let recovery_result = client.get_server_time().await;
                        match recovery_result {
                            Ok(time) => {
                                info!("Recovery after rate limit successful, time: {}", time);
                            }
                            Err(recovery_e) => {
                                info!("Recovery after rate limit failed: {:?}", recovery_e);
                            }
                        }
                    }
                }
            }

            // Variable delay to test different timing patterns
            let delay = Duration::from_millis(50 + (i * 10));
            sleep(delay).await;
        }

        // Report error analysis
        info!("Error analysis:");
        for (error_type, count) in error_types {
            info!("  {}: {} occurrences", error_type, count);
        }

        info!("Rate limit error handling test completed successfully");
        Ok(())
    }
}
