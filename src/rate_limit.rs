//! Rate limiting implementation for Deribit HTTP client
//!
//! This module provides automatic rate limiting to comply with Deribit API limits.
//! It implements a token bucket algorithm with different limits for different
//! endpoint categories.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Rate limiter for different endpoint categories
#[derive(Debug, Clone)]
pub struct RateLimiter {
    limiters: Arc<Mutex<HashMap<RateLimitCategory, TokenBucket>>>,
}

/// Categories of rate limits based on Deribit API documentation
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum RateLimitCategory {
    /// Trading endpoints (buy, sell, cancel, etc.)
    Trading,
    /// Market data endpoints (ticker, orderbook, etc.)
    MarketData,
    /// Account management endpoints (summary, positions, etc.)
    Account,
    /// Authentication endpoints
    Auth,
    /// General/other endpoints
    General,
}

/// Token bucket implementation for rate limiting
#[derive(Debug)]
struct TokenBucket {
    /// Maximum number of tokens
    capacity: u32,
    /// Current number of tokens
    tokens: u32,
    /// Rate of token refill (tokens per second)
    refill_rate: u32,
    /// Last refill time
    last_refill: Instant,
}

impl TokenBucket {
    /// Create a new token bucket
    fn new(capacity: u32, refill_rate: u32) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    /// Try to consume a token, returns true if successful
    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Get time until next token is available
    fn time_until_token(&self) -> Duration {
        if self.tokens > 0 {
            Duration::from_secs(0)
        } else {
            Duration::from_secs_f64(1.0 / self.refill_rate as f64)
        }
    }

    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as u32;
        
        if tokens_to_add > 0 {
            self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
            self.last_refill = now;
        }
    }
}

impl RateLimiter {
    /// Create a new rate limiter with default Deribit limits
    pub fn new() -> Self {
        let mut limiters = HashMap::new();
        
        // Based on Deribit API documentation
        // Trading: 200 requests per second with burst of 250
        limiters.insert(RateLimitCategory::Trading, TokenBucket::new(250, 200));
        
        // Market data: Higher limits for public endpoints
        limiters.insert(RateLimitCategory::MarketData, TokenBucket::new(500, 400));
        
        // Account: Moderate limits
        limiters.insert(RateLimitCategory::Account, TokenBucket::new(200, 150));
        
        // Auth: Lower limits to prevent abuse
        limiters.insert(RateLimitCategory::Auth, TokenBucket::new(50, 30));
        
        // General: Default limits
        limiters.insert(RateLimitCategory::General, TokenBucket::new(300, 200));
        
        Self {
            limiters: Arc::new(Mutex::new(limiters)),
        }
    }

    /// Wait for rate limit permission for the given category
    pub async fn wait_for_permission(&self, category: RateLimitCategory) {
        loop {
            let wait_time = {
                let mut limiters = self.limiters.lock().await;
                let bucket = limiters.get_mut(&category)
                    .expect("Rate limit category should exist");
                
                if bucket.try_consume() {
                    return; // Permission granted
                } else {
                    bucket.time_until_token()
                }
            };
            
            // Wait before trying again
            if wait_time > Duration::from_secs(0) {
                sleep(wait_time).await;
            } else {
                // Small delay to prevent busy waiting
                sleep(Duration::from_millis(10)).await;
            }
        }
    }

    /// Check if permission is available without waiting
    pub async fn check_permission(&self, category: RateLimitCategory) -> bool {
        let mut limiters = self.limiters.lock().await;
        let bucket = limiters.get_mut(&category)
            .expect("Rate limit category should exist");
        bucket.try_consume()
    }

    /// Get current token count for a category (for monitoring)
    pub async fn get_tokens(&self, category: RateLimitCategory) -> u32 {
        let mut limiters = self.limiters.lock().await;
        let bucket = limiters.get_mut(&category)
            .expect("Rate limit category should exist");
        bucket.refill();
        bucket.tokens
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to categorize endpoints
pub fn categorize_endpoint(endpoint: &str) -> RateLimitCategory {
    if endpoint.contains("/private/buy") || 
       endpoint.contains("/private/sell") || 
       endpoint.contains("/private/cancel") || 
       endpoint.contains("/private/edit") {
        RateLimitCategory::Trading
    } else if endpoint.contains("/public/ticker") || 
              endpoint.contains("/public/get_order_book") || 
              endpoint.contains("/public/get_last_trades") || 
              endpoint.contains("/public/get_instruments") {
        RateLimitCategory::MarketData
    } else if endpoint.contains("/private/get_account_summary") || 
              endpoint.contains("/private/get_positions") || 
              endpoint.contains("/private/get_subaccounts") {
        RateLimitCategory::Account
    } else if endpoint.contains("/public/auth") || 
              endpoint.contains("/private/logout") {
        RateLimitCategory::Auth
    } else {
        RateLimitCategory::General
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_token_bucket_basic() {
        let mut bucket = TokenBucket::new(10, 5);
        
        // Should be able to consume initial tokens
        for _ in 0..10 {
            assert!(bucket.try_consume());
        }
        
        // Should be empty now
        assert!(!bucket.try_consume());
    }

    #[tokio::test]
    async fn test_token_bucket_refill() {
        let mut bucket = TokenBucket::new(5, 10); // 10 tokens per second
        
        // Consume all tokens
        for _ in 0..5 {
            assert!(bucket.try_consume());
        }
        assert!(!bucket.try_consume());
        
        // Wait for refill (100ms should give us 1 token at 10/sec rate)
        sleep(Duration::from_millis(200)).await;
        
        // Should have at least 1 token now
        assert!(bucket.try_consume());
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new();
        
        // Should be able to get permission initially
        assert!(limiter.check_permission(RateLimitCategory::Trading).await);
        
        // Test waiting for permission
        limiter.wait_for_permission(RateLimitCategory::MarketData).await;
        // If we get here, the wait succeeded
    }

    #[test]
    fn test_endpoint_categorization() {
        assert_eq!(categorize_endpoint("/private/buy"), RateLimitCategory::Trading);
        assert_eq!(categorize_endpoint("/public/ticker"), RateLimitCategory::MarketData);
        assert_eq!(categorize_endpoint("/private/get_account_summary"), RateLimitCategory::Account);
        assert_eq!(categorize_endpoint("/public/auth"), RateLimitCategory::Auth);
        assert_eq!(categorize_endpoint("/public/get_time"), RateLimitCategory::General);
    }
}