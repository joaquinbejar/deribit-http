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

#[cfg(test)]
mod edit_order_by_label_tests {
    use deribit_http::DeribitHttpClient;
    use deribit_http::model::request::order::OrderRequest;
    use tokio::time::{Duration, Instant};
    use tracing::info;

    /// Test edit_order_by_label endpoint behavior
    ///
    /// Note: This test requires authentication and an existing order with a specific label.
    /// It will attempt to edit an order and verify the response structure.
    /// If no order with the label exists, the API will return an error which is expected.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication and an existing order with label"]
    async fn test_edit_order_by_label() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing edit_order_by_label");
        let start_time = Instant::now();

        let request = OrderRequest {
            order_id: None,
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: Some(150.0),
            contracts: None,
            type_: None,
            label: Some("test_order_label".to_string()),
            price: Some(50000.0),
            time_in_force: None,
            display_amount: None,
            post_only: None,
            reject_post_only: None,
            reduce_only: None,
            trigger_price: None,
            trigger_offset: None,
            trigger: None,
            advanced: None,
            mmp: None,
            valid_until: None,
            linked_order_type: None,
            trigger_fill_condition: None,
            otoco_config: None,
        };

        let result = client.edit_order_by_label(request).await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(response) => {
                info!(
                    "Edit order by label succeeded in {:?}: order_id={}, label={}",
                    elapsed, response.order.order_id, response.order.label
                );
                assert!(
                    response.order.replaced,
                    "Order should be marked as replaced"
                );
            }
            Err(e) => {
                // Expected if no order with label exists
                info!(
                    "Edit order by label failed in {:?} (expected if no order with label): {:?}",
                    elapsed, e
                );
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_edit_order_by_label completed");
        Ok(())
    }
}

#[cfg(test)]
mod get_margins_tests {
    use deribit_http::DeribitHttpClient;
    use tokio::time::{Duration, Instant};
    use tracing::info;

    /// Test get_margins endpoint behavior
    ///
    /// Returns margin requirements for a hypothetical order on a given instrument.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication"]
    async fn test_get_margins() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing get_margins");
        let start_time = Instant::now();

        let result = client.get_margins("BTC-PERPETUAL", 10000.0, 50000.0).await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(margins) => {
                info!(
                    "Get margins succeeded in {:?}: buy={}, sell={}, min_price={}, max_price={}",
                    elapsed, margins.buy, margins.sell, margins.min_price, margins.max_price
                );
                assert!(margins.buy >= 0.0, "Buy margin should be non-negative");
                assert!(margins.sell >= 0.0, "Sell margin should be non-negative");
                assert!(
                    margins.min_price > 0.0,
                    "Min price should be positive for BTC-PERPETUAL"
                );
                assert!(
                    margins.max_price > margins.min_price,
                    "Max price should be greater than min price"
                );
            }
            Err(e) => {
                info!("Get margins failed in {:?}: {:?}", elapsed, e);
                return Err(e.to_string().into());
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_get_margins completed");
        Ok(())
    }
}

#[cfg(test)]
mod mmp_tests {
    use deribit_http::DeribitHttpClient;
    use tokio::time::{Duration, Instant};
    use tracing::info;

    /// Test get_mmp_config endpoint behavior
    ///
    /// Returns MMP configuration for an index.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication"]
    async fn test_get_mmp_config() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing get_mmp_config");
        let start_time = Instant::now();

        let result = client.get_mmp_config(Some("btc_usd"), None, None).await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(configs) => {
                info!(
                    "Get MMP config succeeded in {:?}: {} configs found",
                    elapsed,
                    configs.len()
                );
            }
            Err(e) => {
                info!("Get MMP config failed in {:?}: {:?}", elapsed, e);
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_get_mmp_config completed");
        Ok(())
    }

    /// Test get_mmp_status endpoint behavior
    ///
    /// Returns MMP status for triggered indexes.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication"]
    async fn test_get_mmp_status() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing get_mmp_status");
        let start_time = Instant::now();

        let result = client.get_mmp_status(None, None, None).await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(statuses) => {
                info!(
                    "Get MMP status succeeded in {:?}: {} statuses found",
                    elapsed,
                    statuses.len()
                );
            }
            Err(e) => {
                info!("Get MMP status failed in {:?}: {:?}", elapsed, e);
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_get_mmp_status completed");
        Ok(())
    }
}
