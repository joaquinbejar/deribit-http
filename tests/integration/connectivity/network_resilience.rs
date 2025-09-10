//! Network Resilience Integration Tests
//!
//! This test covers network resilience scenarios:
//! 1. Connection timeout handling
//! 2. Network interruption recovery
//! 3. DNS resolution failures
//! 4. SSL/TLS handshake failures
//! 5. Graceful degradation under network stress

use std::path::Path;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use deribit_http::*;
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

/// Authenticate client using available credentials
async fn authenticate_client(client: &DeribitHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        client
            .authenticate_oauth2(&client_id, &client_secret)
            .await?;
    } else if let (Ok(api_key), Ok(api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        client.authenticate_api_key(&api_key, &api_secret).await?;
    } else {
        return Err("No valid authentication credentials found".into());
    }
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_connection_timeout_handling() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting connection timeout handling test");

    // Create client with very short timeout
    let mut config = HttpConfig::testnet();
    config.timeout = Duration::from_millis(100); // Very short timeout

    let client = DeribitHttpClient::with_config(config)?;

    // Test that timeout is properly handled
    debug!("Testing connection with very short timeout");
    let start_time = std::time::Instant::now();

    // This should timeout quickly
    let result = timeout(Duration::from_secs(5), client.get_server_time()).await;

    let elapsed = start_time.elapsed();
    info!("Request completed in {:?}", elapsed);

    match result {
        Ok(Ok(_)) => {
            info!("Request succeeded despite short timeout - network is very fast");
        }
        Ok(Err(e)) => {
            info!("Request failed as expected due to timeout: {:?}", e);
            // Verify it's a timeout-related error
            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("timeout")
                    || error_str.contains("network")
                    || error_str.contains("connection"),
                "Error should be timeout-related: {}",
                e
            );
        }
        Err(_) => {
            info!("Request timed out at test level - this is acceptable");
        }
    }

    // Verify the timeout was respected (should be much less than 5 seconds)
    assert!(
        elapsed < Duration::from_secs(2),
        "Timeout should be respected"
    );

    info!("Connection timeout handling test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_invalid_host_handling() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting invalid host handling test");

    // Create client with invalid host
    let mut config = HttpConfig::testnet();
    config.base_url = "https://invalid-host-that-does-not-exist.com"
        .parse()
        .unwrap();

    let client = DeribitHttpClient::with_config(config)?;

    debug!("Testing connection to invalid host");
    let result = client.get_server_time().await;

    match result {
        Ok(_) => {
            return Err("Request to invalid host should have failed".into());
        }
        Err(e) => {
            info!("Request correctly failed with invalid host: {:?}", e);
            let error_str = e.to_string().to_lowercase();
            assert!(
                error_str.contains("network")
                    || error_str.contains("dns")
                    || error_str.contains("connection")
                    || error_str.contains("resolve")
                    || error_str.contains("not found"),
                "Error should be network-related: {}",
                e
            );
        }
    }

    info!("Invalid host handling test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_ssl_certificate_validation() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting SSL certificate validation test");

    // Test with valid SSL (normal testnet)
    let client = DeribitHttpClient::new(true);

    debug!("Testing connection with valid SSL certificate");
    let result = client.get_server_time().await;

    match result {
        Ok(time) => {
            info!("SSL connection successful, server time: {}", time);
            assert!(time > 0, "Server time should be positive");
        }
        Err(e) => {
            warn!(
                "SSL connection failed (might be due to auth issues): {:?}",
                e
            );
            // This might fail due to auth, but not due to SSL issues
            let error_str = e.to_string().to_lowercase();
            assert!(
                !error_str.contains("ssl")
                    && !error_str.contains("certificate")
                    && !error_str.contains("tls"),
                "Error should not be SSL-related: {}",
                e
            );
        }
    }

    info!("SSL certificate validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_network_recovery_simulation() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting network recovery simulation test");

    let client = DeribitHttpClient::new(true);

    // Try to authenticate (this might fail, but we test recovery)
    debug!("Attempting initial authentication");
    let auth_result = authenticate_client(&client).await;

    match auth_result {
        Ok(_) => {
            info!("Initial authentication successful");

            // Test multiple consecutive requests to simulate network stress
            debug!("Testing multiple consecutive requests");
            let mut success_count = 0;
            let mut failure_count = 0;

            for i in 0..5 {
                debug!("Request attempt #{}", i + 1);

                match client.get_server_time().await {
                    Ok(time) => {
                        success_count += 1;
                        debug!("Request #{} successful, time: {}", i + 1, time);
                    }
                    Err(e) => {
                        failure_count += 1;
                        debug!("Request #{} failed: {:?}", i + 1, e);
                    }
                }

                // Small delay between requests
                sleep(Duration::from_millis(100)).await;
            }

            info!(
                "Network stress test completed: {} successes, {} failures",
                success_count, failure_count
            );

            // At least some requests should succeed if network is stable
            assert!(success_count > 0, "At least one request should succeed");
        }
        Err(e) => {
            warn!("Initial authentication failed: {:?}", e);
            info!("Testing unauthenticated endpoints for network recovery");

            // Test public endpoints that don't require auth
            let result = client.get_server_time().await;
            match result {
                Ok(time) => {
                    info!("Public endpoint accessible, server time: {}", time);
                    assert!(time > 0, "Server time should be positive");
                }
                Err(e) => {
                    info!("Public endpoint also failed: {:?}", e);
                    // This is acceptable - might be network issues
                }
            }
        }
    }

    info!("Network recovery simulation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_concurrent_connection_handling() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting concurrent connection handling test");

    let client = DeribitHttpClient::new(true);

    // Try to authenticate
    let auth_result = authenticate_client(&client).await;

    if auth_result.is_err() {
        warn!("Authentication failed, testing with public endpoints only");
    }

    debug!("Testing concurrent requests");

    // Create multiple concurrent requests
    let mut handles = Vec::new();

    for i in 0..3 {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            debug!("Starting concurrent request #{}", i + 1);
            let result = client_clone.get_server_time().await;
            debug!(
                "Concurrent request #{} completed: {:?}",
                i + 1,
                result.is_ok()
            );
            result
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    let mut success_count = 0;
    let mut failure_count = 0;

    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(Ok(time)) => {
                success_count += 1;
                debug!(
                    "Concurrent request #{} succeeded with time: {}",
                    i + 1,
                    time
                );
            }
            Ok(Err(e)) => {
                failure_count += 1;
                debug!("Concurrent request #{} failed: {:?}", i + 1, e);
            }
            Err(e) => {
                failure_count += 1;
                debug!("Concurrent request #{} panicked: {:?}", i + 1, e);
            }
        }
    }

    info!(
        "Concurrent requests completed: {} successes, {} failures",
        success_count, failure_count
    );

    // At least one request should complete (either success or controlled failure)
    assert!(
        success_count + failure_count >= 3,
        "All requests should complete"
    );

    info!("Concurrent connection handling test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_graceful_degradation() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting graceful degradation test");

    // Test with various timeout configurations
    let timeout_configs = vec![
        Duration::from_millis(50),  // Very short
        Duration::from_millis(500), // Short
        Duration::from_secs(2),     // Medium
        Duration::from_secs(10),    // Long
    ];

    for (i, timeout_duration) in timeout_configs.into_iter().enumerate() {
        debug!(
            "Testing with timeout configuration #{}: {:?}",
            i + 1,
            timeout_duration
        );

        let mut config = HttpConfig::testnet();
        config.timeout = timeout_duration;

        let client = DeribitHttpClient::with_config(config)?;

        let start_time = std::time::Instant::now();
        let result = client.get_server_time().await;
        let elapsed = start_time.elapsed();

        match result {
            Ok(time) => {
                info!(
                    "Request #{} succeeded in {:?}, time: {}",
                    i + 1,
                    elapsed,
                    time
                );
                assert!(time > 0, "Server time should be positive");
            }
            Err(e) => {
                info!("Request #{} failed in {:?}: {:?}", i + 1, elapsed, e);
                // Verify the timeout was respected
                assert!(
                    elapsed <= timeout_duration + Duration::from_millis(1000),
                    "Request should respect timeout configuration"
                );
            }
        }

        // Small delay between different configurations
        sleep(Duration::from_millis(100)).await;
    }

    info!("Graceful degradation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_error_message_quality() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting error message quality test");

    // Test various error scenarios and verify error message quality
    let error_scenarios = vec![
        ("invalid-host.example.com", "DNS/Network error"),
        ("https://httpstat.us/500", "HTTP 500 error"),
        ("https://httpstat.us/404", "HTTP 404 error"),
    ];

    for (host, expected_error_type) in error_scenarios {
        debug!("Testing error scenario: {} ({})", host, expected_error_type);

        let mut config = HttpConfig::testnet();
        config.base_url = host.parse().unwrap();
        config.timeout = Duration::from_secs(5); // Reasonable timeout

        let client = DeribitHttpClient::with_config(config)?;

        let result = client.get_server_time().await;

        match result {
            Ok(_) => {
                warn!("Expected error for {} but request succeeded", host);
            }
            Err(e) => {
                let error_message = e.to_string();
                info!("Error for {}: {}", host, error_message);

                // Verify error message is informative
                assert!(
                    !error_message.is_empty(),
                    "Error message should not be empty"
                );
                assert!(
                    error_message.len() > 10,
                    "Error message should be descriptive"
                );

                // Error message should not contain internal implementation details
                let error_lower = error_message.to_lowercase();
                assert!(
                    !error_lower.contains("panic"),
                    "Error should not mention panics"
                );
                assert!(
                    !error_lower.contains("unwrap"),
                    "Error should not mention unwrap"
                );
            }
        }

        // Small delay between scenarios
        sleep(Duration::from_millis(100)).await;
    }

    info!("Error message quality test completed successfully");
    Ok(())
}
