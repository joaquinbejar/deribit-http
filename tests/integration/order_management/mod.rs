//! Order management integration tests
//!
//! This module contains integration tests for trading and order management functionality,
//! including buy/sell orders, order cancellation, order history, and order modifications.

// Note: Order management tests can be added here as needed
// These tests cover private trading endpoints that require authentication

#[cfg(test)]
mod close_position_tests {
    use deribit_http::DeribitHttpClient;
    use tokio::time::{Duration, Instant};
    use tracing::info;

    /// Test close_position endpoint behavior
    ///
    /// Note: This test requires authentication and an open position.
    /// It will attempt to close a position and verify the response structure.
    /// If no position exists, the API will return an error which is expected.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication and may affect real positions"]
    async fn test_close_position_market_order() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing close_position with market order");
        let start_time = Instant::now();
        let result = client.close_position("BTC-PERPETUAL", "market", None).await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(response) => {
                info!(
                    "Close position succeeded in {:?}: order_id={}, state={}",
                    elapsed, response.order.order_id, response.order.order_state
                );
                // Verify reduce_only is true (required for close_position)
                assert!(
                    response.order.reduce_only,
                    "Close position order should be reduce_only"
                );
            }
            Err(e) => {
                // Expected if no position exists
                info!(
                    "Close position failed in {:?} (expected if no position): {:?}",
                    elapsed, e
                );
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_close_position_market_order completed");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication and may affect real positions"]
    async fn test_close_position_limit_order() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing close_position with limit order");
        let start_time = Instant::now();
        // Use a price far from market to avoid accidental execution
        let result = client
            .close_position("BTC-PERPETUAL", "limit", Some(1.0))
            .await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(response) => {
                info!(
                    "Close position (limit) succeeded in {:?}: order_id={}, type={}",
                    elapsed, response.order.order_id, response.order.order_type
                );
                assert_eq!(response.order.order_type, "limit");
                assert!(response.order.reduce_only);
            }
            Err(e) => {
                info!(
                    "Close position (limit) failed in {:?} (expected if no position): {:?}",
                    elapsed, e
                );
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_close_position_limit_order completed");
        Ok(())
    }
}
