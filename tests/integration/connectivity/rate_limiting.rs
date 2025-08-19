//! Rate Limiting Integration Tests
//!
//! This test covers rate limiting scenarios:
//! 1. Rate limit detection and handling
//! 2. Automatic backoff and retry
//! 3. Rate limit recovery
//! 4. Burst request handling
//! 5. Rate limit compliance verification

use std::path::Path;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};

use deribit_http::*;
use deribit_http::{DeribitHttpClient, HttpConfig};

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with authentication credentials".into());
    }

    dotenv::dotenv().ok();

    let has_oauth2 = std::env::var("DERIBIT_CLIENT_ID").is_ok() && std::env::var("DERIBIT_CLIENT_SECRET").is_ok();
    let has_api_key = std::env::var("DERIBIT_API_KEY").is_ok() && std::env::var("DERIBIT_API_SECRET").is_ok();
    
    if !has_oauth2 && !has_api_key {
        return Err("Missing authentication credentials".into());
    }

    Ok(())
}

/// Authenticate client using available credentials
async fn authenticate_client(client: &DeribitHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    if let (Ok(client_id), Ok(client_secret)) = (std::env::var("DERIBIT_CLIENT_ID"), std::env::var("DERIBIT_CLIENT_SECRET")) {
        client.authenticate_oauth2(&client_id, &client_secret).await?;
    } else if let (Ok(api_key), Ok(api_secret)) = (std::env::var("DERIBIT_API_KEY"), std::env::var("DERIBIT_API_SECRET")) {
        client.authenticate_api_key(&api_key, &api_secret).await?;
    } else {
        return Err("No valid authentication credentials found".into());
    }
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_rate_limit_compliance() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting rate limit compliance test");

    let client = DeribitHttpClient::new(true);
    
    // Test sequential requests with proper spacing
    let num_requests = 10;
    let min_interval = Duration::from_millis(100); // Minimum interval between requests
    let mut request_times = Vec::new();
    
    debug!("Testing {} sequential requests with proper spacing", num_requests);
    
    for i in 0..num_requests {
        let start_time = Instant::now();
        
        debug!("Making request #{}", i + 1);
        let result = client.get_server_time().await;
        
        let elapsed = start_time.elapsed();
        request_times.push((start_time, elapsed, result.is_ok()));
        
        match result {
            Ok(time) => {
                debug!("Request #{} succeeded in {:?}, time: {}", i + 1, elapsed, time);
            }
            Err(e) => {
                debug!("Request #{} failed in {:?}: {:?}", i + 1, elapsed, e);
                
                // Check if it's a rate limit error
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("rate") || error_str.contains("limit") || error_str.contains("429") {
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
        let interval = request_times[i].0.duration_since(request_times[i-1].0);
        intervals.push(interval);
    }
    
    let avg_interval = if !intervals.is_empty() {
        Duration::from_nanos(intervals.iter().map(|d| d.as_nanos()).sum::<u128>() / intervals.len() as u128)
    } else {
        Duration::from_secs(0)
    };
    
    info!("Average interval between requests: {:?}", avg_interval);
    
    // Count successes and failures
    let success_count = request_times.iter().filter(|(_, _, success)| *success).count();
    let failure_count = request_times.len() - success_count;
    
    info!("Rate limit compliance test completed: {} successes, {} failures", success_count, failure_count);
    
    // At least some requests should succeed with proper spacing
    assert!(success_count > 0, "At least some requests should succeed with proper rate limiting");
    
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_burst_request_handling() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting burst request handling test");

    let client = DeribitHttpClient::new(true);
    
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
        
        debug!("Burst request #{} completed in {:?}", i + 1, request_elapsed);
    }
    
    let total_burst_time = burst_start.elapsed();
    info!("Burst of {} requests completed in {:?}", burst_size, total_burst_time);
    
    // Analyze burst results
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut rate_limit_count = 0;
    
    for (request_num, result, elapsed) in burst_results {
        match result {
            Ok(time) => {
                success_count += 1;
                debug!("Burst request #{} succeeded in {:?}, time: {}", request_num, elapsed, time);
            }
            Err(e) => {
                failure_count += 1;
                debug!("Burst request #{} failed in {:?}: {:?}", request_num, elapsed, e);
                
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("rate") || error_str.contains("limit") || error_str.contains("429") {
                    rate_limit_count += 1;
                    info!("Rate limit hit on burst request #{}", request_num);
                }
            }
        }
    }
    
    info!("Burst results: {} successes, {} failures, {} rate limits", 
          success_count, failure_count, rate_limit_count);
    
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
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting rate limit backoff strategy test");

    let client = DeribitHttpClient::new(true);
    
    // Test exponential backoff pattern
    let backoff_intervals = vec![
        Duration::from_millis(100),
        Duration::from_millis(200),
        Duration::from_millis(400),
        Duration::from_millis(800),
        Duration::from_millis(1600),
    ];
    
    for (attempt, backoff_duration) in backoff_intervals.into_iter().enumerate() {
        debug!("Backoff attempt #{} with interval {:?}", attempt + 1, backoff_duration);
        
        let start_time = Instant::now();
        let result = client.get_server_time().await;
        let elapsed = start_time.elapsed();
        
        match result {
            Ok(time) => {
                info!("Backoff attempt #{} succeeded in {:?}, time: {}", 
                      attempt + 1, elapsed, time);
                // If successful, we can break the backoff pattern
                break;
            }
            Err(e) => {
                info!("Backoff attempt #{} failed in {:?}: {:?}", 
                      attempt + 1, elapsed, e);
                
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("rate") || error_str.contains("limit") || error_str.contains("429") {
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
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting rate limit with different endpoints test");

    let client = DeribitHttpClient::new(true);
    
    // Try to authenticate (may fail, but we'll test what we can)
    let auth_result = authenticate_client(&client).await;
    
    // Test different endpoint categories
    let endpoint_tests = vec![
        ("Public - Server Time", || async { client.get_server_time().await.map(|t| t.to_string()) }),
        ("Public - Instruments", || async { 
            client.get_instruments("BTC", None, None).await.map(|i| format!("{} instruments", i.len()))
        }),
    ];
    
    // Add authenticated endpoints if auth succeeded
    let mut all_tests = endpoint_tests;
    if auth_result.is_ok() {
        info!("Authentication successful, testing authenticated endpoints");
        // Note: These closures would need to be properly constructed for authenticated endpoints
        // For now, we'll focus on public endpoints
    } else {
        info!("Authentication failed, testing public endpoints only");
    }
    
    for (endpoint_name, test_fn) in all_tests {
        debug!("Testing rate limits for: {}", endpoint_name);
        
        // Test multiple requests to this endpoint
        let mut endpoint_results = Vec::new();
        
        for i in 0..3 {
            let start_time = Instant::now();
            let result = test_fn().await;
            let elapsed = start_time.elapsed();
            
            endpoint_results.push((i + 1, result, elapsed));
            
            match result {
                Ok(response) => {
                    debug!("{} request #{} succeeded in {:?}: {}", 
                           endpoint_name, i + 1, elapsed, response);
                }
                Err(e) => {
                    debug!("{} request #{} failed in {:?}: {:?}", 
                           endpoint_name, i + 1, elapsed, e);
                }
            }
            
            // Small delay between requests to same endpoint
            sleep(Duration::from_millis(200)).await;
        }
        
        // Analyze results for this endpoint
        let success_count = endpoint_results.iter().filter(|(_, result, _)| result.is_ok()).count();
        let failure_count = endpoint_results.len() - success_count;
        
        info!("{} results: {} successes, {} failures", endpoint_name, success_count, failure_count);
        
        // Delay before testing next endpoint category
        sleep(Duration::from_millis(500)).await;
    }
    
    info!("Rate limit with different endpoints test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_rate_limit_recovery_time() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting rate limit recovery time test");

    let client = DeribitHttpClient::new(true);
    
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
                if error_str.contains("rate") || error_str.contains("limit") || error_str.contains("429") {
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
        
        for (attempt, wait_duration) in recovery_intervals.into_iter().enumerate() {
            debug!("Testing recovery after waiting {:?}", wait_duration);
            sleep(wait_duration).await;
            
            let recovery_start = Instant::now();
            let result = client.get_server_time().await;
            let recovery_elapsed = recovery_start.elapsed();
            
            match result {
                Ok(time) => {
                    info!("Recovery successful after {:?} wait (request took {:?}), time: {}", 
                          wait_duration, recovery_elapsed, time);
                    break;
                }
                Err(e) => {
                    let error_str = e.to_string().to_lowercase();
                    if error_str.contains("rate") || error_str.contains("limit") || error_str.contains("429") {
                        info!("Rate limit still active after {:?} wait", wait_duration);
                    } else {
                        info!("Recovery attempt failed with non-rate-limit error: {:?}", e);
                        break;
                    }
                }
            }
        }
    } else {
        info!("Rate limit not triggered - either limits are generous or requests were spaced enough");
        
        // Test normal operation timing
        let mut normal_timings = Vec::new();
        
        for i in 0..5 {
            let start_time = Instant::now();
            let result = client.get_server_time().await;
            let elapsed = start_time.elapsed();
            
            normal_timings.push(elapsed);
            
            match result {
                Ok(time) => {
                    debug!("Normal request #{} succeeded in {:?}, time: {}", i + 1, elapsed, time);
                }
                Err(e) => {
                    debug!("Normal request #{} failed in {:?}: {:?}", i + 1, elapsed, e);
                }
            }
            
            sleep(Duration::from_millis(200)).await;
        }
        
        let avg_timing = if !normal_timings.is_empty() {
            Duration::from_nanos(normal_timings.iter().map(|d| d.as_nanos()).sum::<u128>() / normal_timings.len() as u128)
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
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting rate limit error handling test");

    let client = DeribitHttpClient::new(true);
    
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
                let error_category = if error_str.contains("rate") || error_str.contains("limit") || error_str.contains("429") {
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
                
                debug!("Request #{} failed with {} error: {:?}", i + 1, error_category, e);
                
                // If it's a rate limit error, test the error details
                if error_category == "rate_limit" {
                    info!("Rate limit error detected: {:?}", e);
                    
                    // Verify error message quality
                    assert!(!error_str.is_empty(), "Rate limit error message should not be empty");
                    assert!(error_str.len() > 5, "Rate limit error message should be descriptive");
                    
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