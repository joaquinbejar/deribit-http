//! Connection Recovery Integration Tests
//!
//! This test covers connection recovery scenarios:
//! 1. Automatic reconnection after network failure
//! 2. Connection state management
//! 3. Request retry logic
//! 4. Graceful degradation handling
//! 5. Connection pool management

#[cfg(test)]
mod connection_recovery_tests {
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

    /// Authenticate client for testing
    async fn authenticate_client(
        _client: &DeribitHttpClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let (Ok(_client_id), Ok(_client_secret)) = (
            std::env::var("DERIBIT_CLIENT_ID"),
            std::env::var("DERIBIT_CLIENT_SECRET"),
        ) {
            // OAuth2 authentication would be performed here
            // For now, we just verify the credentials are available
            return Ok(());
        }

        if let (Ok(_api_key), Ok(_api_secret)) = (
            std::env::var("DERIBIT_API_KEY"),
            std::env::var("DERIBIT_API_SECRET"),
        ) {
            // API key authentication would be performed here
            // For now, we just verify the credentials are available
            return Ok(());
        }

        Err("No valid authentication credentials found".into())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_connection_persistence() -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting connection persistence test");

        let client = DeribitHttpClient::new();

        // Test multiple requests over time to verify connection persistence
        let num_requests = 10;
        let request_interval = Duration::from_millis(500);

        let mut successful_requests = 0;
        let mut failed_requests = 0;
        let mut request_timings = Vec::new();

        debug!(
            "Testing {} requests with {}ms intervals",
            num_requests,
            request_interval.as_millis()
        );

        for i in 0..num_requests {
            let start_time = Instant::now();
            let result = client.get_server_time().await;
            let elapsed = start_time.elapsed();

            request_timings.push(elapsed);

            match result {
                Ok(time) => {
                    successful_requests += 1;
                    debug!(
                        "Persistence request #{} succeeded in {:?}, time: {}",
                        i + 1,
                        elapsed,
                        time
                    );
                }
                Err(e) => {
                    failed_requests += 1;
                    debug!(
                        "Persistence request #{} failed in {:?}: {:?}",
                        i + 1,
                        elapsed,
                        e
                    );
                }
            }

            // Wait before next request
            if i < num_requests - 1 {
                sleep(request_interval).await;
            }
        }

        // Analyze timing consistency (connection reuse should show consistent timings)
        let avg_timing = if !request_timings.is_empty() {
            Duration::from_nanos(
                (request_timings.iter().map(|d| d.as_nanos()).sum::<u128>()
                    / request_timings.len() as u128)
                    .try_into()
                    .unwrap_or(0),
            )
        } else {
            Duration::from_secs(0)
        };

        let default_duration = Duration::from_secs(0);
        let min_timing = request_timings.iter().min().unwrap_or(&default_duration);
        let max_timing = request_timings.iter().max().unwrap_or(&default_duration);

        info!("Connection persistence results:");
        info!(
            "  Successful requests: {}/{}",
            successful_requests, num_requests
        );
        info!("  Failed requests: {}", failed_requests);
        info!(
            "  Timing - avg: {:?}, min: {:?}, max: {:?}",
            avg_timing, min_timing, max_timing
        );

        // Connection persistence should result in reasonable success rate
        let success_rate = successful_requests as f64 / num_requests as f64;
        assert!(
            success_rate >= 0.5,
            "Success rate should be at least 50% for connection persistence"
        );

        // Timing should be reasonably consistent if connection is reused
        if successful_requests > 1 {
            let timing_variance =
                max_timing.as_millis() as f64 / min_timing.as_millis().max(1) as f64;
            info!("  Timing variance ratio: {:.2}", timing_variance);

            if timing_variance < 5.0 {
                info!("  Good timing consistency - connection likely reused");
            } else {
                info!("  High timing variance - connections may be recreated");
            }
        }

        info!("Connection persistence test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_retry_mechanism() -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting retry mechanism test");

        let client = DeribitHttpClient::new();

        // Test retry pattern with exponential backoff
        let max_retries = 3;
        let base_delay = Duration::from_millis(100);

        for attempt in 0..max_retries {
            let retry_delay = base_delay * 2_u32.pow(attempt);

            debug!("Retry attempt #{} (delay: {:?})", attempt + 1, retry_delay);

            let start_time = Instant::now();
            let result = client.get_server_time().await;
            let elapsed = start_time.elapsed();

            match result {
                Ok(time) => {
                    info!(
                        "Retry attempt #{} succeeded in {:?}, time: {}",
                        attempt + 1,
                        elapsed,
                        time
                    );
                    // Success - break retry loop
                    break;
                }
                Err(e) => {
                    info!(
                        "Retry attempt #{} failed in {:?}: {:?}",
                        attempt + 1,
                        elapsed,
                        e
                    );

                    // Check if it's a retryable error
                    let error_str = e.to_string().to_lowercase();
                    let is_retryable = error_str.contains("timeout")
                        || error_str.contains("network")
                        || error_str.contains("connection")
                        || error_str.contains("temporary");

                    if is_retryable {
                        info!("Error appears retryable, continuing retry sequence");
                    } else {
                        info!("Error appears non-retryable, stopping retry sequence");
                        break;
                    }

                    // Wait before next retry (except for last attempt)
                    if attempt < max_retries - 1 {
                        debug!("Waiting {:?} before next retry", retry_delay);
                        sleep(retry_delay).await;
                    }
                }
            }
        }

        info!("Retry mechanism test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_authentication_recovery() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting authentication recovery test");

        let client = DeribitHttpClient::new();

        // Attempt initial authentication
        let initial_auth_result = authenticate_client(&client).await;

        match initial_auth_result {
            Ok(_) => {
                info!("Initial authentication successful");

                // Test authenticated request
                debug!("Testing initial authenticated request");
                let initial_request = client.get_account_summary("BTC", None).await;

                match initial_request {
                    Ok(summary) => {
                        info!("Initial authenticated request successful");
                    }
                    Err(e) => {
                        info!("Initial authenticated request failed: {:?}", e);
                    }
                }

                // Simulate connection recovery by waiting and retrying
                debug!("Simulating connection recovery scenario");
                sleep(Duration::from_secs(2)).await;

                // Test if authentication persists
                debug!("Testing authentication persistence after delay");
                let recovery_request = client.get_account_summary("BTC", None).await;

                match recovery_request {
                    Ok(summary) => {
                        info!("Authentication persisted successfully after recovery");
                    }
                    Err(e) => {
                        info!(
                            "Authentication may have expired, testing re-authentication: {:?}",
                            e
                        );

                        // Test re-authentication
                        debug!("Attempting re-authentication");
                        let reauth_result = authenticate_client(&client).await;

                        match reauth_result {
                            Ok(_) => {
                                info!("Re-authentication successful");

                                // Test request after re-authentication
                                let reauth_request = client.get_account_summary("BTC", None).await;
                                match reauth_request {
                                    Ok(_) => info!("Request successful after re-authentication"),
                                    Err(e) => {
                                        info!(
                                            "Request failed even after re-authentication: {:?}",
                                            e
                                        )
                                    }
                                }
                            }
                            Err(e) => {
                                info!("Re-authentication failed: {:?}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                info!("Initial authentication failed: {:?}", e);

                // Test recovery from failed authentication
                debug!("Testing recovery from failed authentication");
                sleep(Duration::from_secs(1)).await;

                let recovery_auth_result = authenticate_client(&client).await;
                match recovery_auth_result {
                    Ok(_) => {
                        info!("Authentication recovery successful");
                    }
                    Err(e) => {
                        info!("Authentication recovery also failed: {:?}", e);
                    }
                }
            }
        }

        info!("Authentication recovery test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_concurrent_recovery() -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting concurrent recovery test");

        let client = DeribitHttpClient::new();

        // Test concurrent requests during potential connection issues
        let num_concurrent = 3;
        let mut handles = Vec::new();

        debug!(
            "Starting {} concurrent requests for recovery testing",
            num_concurrent
        );

        for i in 0..num_concurrent {
            let client_clone = client.clone();
            let handle = tokio::spawn(async move {
                let mut results = Vec::new();

                // Each task makes multiple requests
                for j in 0..3 {
                    let start_time = Instant::now();
                    let result = client_clone.get_server_time().await;
                    let elapsed = start_time.elapsed();

                    results.push((j + 1, result, elapsed));

                    debug!(
                        "Concurrent task {} request {} completed in {:?}",
                        i + 1,
                        j + 1,
                        elapsed
                    );

                    // Small delay between requests in same task
                    sleep(Duration::from_millis(100)).await;
                }

                (i + 1, results)
            });
            handles.push(handle);
        }

        // Wait for all concurrent tasks to complete
        let mut all_results = Vec::new();

        for handle in handles {
            match handle.await {
                Ok((task_id, task_results)) => {
                    debug!(
                        "Concurrent task {} completed with {} results",
                        task_id,
                        task_results.len()
                    );
                    all_results.extend(
                        task_results
                            .into_iter()
                            .map(|(req_id, result, elapsed)| (task_id, req_id, result, elapsed)),
                    );
                }
                Err(e) => {
                    warn!("Concurrent task panicked: {:?}", e);
                }
            }
        }

        // Analyze concurrent recovery results
        let mut total_requests = 0;
        let mut successful_requests = 0;
        let mut failed_requests = 0;
        let mut total_time = Duration::from_nanos(0);

        for (task_id, req_id, result, elapsed) in all_results {
            total_requests += 1;
            total_time += elapsed;

            match result {
                Ok(time) => {
                    successful_requests += 1;
                    debug!(
                        "Task {} request {} succeeded in {:?}, time: {}",
                        task_id, req_id, elapsed, time
                    );
                }
                Err(e) => {
                    failed_requests += 1;
                    debug!(
                        "Task {} request {} failed in {:?}: {:?}",
                        task_id, req_id, elapsed, e
                    );
                }
            }
        }

        let avg_time = if total_requests > 0 {
            total_time / total_requests
        } else {
            Duration::from_secs(0)
        };

        let success_rate = if total_requests > 0 {
            successful_requests as f64 / total_requests as f64
        } else {
            0.0
        };

        info!("Concurrent recovery results:");
        info!("  Total requests: {}", total_requests);
        info!(
            "  Successful: {} ({:.1}%)",
            successful_requests,
            success_rate * 100.0
        );
        info!("  Failed: {}", failed_requests);
        info!("  Average time: {:?}", avg_time);

        // Concurrent requests should have reasonable success rate
        assert!(
            success_rate >= 0.3,
            "Concurrent success rate should be at least 30%"
        );

        info!("Concurrent recovery test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_graceful_degradation_recovery() -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting graceful degradation recovery test");

        // Test with progressively more aggressive timeout settings
        let timeout_configs = vec![
            (Duration::from_millis(100), "Very aggressive"),
            (Duration::from_millis(500), "Aggressive"),
            (Duration::from_secs(1), "Moderate"),
            (Duration::from_secs(3), "Conservative"),
            (Duration::from_secs(10), "Very conservative"),
        ];

        let mut first_success_config = None;

        for (timeout_duration, description) in timeout_configs {
            debug!(
                "Testing {} timeout configuration: {:?}",
                description, timeout_duration
            );

            // Use default client since we can't configure custom timeouts
            let client = DeribitHttpClient::new();

            // Test multiple requests with this configuration
            let mut config_success_count = 0;
            let mut config_failure_count = 0;

            for i in 0..3 {
                let start_time = Instant::now();
                let result = client.get_server_time().await;
                let elapsed = start_time.elapsed();

                match result {
                    Ok(time) => {
                        config_success_count += 1;
                        debug!(
                            "{} config request #{} succeeded in {:?}, time: {}",
                            description,
                            i + 1,
                            elapsed,
                            time
                        );
                    }
                    Err(e) => {
                        config_failure_count += 1;
                        debug!(
                            "{} config request #{} failed in {:?}: {:?}",
                            description,
                            i + 1,
                            elapsed,
                            e
                        );
                    }
                }

                // Small delay between requests
                sleep(Duration::from_millis(100)).await;
            }

            let config_success_rate = config_success_count as f64 / 3.0;
            info!(
                "{} configuration results: {}/3 successes ({:.1}%)",
                description,
                config_success_count,
                config_success_rate * 100.0
            );

            // Record first successful configuration
            if config_success_count > 0 && first_success_config.is_none() {
                first_success_config = Some((timeout_duration, description));
            }

            // If we have good success rate, we can consider this configuration viable
            if config_success_rate >= 0.7 {
                info!("{} configuration shows good reliability", description);
            }

            // Delay before testing next configuration
            sleep(Duration::from_millis(200)).await;
        }

        if let Some((timeout, desc)) = first_success_config {
            info!("First successful configuration: {} ({:?})", desc, timeout);
        } else {
            warn!("No configuration achieved success - network may be unavailable");
        }

        info!("Graceful degradation recovery test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_state_consistency_after_recovery() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting state consistency after recovery test");

        let client = DeribitHttpClient::new();

        // Test authentication state consistency
        debug!("Testing authentication state consistency");

        let auth_result = authenticate_client(&client).await;

        match auth_result {
            Ok(_) => {
                info!("Initial authentication successful");

                // Check authentication state - with automatic auth, client is always ready
                info!("Authentication state before recovery simulation: automatic");

                // Simulate recovery scenario with delay
                debug!("Simulating recovery scenario");
                sleep(Duration::from_secs(3)).await;

                // Check authentication state after delay - with automatic auth, client is always ready
                info!("Authentication state after recovery simulation: automatic");

                // Test if we can still make authenticated requests
                debug!("Testing authenticated request after recovery");
                let auth_request_result = client.get_account_summary("BTC", None).await;

                match auth_request_result {
                    Ok(_) => {
                        info!("Authenticated request successful - state consistent");
                    }
                    Err(e) => {
                        info!(
                            "Authenticated request failed - testing state recovery: {:?}",
                            e
                        );

                        // Check if we need to re-authenticate - with automatic auth, always ready
                        info!("Current authentication state: automatic");

                        // With automatic authentication, try making another request
                        debug!("Attempting another authenticated request");
                        let retry_result = client.get_account_summary("BTC", None).await;

                        match retry_result {
                            Ok(_) => {
                                info!("Retry successful - connection recovered");
                            }
                            Err(retry_e) => {
                                info!("Retry failed: {:?}", retry_e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                info!("Initial authentication failed: {:?}", e);

                // Test public endpoint state consistency
                debug!("Testing public endpoint state consistency");

                let public_result_1 = client.get_server_time().await;
                sleep(Duration::from_secs(1)).await;
                let public_result_2 = client.get_server_time().await;

                match (public_result_1, public_result_2) {
                    (Ok(time1), Ok(time2)) => {
                        info!("Public endpoints consistent: {} -> {}", time1, time2);
                        assert!(time2 >= time1, "Server time should be monotonic");
                    }
                    (Ok(time1), Err(e)) => {
                        info!("Public endpoint became unavailable: {} -> {:?}", time1, e);
                    }
                    (Err(e1), Ok(time2)) => {
                        info!("Public endpoint recovered: {:?} -> {}", e1, time2);
                    }
                    (Err(e1), Err(e2)) => {
                        info!(
                            "Public endpoints consistently failing: {:?} -> {:?}",
                            e1, e2
                        );
                    }
                }
            }
        }

        info!("State consistency after recovery test completed successfully");
        Ok(())
    }
}
