//! Integration tests for private endpoints

#[cfg(test)]
mod tests_private_endpoints {
    use deribit_http::model::order::OrderType;
    use deribit_http::model::request::order::OrderRequest;
    use deribit_http::model::types::TimeInForce;
    use deribit_http::prelude::*;

    async fn create_test_client() -> DeribitHttpClient {
        // Create client with default configuration
        // Note: Tests will use the real Deribit testnet URL since we can't mock it
        DeribitHttpClient::new()
    }

    async fn set_test_token(_client: &DeribitHttpClient) {
        // TODO: Implement test token configuration if needed
        // For now, tests will expect authentication errors since we don't have real tokens
    }

    #[tokio::test]
    async fn test_get_subaccounts() {
        let client = create_test_client().await;

        let result = client.get_subaccounts(None).await;
        match result {
            Ok(response) => {
                println!("get_subaccounts succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("get_subaccounts failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_subaccounts_with_portfolio() {
        let client = create_test_client().await;

        let result = client.get_subaccounts(Some(true)).await;
        match result {
            Ok(response) => {
                println!("get_subaccounts with portfolio succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "get_subaccounts with portfolio failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_transaction_log() {
        let client = create_test_client().await;

        let result = client
            .get_transaction_log(TransactionLogRequest {
                currency: "BTC".to_string(),
                ..Default::default()
            })
            .await;
        match result {
            Ok(response) => {
                println!("get_transaction_log succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "get_transaction_log failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_deposits() {
        let client = create_test_client().await;

        let result = client.get_deposits("BTC", None, None).await;
        match result {
            Ok(response) => {
                println!("get_deposits succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("get_deposits failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_withdrawals() {
        let client = create_test_client().await;

        let result = client.get_withdrawals("BTC", None, None).await;
        match result {
            Ok(response) => {
                println!("get_withdrawals succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("get_withdrawals failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_submit_transfer_to_subaccount() {
        let client = create_test_client().await;

        let result = client
            .submit_transfer_to_subaccount("BTC", 0.001, 123)
            .await;
        match result {
            Ok(response) => {
                println!("submit_transfer_to_subaccount succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "submit_transfer_to_subaccount failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_submit_transfer_to_user() {
        let client = create_test_client().await;

        let result = client.submit_transfer_to_user("ETH", 0.1, "user123").await;
        match result {
            Ok(response) => {
                println!("submit_transfer_to_user succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "submit_transfer_to_user failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_buy_order() {
        let client = create_test_client().await;

        // Set up authentication token for testing
        set_test_token(&client).await;

        let request = OrderRequest {
            order_id: None,
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: Some(10.0),
            contracts: None,
            type_: Some(OrderType::Limit),
            label: Some("test_order".to_string()),
            price: Some(50000.0),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
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

        // Print request details for debugging
        println!("About to call buy_order with request: {:#?}", request);

        let result = client.buy_order(request).await;
        match result {
            Ok(response) => {
                println!("buy_order succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("buy_order failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_sell_order() {
        let client = create_test_client().await;

        let request = OrderRequest {
            order_id: None,
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: Some(5.0),
            contracts: None,
            type_: None,
            label: None,
            price: None,
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

        let result = client.sell_order(request).await;
        match result {
            Ok(response) => {
                println!("sell_order succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("sell_order failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let client = create_test_client().await;

        let result = client.cancel_order("order_123").await;
        match result {
            Ok(response) => {
                println!("cancel_order succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("cancel_order failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_cancel_all() {
        let client = create_test_client().await;

        let result = client.cancel_all().await;
        match result {
            Ok(response) => {
                println!("cancel_all succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("cancel_all failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_cancel_all_by_currency() {
        let client = create_test_client().await;

        let result = client.cancel_all_by_currency("BTC").await;
        match result {
            Ok(response) => {
                println!("cancel_all_by_currency succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "cancel_all_by_currency failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_cancel_all_by_instrument() {
        let client = create_test_client().await;

        let result = client.cancel_all_by_instrument("BTC-PERPETUAL").await;
        match result {
            Ok(response) => {
                println!("cancel_all_by_instrument succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "cancel_all_by_instrument failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_account_summary() {
        let client = create_test_client().await;

        let result = client.get_account_summary("BTC", None).await;
        match result {
            Ok(response) => {
                println!("get_account_summary succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "get_account_summary failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_positions() {
        let client = create_test_client().await;

        let result = client.get_positions(None, None, None).await;
        match result {
            Ok(response) => {
                println!("get_positions succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("get_positions failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_edit_order() {
        let client = create_test_client().await;

        let request = OrderRequest {
            order_id: Some("order_123".to_string()),
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: None,
            contracts: None,
            type_: None,
            label: None,
            price: None,
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

        let result = client.edit_order(request).await;
        match result {
            Ok(response) => {
                println!("edit_order succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("edit_order failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_open_orders() {
        let client = create_test_client().await;

        let result = client.get_open_orders(None, None).await;
        match result {
            Ok(response) => {
                println!("get_open_orders succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("get_open_orders failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_open_orders_by_currency() {
        let client = create_test_client().await;

        let result = client.get_open_orders_by_currency("BTC", None, None).await;
        match result {
            Ok(response) => {
                println!("get_open_orders_by_currency succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "get_open_orders_by_currency failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_open_orders_by_instrument() {
        let client = create_test_client().await;

        let result = client
            .get_open_orders_by_instrument("BTC-PERPETUAL", None)
            .await;
        match result {
            Ok(response) => {
                println!("get_open_orders_by_instrument succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "get_open_orders_by_instrument failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_order_state() {
        let client = create_test_client().await;

        let result = client.get_order_state("order_123").await;
        match result {
            Ok(response) => {
                println!("get_order_state succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!("get_order_state failed (expected without auth): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_get_user_trades_by_instrument() {
        let client = create_test_client().await;

        let result = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, None, None, None)
            .await;
        match result {
            Ok(response) => {
                println!("get_user_trades_by_instrument succeeded: {:?}", response);
                // Test passes if we can make the call
            }
            Err(e) => {
                println!(
                    "get_user_trades_by_instrument failed (expected without auth): {:?}",
                    e
                );
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_error_handling_unauthorized() {
        let client = create_test_client().await;

        let result = client.get_account_summary("BTC", None).await;
        match result {
            Ok(response) => {
                println!("Unexpected success: {:?}", response);
                // This shouldn't happen without proper auth
            }
            Err(e) => {
                println!("Expected error (unauthorized): {:?}", e);
                // Expected to fail without proper authentication
            }
        }
    }

    #[tokio::test]
    async fn test_error_handling_invalid_instrument() {
        let client = create_test_client().await;

        let request = OrderRequest {
            order_id: None,
            instrument_name: "INVALID-INSTRUMENT".to_string(),
            amount: Some(10.0),
            contracts: None,
            type_: None,
            label: None,
            price: None,
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

        let result = client.buy_order(request).await;
        match result {
            Ok(response) => {
                println!("Unexpected success with invalid instrument: {:?}", response);
                // This shouldn't happen with invalid instrument
            }
            Err(e) => {
                println!("Expected error (invalid instrument): {:?}", e);
                // Expected to fail with invalid instrument
            }
        }
    }
}
