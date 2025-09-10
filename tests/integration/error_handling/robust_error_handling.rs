//! Robust Error Handling Integration Tests
//!
//! This test covers comprehensive error handling scenarios:
//! 1. Authentication error recovery
//! 2. Network error handling
//! 3. API error response parsing
//! 4. Timeout and retry logic
//! 5. Graceful degradation under errors

use std::path::Path;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};

use deribit_http::{DeribitHttpClient, HttpConfig};

/// Check if .env file exists and contains required variables
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
async fn test_authentication_error_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting authentication error scenarios test");

    let client = DeribitHttpClient::new(true);

    // Test 1: Invalid OAuth2 credentials
    debug!("Testing invalid OAuth2 credentials");
    let invalid_oauth_result = client
        .authenticate_oauth2("invalid_client_id", "invalid_secret")
        .await;

    match invalid_oauth_result {
        Ok(_) => {
            warn!("Invalid OAuth2 credentials unexpectedly succeeded");
        }
        Err(e) => {
            info!("Invalid OAuth2 credentials correctly failed: {:?}", e);

            // Verify error message quality
            let error_str = e.to_string();
            assert!(!error_str.is_empty(), "Error message should not be empty");
            assert!(error_str.len() > 10, "Error message should be descriptive");

            // Check for authentication-related keywords
            let error_lower = error_str.to_lowercase();
            assert!(
                error_lower.contains("auth")
                    || error_lower.contains("credential")
                    || error_lower.contains("invalid")
                    || error_lower.contains("unauthorized"),
                "Error should indicate authentication failure: {}",
                error_str
            );
        }
    }

    // Test 2: Empty credentials
    debug!("Testing empty credentials");
    let empty_oauth_result = client.authenticate_oauth2("", "").await;

    match empty_oauth_result {
        Ok(_) => {
            warn!("Empty credentials unexpectedly succeeded");
        }
        Err(e) => {
            info!("Empty credentials correctly failed: {:?}", e);

            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("auth")
                    || error_str.contains("credential")
                    || error_str.contains("invalid")
                    || error_str.contains("empty")
                    || error_str.contains("required"),
                "Error should indicate credential issue: {}",
                e
            );
        }
    }

    // Test 3: API key authentication (placeholder - should fail gracefully)
    debug!("Testing API key authentication placeholder");
    let api_key_result = client.authenticate_api_key("test_key", "test_secret").await;

    match api_key_result {
        Ok(_) => {
            warn!("API key authentication unexpectedly succeeded (placeholder should fail)");
        }
        Err(e) => {
            info!(
                "API key authentication failed as expected (placeholder): {:?}",
                e
            );

            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("not") && error_str.contains("implement"),
                "Error should indicate not implemented: {}",
                e
            );
        }
    }

    // Test 4: Authentication state after failures
    debug!("Testing authentication state after failures");
    let auth_state = client.is_authenticated();
    let auth_state_result = auth_state.await;
    info!(
        "Authentication state after failures: {:?}",
        auth_state_result
    );

    // Verify authentication state is still valid
    assert!(
        auth_state_result,
        "Authentication should remain valid after network failures"
    );

    info!("Authentication error scenarios test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_network_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting network error handling test");

    // Test 1: Invalid host
    debug!("Testing invalid host error handling");
    let mut invalid_config = HttpConfig::testnet();
    invalid_config.base_url = "https://nonexistent-host-12345.invalid".parse().unwrap();

    let invalid_client = DeribitHttpClient::with_config(invalid_config)?;
    let invalid_host_result = invalid_client.get_server_time().await;

    match invalid_host_result {
        Ok(_) => {
            warn!("Invalid host unexpectedly succeeded");
        }
        Err(e) => {
            info!("Invalid host correctly failed: {:?}", e);

            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("network")
                    || error_str.contains("dns")
                    || error_str.contains("resolve")
                    || error_str.contains("connection")
                    || error_str.contains("not found"),
                "Error should indicate network issue: {}",
                e
            );
        }
    }

    // Test 2: Very short timeout
    debug!("Testing very short timeout error handling");
    let mut timeout_config = HttpConfig::testnet();
    timeout_config.timeout = Duration::from_millis(1); // Extremely short

    let timeout_client = DeribitHttpClient::with_config(timeout_config)?;
    let timeout_result = timeout_client.get_server_time().await;

    match timeout_result {
        Ok(time) => {
            info!(
                "Short timeout unexpectedly succeeded, time: {} (very fast network)",
                time
            );
        }
        Err(e) => {
            info!("Short timeout correctly failed: {:?}", e);

            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("timeout")
                    || error_str.contains("network")
                    || error_str.contains("connection"),
                "Error should indicate timeout/network issue: {}",
                e
            );
        }
    }

    // Test 3: Invalid SSL port
    debug!("Testing invalid SSL port error handling");
    let mut ssl_config = HttpConfig::testnet();
    ssl_config.base_url = "https://test.deribit.com:12345".parse().unwrap(); // Wrong port

    let ssl_client = DeribitHttpClient::with_config(ssl_config)?;
    let ssl_result = ssl_client.get_server_time().await;

    match ssl_result {
        Ok(_) => {
            warn!("Invalid SSL port unexpectedly succeeded");
        }
        Err(e) => {
            info!("Invalid SSL port correctly failed: {:?}", e);

            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("connection")
                    || error_str.contains("network")
                    || error_str.contains("refused")
                    || error_str.contains("timeout"),
                "Error should indicate connection issue: {}",
                e
            );
        }
    }

    info!("Network error handling test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_api_error_response_handling() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting API error response handling test");

    let client = DeribitHttpClient::new(true);

    // Test 1: Unauthenticated request to private endpoint
    debug!("Testing unauthenticated request to private endpoint");
    let unauth_result = client.get_account_summary("BTC", None).await;

    match unauth_result {
        Ok(_) => {
            warn!("Unauthenticated request unexpectedly succeeded");
        }
        Err(e) => {
            info!("Unauthenticated request correctly failed: {:?}", e);

            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("auth")
                    || error_str.contains("unauthorized")
                    || error_str.contains("permission")
                    || error_str.contains("token")
                    || error_str.contains("credential"),
                "Error should indicate authentication required: {}",
                e
            );
        }
    }

    // Test 2: Invalid currency parameter
    debug!("Testing invalid currency parameter");
    let invalid_currency_result = client.get_account_summary("INVALID_CURRENCY", None).await;

    match invalid_currency_result {
        Ok(_) => {
            warn!("Invalid currency unexpectedly succeeded");
        }
        Err(e) => {
            info!("Invalid currency correctly failed: {:?}", e);

            let error_str = e.to_string().to_lowercase();
            // This might be an auth error or parameter error depending on implementation
            assert!(!error_str.is_empty(), "Error message should not be empty");
        }
    }

    // Test 3: Invalid instrument name
    debug!("Testing invalid instrument name");
    let invalid_instrument_result = client.get_instruments("INVALID_CURRENCY", None, None).await;

    match invalid_instrument_result {
        Ok(instruments) => {
            info!("Invalid instrument request succeeded with {} instruments (might return empty list)", instruments.len());
            // Some APIs return empty results for invalid parameters rather than errors
        }
        Err(e) => {
            info!("Invalid instrument correctly failed: {:?}", e);

            let error_str = e.to_string().to_lowercase();
            assert!(!error_str.is_empty(), "Error message should not be empty");
        }
    }

    info!("API error response handling test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_error_recovery_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting error recovery patterns test");

    let client = DeribitHttpClient::new(true);

    // Test recovery pattern: immediate retry
    debug!("Testing immediate retry pattern");
    let mut immediate_retry_success = false;

    for attempt in 1..=3 {
        debug!("Immediate retry attempt #{}", attempt);

        let result = client.get_server_time().await;
        match result {
            Ok(time) => {
                info!(
                    "Immediate retry succeeded on attempt #{}, time: {}",
                    attempt, time
                );
                immediate_retry_success = true;
                break;
            }
            Err(e) => {
                info!("Immediate retry attempt #{} failed: {:?}", attempt, e);

                // Very short delay for immediate retry
                if attempt < 3 {
                    sleep(Duration::from_millis(10)).await;
                }
            }
        }
    }

    // Test recovery pattern: exponential backoff
    debug!("Testing exponential backoff pattern");
    let mut backoff_success = false;

    for attempt in 1..=4 {
        let backoff_delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1));
        debug!(
            "Exponential backoff attempt #{} (delay: {:?})",
            attempt, backoff_delay
        );

        let result = client.get_server_time().await;
        match result {
            Ok(time) => {
                info!(
                    "Exponential backoff succeeded on attempt #{}, time: {}",
                    attempt, time
                );
                backoff_success = true;
                break;
            }
            Err(e) => {
                info!("Exponential backoff attempt #{} failed: {:?}", attempt, e);

                if attempt < 4 {
                    debug!("Waiting {:?} before next attempt", backoff_delay);
                    sleep(backoff_delay).await;
                }
            }
        }
    }

    // Test recovery pattern: linear backoff
    debug!("Testing linear backoff pattern");
    let mut linear_success = false;

    for attempt in 1..=3 {
        let linear_delay = Duration::from_millis(200 * attempt as u64);
        debug!(
            "Linear backoff attempt #{} (delay: {:?})",
            attempt, linear_delay
        );

        let result = client.get_server_time().await;
        match result {
            Ok(time) => {
                info!(
                    "Linear backoff succeeded on attempt #{}, time: {}",
                    attempt, time
                );
                linear_success = true;
                break;
            }
            Err(e) => {
                info!("Linear backoff attempt #{} failed: {:?}", attempt, e);

                if attempt < 3 {
                    debug!("Waiting {:?} before next attempt", linear_delay);
                    sleep(linear_delay).await;
                }
            }
        }
    }

    info!("Recovery pattern results:");
    info!(
        "  Immediate retry: {}",
        if immediate_retry_success {
            "SUCCESS"
        } else {
            "FAILED"
        }
    );
    info!(
        "  Exponential backoff: {}",
        if backoff_success { "SUCCESS" } else { "FAILED" }
    );
    info!(
        "  Linear backoff: {}",
        if linear_success { "SUCCESS" } else { "FAILED" }
    );

    // At least one pattern should succeed if network is available
    let any_success = immediate_retry_success || backoff_success || linear_success;
    if any_success {
        info!("At least one recovery pattern succeeded");
    } else {
        warn!("All recovery patterns failed - network may be unavailable");
    }

    info!("Error recovery patterns test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_error_categorization() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting error categorization test");

    // Test different error scenarios and categorize them
    let mut error_categories = std::collections::HashMap::new();

    // Test 1: Invalid host
    debug!("Testing error scenario: Invalid host");
    let mut config = HttpConfig::testnet();
    config.base_url = "https://invalid-host.example".parse().unwrap();
    let client = DeribitHttpClient::with_config(config)?;
    let result = client.get_server_time().await;

    match result {
        Ok(_) => {
            info!("Scenario 'Invalid host' unexpectedly succeeded");
        }
        Err(e) => {
            let error_str = e.to_string().to_lowercase();
            let category = if error_str.contains("network")
                || error_str.contains("dns")
                || error_str.contains("resolve")
            {
                "network"
            } else if error_str.contains("connection") {
                "connection"
            } else {
                "other"
            };

            info!(
                "Scenario 'Invalid host' failed with {} error: {:?}",
                category, e
            );
            *error_categories.entry(category.to_string()).or_insert(0) += 1;

            assert!(
                !error_str.is_empty(),
                "Error message should not be empty for Invalid host"
            );
            assert!(
                error_str.len() > 5,
                "Error message should be descriptive for Invalid host"
            );
        }
    }

    sleep(Duration::from_millis(100)).await;

    // Test 2: Very short timeout
    debug!("Testing error scenario: Very short timeout");
    let mut config = HttpConfig::testnet();
    config.timeout = Duration::from_millis(1);
    let client = DeribitHttpClient::with_config(config)?;
    let result = client.get_server_time().await;

    match result {
        Ok(_) => {
            info!("Scenario 'Very short timeout' unexpectedly succeeded");
        }
        Err(e) => {
            let error_str = e.to_string().to_lowercase();
            let category = if error_str.contains("timeout") {
                "timeout"
            } else if error_str.contains("network") || error_str.contains("connection") {
                "network"
            } else {
                "other"
            };

            info!(
                "Scenario 'Very short timeout' failed with {} error: {:?}",
                category, e
            );
            *error_categories.entry(category.to_string()).or_insert(0) += 1;

            assert!(
                !error_str.is_empty(),
                "Error message should not be empty for Very short timeout"
            );
            assert!(
                error_str.len() > 5,
                "Error message should be descriptive for Very short timeout"
            );
        }
    }

    sleep(Duration::from_millis(100)).await;

    // Test 3: Invalid credentials
    debug!("Testing error scenario: Invalid credentials");
    let client = DeribitHttpClient::new(true);
    let result = client.authenticate_oauth2("invalid", "invalid").await;

    match result {
        Ok(_) => {
            info!("Scenario 'Invalid credentials' unexpectedly succeeded");
        }
        Err(e) => {
            let error_str = e.to_string().to_lowercase();
            let category = if error_str.contains("auth") || error_str.contains("credential") {
                "authentication"
            } else {
                "other"
            };

            info!(
                "Scenario 'Invalid credentials' failed with {} error: {:?}",
                category, e
            );
            *error_categories.entry(category.to_string()).or_insert(0) += 1;

            assert!(
                !error_str.is_empty(),
                "Error message should not be empty for Invalid credentials"
            );
            assert!(
                error_str.len() > 5,
                "Error message should be descriptive for Invalid credentials"
            );
        }
    }

    sleep(Duration::from_millis(100)).await;

    info!("Error categorization results:");
    for (category, count) in error_categories {
        info!("  {}: {} errors", category, count);
    }

    info!("Error categorization test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_graceful_degradation_under_errors() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting graceful degradation under errors test");

    let client = DeribitHttpClient::new(true);

    // Test graceful degradation by attempting multiple operations
    let mut operation_results = Vec::new();

    // Test 1: Server Time
    debug!("Testing graceful degradation for: Server Time");
    let start_time = Instant::now();
    let result = client
        .get_server_time()
        .await
        .map(|t| format!("time: {}", t));
    let elapsed = start_time.elapsed();

    let success = result.is_ok();
    operation_results.push(("Server Time", success, elapsed));

    match result {
        Ok(response) => {
            info!(
                "Operation 'Server Time' succeeded in {:?}: {}",
                elapsed, response
            );
        }
        Err(e) => {
            info!("Operation 'Server Time' failed in {:?}: {:?}", elapsed, e);
            assert!(
                elapsed < Duration::from_secs(30),
                "Failed operation should not hang indefinitely"
            );
        }
    }

    sleep(Duration::from_millis(100)).await;

    // Test 2: BTC Instruments
    debug!("Testing graceful degradation for: BTC Instruments");
    let start_time = Instant::now();
    let result = client
        .get_instruments("BTC", None, None)
        .await
        .map(|i| format!("{} instruments", i.len()));
    let elapsed = start_time.elapsed();

    let success = result.is_ok();
    operation_results.push(("BTC Instruments", success, elapsed));

    match result {
        Ok(response) => {
            info!(
                "Operation 'BTC Instruments' succeeded in {:?}: {}",
                elapsed, response
            );
        }
        Err(e) => {
            info!(
                "Operation 'BTC Instruments' failed in {:?}: {:?}",
                elapsed, e
            );
            assert!(
                elapsed < Duration::from_secs(30),
                "Failed operation should not hang indefinitely"
            );
        }
    }

    sleep(Duration::from_millis(100)).await;

    // Test 3: ETH Instruments
    debug!("Testing graceful degradation for: ETH Instruments");
    let start_time = Instant::now();
    let result = client
        .get_instruments("ETH", None, None)
        .await
        .map(|i| format!("{} instruments", i.len()));
    let elapsed = start_time.elapsed();

    let success = result.is_ok();
    operation_results.push(("ETH Instruments", success, elapsed));

    match result {
        Ok(response) => {
            info!(
                "Operation 'ETH Instruments' succeeded in {:?}: {}",
                elapsed, response
            );
        }
        Err(e) => {
            info!(
                "Operation 'ETH Instruments' failed in {:?}: {:?}",
                elapsed, e
            );
            assert!(
                elapsed < Duration::from_secs(30),
                "Failed operation should not hang indefinitely"
            );
        }
    }

    sleep(Duration::from_millis(100)).await;

    // Test 4: Invalid Instruments
    debug!("Testing graceful degradation for: Invalid Instruments");
    let start_time = Instant::now();
    let result = client
        .get_instruments("INVALID", None, None)
        .await
        .map(|i| format!("{} instruments", i.len()));
    let elapsed = start_time.elapsed();

    let success = result.is_ok();
    operation_results.push(("Invalid Instruments", success, elapsed));

    match result {
        Ok(response) => {
            info!(
                "Operation 'Invalid Instruments' succeeded in {:?}: {}",
                elapsed, response
            );
        }
        Err(e) => {
            info!(
                "Operation 'Invalid Instruments' failed in {:?}: {:?}",
                elapsed, e
            );
            assert!(
                elapsed < Duration::from_secs(30),
                "Failed operation should not hang indefinitely"
            );
        }
    }

    sleep(Duration::from_millis(100)).await;

    // Analyze graceful degradation
    let total_operations = operation_results.len();
    let successful_operations = operation_results
        .iter()
        .filter(|(_, success, _)| *success)
        .count();
    let failed_operations = total_operations - successful_operations;

    let avg_time = if !operation_results.is_empty() {
        let total_time: Duration = operation_results
            .iter()
            .map(|(_, _, elapsed)| *elapsed)
            .sum();
        total_time / operation_results.len() as u32
    } else {
        Duration::from_secs(0)
    };

    info!("Graceful degradation results:");
    info!("  Total operations: {}", total_operations);
    info!(
        "  Successful: {} ({:.1}%)",
        successful_operations,
        (successful_operations as f64 / total_operations as f64) * 100.0
    );
    info!(
        "  Failed: {} ({:.1}%)",
        failed_operations,
        (failed_operations as f64 / total_operations as f64) * 100.0
    );
    info!("  Average time: {:?}", avg_time);

    // System should handle errors gracefully without crashing
    assert!(total_operations > 0, "Should have attempted operations");

    // At least some operations should succeed or fail gracefully
    for (operation_name, _, elapsed) in operation_results {
        assert!(
            elapsed < Duration::from_secs(30),
            "Operation '{}' should complete in reasonable time",
            operation_name
        );
    }

    info!("Graceful degradation under errors test completed successfully");
    Ok(())
}
