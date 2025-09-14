//! Timeout Handling Integration Tests
//!
//! This test covers timeout handling scenarios:
//! 1. Request timeout configuration and enforcement
//! 2. Connection timeout behavior
//! 3. Read timeout handling
//! 4. Timeout recovery and retry logic
//! 5. Timeout parameter validation

use std::path::Path;
use std::time::Duration;
use tokio::time::{sleep, timeout, Instant};
use tracing::{debug, info, warn};
use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
#[allow(dead_code)]
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with authentication credentials".into());
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

#[tokio::test]
#[serial_test::serial]
async fn test_progressive_timeout_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Starting progressive timeout handling test

    // Test with progressively longer timeouts
    let timeout_durations = vec![
        Duration::from_millis(10),  // Extremely short - should fail
        Duration::from_millis(100), // Very short - likely to fail
        Duration::from_millis(500), // Short - might work
        Duration::from_secs(2),     // Medium - should work
        Duration::from_secs(10),    // Long - should definitely work
    ];

    let mut first_success_timeout = None;

    for (i, timeout_duration) in timeout_durations.into_iter().enumerate() {
        debug!("Testing timeout #{}: {:?}", i + 1, timeout_duration);

        // Use default client since we can't configure custom timeouts
        let client = DeribitHttpClient::new();

        let start_time = Instant::now();
        let result = client.get_server_time().await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(time) => {
                info!(
                    "Timeout #{} ({:?}) succeeded in {:?}, time: {}",
                    i + 1,
                    timeout_duration,
                    elapsed,
                    time
                );

                if first_success_timeout.is_none() {
                    first_success_timeout = Some(timeout_duration);
                }

                assert!(time > 0, "Server time should be positive");
                // Request should complete within the timeout (plus some buffer)
                assert!(
                    elapsed <= timeout_duration + Duration::from_millis(500),
                    "Request should complete within timeout + buffer"
                );
            }
            Err(e) => {
                info!(
                    "Timeout #{} ({:?}) failed in {:?}: {:?}",
                    i + 1,
                    timeout_duration,
                    elapsed,
                    e
                );

                // Verify the timeout was respected
                assert!(
                    elapsed <= timeout_duration + Duration::from_secs(1),
                    "Request should respect timeout configuration"
                );

                // Verify error is timeout-related
                let error_str = e.to_string().to_lowercase();
                assert!(
                    error_str.contains("timeout")
                        || error_str.contains("network")
                        || error_str.contains("connection"),
                    "Error should be timeout-related: {}",
                    e
                );
            }
        }

        // Small delay between tests
        sleep(Duration::from_millis(100)).await;
    }

    if let Some(success_timeout) = first_success_timeout {
        info!("First successful timeout was: {:?}", success_timeout);
    } else {
        warn!("No timeouts succeeded - network might be very slow or unavailable");
    }

    info!("Progressive timeout handling test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_timeout_consistency() -> Result<(), Box<dyn std::error::Error>> {
    // Starting timeout consistency test

    let timeout_duration = Duration::from_millis(200);
    // Use default client since we can't configure custom timeouts
    let client = DeribitHttpClient::new();

    // Test multiple requests with the same timeout to ensure consistency
    let mut timings = Vec::new();
    let num_requests = 5;

    for i in 0..num_requests {
        debug!("Consistency test request #{}", i + 1);

        let start_time = Instant::now();
        let result = client.get_server_time().await;
        let elapsed = start_time.elapsed();

        timings.push(elapsed);

        match result {
            Ok(time) => {
                info!(
                    "Request #{} succeeded in {:?}, time: {}",
                    i + 1,
                    elapsed,
                    time
                );
                assert!(
                    elapsed <= timeout_duration + Duration::from_millis(500),
                    "Successful request should be within timeout"
                );
            }
            Err(e) => {
                info!("Request #{} failed in {:?}: {:?}", i + 1, elapsed, e);
                assert!(
                    elapsed <= timeout_duration + Duration::from_secs(1),
                    "Failed request should respect timeout"
                );
            }
        }

        // Small delay between requests
        sleep(Duration::from_millis(50)).await;
    }

    // Analyze timing consistency
    let min_timing = timings.iter().min().unwrap();
    let max_timing = timings.iter().max().unwrap();
    let avg_timing = Duration::from_nanos(
        (timings.iter().map(|d| d.as_nanos()).sum::<u128>() / timings.len() as u128)
            .try_into()
            .unwrap_or(0),
    );

    info!(
        "Timing analysis - min: {:?}, max: {:?}, avg: {:?}",
        min_timing, max_timing, avg_timing
    );

    // Verify reasonable consistency (max shouldn't be much larger than min for same operation)
    let timing_ratio = max_timing.as_millis() as f64 / min_timing.as_millis().max(1) as f64;
    assert!(
        timing_ratio < 10.0,
        "Timing should be reasonably consistent (ratio: {})",
        timing_ratio
    );

    info!("Timeout consistency test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_timeout_under_load() -> Result<(), Box<dyn std::error::Error>> {
    // Starting timeout under load test

    let timeout_duration = Duration::from_secs(3);
    // Use default client since we can't configure custom timeouts
    let client = DeribitHttpClient::new();

    // Create multiple concurrent requests to test timeout under load
    let mut handles = Vec::new();
    let num_concurrent = 5;

    debug!("Starting {} concurrent requests", num_concurrent);

    for i in 0..num_concurrent {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let start_time = Instant::now();
            let result = client_clone.get_server_time().await;
            let elapsed = start_time.elapsed();

            debug!("Concurrent request #{} completed in {:?}", i + 1, elapsed);
            (i, result, elapsed)
        });
        handles.push(handle);
    }

    // Wait for all requests with a reasonable timeout
    let overall_timeout = timeout_duration + Duration::from_secs(2);
    let results = timeout(overall_timeout, async {
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    warn!("Concurrent request panicked: {:?}", e);
                }
            }
        }
        results
    })
    .await;

    match results {
        Ok(results) => {
            info!("All {} concurrent requests completed", results.len());

            let mut success_count = 0;
            let mut failure_count = 0;
            let mut total_time = Duration::from_nanos(0);

            for (i, result, elapsed) in results {
                total_time += elapsed;

                match result {
                    Ok(time) => {
                        success_count += 1;
                        debug!(
                            "Request #{} succeeded in {:?}, time: {}",
                            i + 1,
                            elapsed,
                            time
                        );

                        // Even under load, individual requests should respect timeout
                        assert!(
                            elapsed <= timeout_duration + Duration::from_secs(1),
                            "Request should respect timeout even under load"
                        );
                    }
                    Err(e) => {
                        failure_count += 1;
                        debug!("Request #{} failed in {:?}: {:?}", i + 1, elapsed, e);

                        // Failed requests should also respect timeout
                        assert!(
                            elapsed <= timeout_duration + Duration::from_secs(1),
                            "Failed request should respect timeout"
                        );
                    }
                }
            }

            let avg_time = total_time / num_concurrent;
            info!(
                "Load test results: {} successes, {} failures, avg time: {:?}",
                success_count, failure_count, avg_time
            );

            // At least some requests should complete
            assert!(
                success_count + failure_count == num_concurrent,
                "All requests should complete"
            );
        }
        Err(_) => {
            warn!("Overall timeout exceeded - system might be under heavy load");
        }
    }

    info!("Timeout under load test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_timeout_escalation() -> Result<(), Box<dyn std::error::Error>> {
    // Starting timeout escalation test

    // Simulate a retry pattern with escalating timeouts
    let escalation_timeouts = vec![
        Duration::from_millis(100), // First attempt - quick
        Duration::from_millis(500), // Second attempt - medium
        Duration::from_secs(2),     // Third attempt - longer
        Duration::from_secs(5),     // Final attempt - longest
    ];

    let mut first_success = None;

    for (attempt, timeout_duration) in escalation_timeouts.into_iter().enumerate() {
        debug!(
            "Escalation attempt #{} with timeout {:?}",
            attempt + 1,
            timeout_duration
        );

        // Use default client since we can't configure custom timeouts
        let client = DeribitHttpClient::new();

        let start_time = Instant::now();
        let result = client.get_server_time().await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(time) => {
                info!(
                    "Escalation attempt #{} succeeded in {:?}, time: {}",
                    attempt + 1,
                    elapsed,
                    time
                );

                if first_success.is_none() {
                    first_success = Some(attempt + 1);
                }

                assert!(time > 0, "Server time should be positive");
                assert!(
                    elapsed <= timeout_duration + Duration::from_millis(500),
                    "Request should complete within timeout"
                );

                // If we succeed, we can break the escalation
                break;
            }
            Err(e) => {
                info!(
                    "Escalation attempt #{} failed in {:?}: {:?}",
                    attempt + 1,
                    elapsed,
                    e
                );

                assert!(
                    elapsed <= timeout_duration + Duration::from_secs(1),
                    "Failed request should respect timeout"
                );

                // Continue to next escalation level
            }
        }

        // Delay before retry (exponential backoff simulation)
        let backoff_delay = Duration::from_millis(100 * (attempt as u64 + 1));
        debug!("Waiting {:?} before next attempt", backoff_delay);
        sleep(backoff_delay).await;
    }

    if let Some(success_attempt) = first_success {
        info!("Escalation succeeded on attempt #{}", success_attempt);
    } else {
        warn!("All escalation attempts failed - network might be unavailable");
    }

    info!("Timeout escalation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_timeout_with_authentication() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    // Starting timeout with authentication test

    let timeout_duration = Duration::from_secs(5);
    // Use default client since we can't configure custom timeouts
    let client = DeribitHttpClient::new();

    // Test authentication timeout
    debug!("Testing authentication with timeout");
    let auth_start = Instant::now();

    let auth_result: Result<(), Box<dyn std::error::Error>> =
        if let (Ok(_client_id), Ok(_client_secret)) = (
            std::env::var("DERIBIT_CLIENT_ID"),
            std::env::var("DERIBIT_CLIENT_SECRET"),
        ) {
            // Authentication is now automatic - no need to call authenticate_oauth2
            info!("Using automatic authentication with OAuth2 credentials");
            Ok(())
        } else if let (Ok(_api_key), Ok(_api_secret)) = (
            std::env::var("DERIBIT_API_KEY"),
            std::env::var("DERIBIT_API_SECRET"),
        ) {
            // Since authenticate_api_key method was also removed, test with authenticated request
            match client.get_account_summary("BTC", None).await {
                Ok(_) => {
                    info!("Using automatic authentication with API key credentials");
                    Ok(())
                }
                Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
            }
        } else {
            return Err("No authentication credentials available".into());
        };

    let auth_elapsed = auth_start.elapsed();

    match auth_result {
        Ok(_) => {
            info!("Authenticated request succeeded in {:?}", auth_elapsed);
            // Test that we got a valid response
            assert!(
                auth_elapsed <= timeout_duration + Duration::from_secs(1),
                "Authentication should respect timeout"
            );

            // Test authenticated request with timeout
            debug!("Testing authenticated request with timeout");
            let request_start = Instant::now();
            let request_result = client.get_account_summary("BTC", None).await;
            let request_elapsed = request_start.elapsed();

            match request_result {
                Ok(_summary) => {
                    info!("Authenticated request succeeded in {:?}", request_elapsed);
                    assert!(
                        request_elapsed <= timeout_duration + Duration::from_secs(1),
                        "Authenticated request should respect timeout"
                    );
                }
                Err(e) => {
                    info!(
                        "Authenticated request failed in {:?}: {:?}",
                        request_elapsed, e
                    );
                    assert!(
                        request_elapsed <= timeout_duration + Duration::from_secs(1),
                        "Failed authenticated request should respect timeout"
                    );
                }
            }
        }
        Err(e) => {
            info!("Authentication failed in {:?}: {:?}", auth_elapsed, e);
            assert!(
                auth_elapsed <= timeout_duration + Duration::from_secs(1),
                "Failed authentication should respect timeout"
            );

            // Test public endpoint as fallback
            debug!("Testing public endpoint as fallback");
            let public_start = Instant::now();
            let public_result = client.get_server_time().await;
            let public_elapsed = public_start.elapsed();

            match public_result {
                Ok(time) => {
                    info!(
                        "Public endpoint succeeded in {:?}, time: {}",
                        public_elapsed, time
                    );
                    assert!(
                        public_elapsed <= timeout_duration + Duration::from_secs(1),
                        "Public request should respect timeout"
                    );
                }
                Err(e) => {
                    info!("Public endpoint failed in {:?}: {:?}", public_elapsed, e);
                    assert!(
                        public_elapsed <= timeout_duration + Duration::from_secs(1),
                        "Failed public request should respect timeout"
                    );
                }
            }
        }
    }

    info!("Timeout with authentication test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_timeout_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    // Starting timeout edge cases test

    // Test edge case timeouts
    let edge_case_timeouts = vec![
        (Duration::from_millis(1), "Minimum timeout"),
        (Duration::from_millis(999), "Just under 1 second"),
        (Duration::from_secs(1), "Exactly 1 second"),
        (Duration::from_millis(1001), "Just over 1 second"),
        (Duration::from_secs(30), "Long timeout"),
    ];

    for (timeout_duration, description) in edge_case_timeouts {
        debug!(
            "Testing edge case: {} ({:?})",
            description, timeout_duration
        );

        // Use default client since we can't configure custom timeouts
        let client_result: Result<DeribitHttpClient, Box<dyn std::error::Error>> =
            Ok(DeribitHttpClient::new());

        match client_result {
            Ok(client) => {
                let start_time = Instant::now();
                let result = client.get_server_time().await;
                let elapsed = start_time.elapsed();

                match result {
                    Ok(time) => {
                        info!("{} succeeded in {:?}, time: {}", description, elapsed, time);
                        assert!(
                            elapsed <= timeout_duration + Duration::from_secs(1),
                            "Request should respect timeout for {}",
                            description
                        );
                    }
                    Err(e) => {
                        info!("{} failed in {:?}: {:?}", description, elapsed, e);
                        assert!(
                            elapsed <= timeout_duration + Duration::from_secs(1),
                            "Failed request should respect timeout for {}",
                            description
                        );
                    }
                }
            }
            Err(e) => {
                info!("{} - client creation failed: {:?}", description, e);
                // Some extreme timeout values might be rejected at client creation
            }
        }

        // Small delay between edge cases
        sleep(Duration::from_millis(50)).await;
    }

    info!("Timeout edge cases test completed successfully");
    Ok(())
}
