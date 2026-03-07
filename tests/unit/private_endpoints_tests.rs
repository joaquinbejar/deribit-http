//! Unit tests for private endpoints

use deribit_http::DeribitHttpClient;
use deribit_http::config::HttpConfig;
use deribit_http::model::transaction::TransactionLogRequest;
use serde_json::json;
use std::env;
use url::Url;

// Helper function to create a test client
fn create_test_client(server: &mockito::ServerGuard) -> DeribitHttpClient {
    unsafe {
        env::set_var("DERIBIT_CLIENT_ID", "test_client_id");
        env::set_var("DERIBIT_CLIENT_SECRET", "test_client_secret");
        env::set_var("DERIBIT_TESTNET", "true");
    }

    let config = HttpConfig {
        base_url: Url::parse(&format!("{}/api/v2", server.url())).unwrap(),
        ..Default::default()
    };

    DeribitHttpClient::with_config(config)
}

// Helper function to create OAuth2 authentication mock
async fn create_auth_mock(server: &mut mockito::Server) -> mockito::Mock {
    server
        .mock("GET", "/api/v2/public/auth?grant_type=client_credentials&client_id=test_client_id&client_secret=test_client_secret")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "access_token": "test_access_token",
                "expires_in": 3600,
                "refresh_token": "test_refresh_token",
                "scope": "read",
                "state": "",
                "token_type": "bearer"
            }
        }"#)
        .create_async()
        .await
}

#[tokio::test]
async fn test_get_subaccounts_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/get_subaccounts?with_portfolio=true")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": [
                {
                    "email": "test@example.com",
                    "id": 1,
                    "is_password": true,
                    "login_enabled": true,
                    "portfolio_margining_enabled": false,
                    "receive_notifications": true,
                    "system_name": "test_user",
                    "tfa_enabled": false,
                    "type": "main",
                    "username": "test_user"
                }
            ]
        }"#,
        )
        .create_async()
        .await;

    let result = client.get_subaccounts(Some(true)).await;
    assert!(result.is_ok());
    let subaccounts = result.unwrap();
    assert_eq!(subaccounts.len(), 1);
    assert_eq!(subaccounts[0].username, "test_user");

    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_subaccounts_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/get_subaccounts")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32602,
                "message": "Invalid params"
            }
        }"#,
        )
        .create_async()
        .await;

    let result = client.get_subaccounts(None).await;
    assert!(result.is_err());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_transaction_log_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "logs": [
                {
                    "id": 12345,
                    "currency": "BTC",
                    "amount": 0.001,
                    "balance": 1.5,
                    "timestamp": 1609459200000u64,
                    "type": "trade",
                    "change": 0.001,
                    "cashflow": 0.001,
                    "user_id": 1,
                    "user_seq": 1,
                    "equity": 1.5,
                    "username": "test_user"
                }
            ],
            "continuation": null
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/get_transaction_log?currency=BTC&start_timestamp=1609459200000&end_timestamp=1609459300000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let request = TransactionLogRequest {
        currency: "BTC".to_string(),
        start_timestamp: 1609459200000,
        end_timestamp: 1609459300000,
        query: None,
        count: None,
        subaccount_id: None,
        continuation: None,
    };
    let result = client.get_transaction_log(request).await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error in test_get_transaction_log_success: {:?}", e);
    }
    assert!(result.is_ok());
    let log_response = result.unwrap();
    assert_eq!(log_response.logs.len(), 1);
    assert_eq!(log_response.logs[0].currency, "BTC");
}

#[tokio::test]
async fn test_get_transaction_log_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/get_transaction_log?currency=BTC&start_timestamp=1609459200000&end_timestamp=1609459300000")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32602,
                "message": "Invalid params"
            }
        }"#)
        .create_async()
        .await;

    let request = TransactionLogRequest {
        currency: "BTC".to_string(),
        start_timestamp: 1609459200000,
        end_timestamp: 1609459300000,
        query: None,
        count: None,
        subaccount_id: None,
        continuation: None,
    };
    let result = client.get_transaction_log(request).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_deposits_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "count": 1,
            "data": [
                {
                    "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                    "amount": 0.001,
                    "currency": "BTC",
                    "state": "completed",
                    "received_timestamp": 1609459200000u64,
                    "transaction_id": "abc123",
                    "updated_timestamp": 1609459200000u64
                }
            ]
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/get_deposits?currency=BTC")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_deposits("BTC", None, None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_withdrawals_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "count": 1,
            "data": [
                {
                    "id": 123,
                    "currency": "BTC",
                    "amount": 0.001,
                    "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                    "state": "completed",
                    "created_timestamp": 1609459200000u64,
                    "fee": 0.0001,
                    "priority": "normal"
                }
            ]
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/get_withdrawals?currency=BTC")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_withdrawals("BTC", None, None).await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error in test_get_withdrawals_success: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_submit_transfer_to_subaccount_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "id": "12345",
            "status": "ok"
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/submit_transfer_to_subaccount?currency=BTC&amount=0.001&destination=123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .submit_transfer_to_subaccount("BTC", 0.001, 123)
        .await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!(
            "Error in test_submit_transfer_to_subaccount_success: {:?}",
            e
        );
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_submit_transfer_to_user_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "id": "12345",
            "status": "ok"
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/submit_transfer_to_user?currency=BTC&amount=0.001&destination=test_user")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .submit_transfer_to_user("BTC", 0.001, "test_user")
        .await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error in test_submit_transfer_to_user_success: {:?}", e);
    }
    assert!(result.is_ok());
}

// =========================================================================
// Close Position Tests (Issue #13)
// =========================================================================

#[tokio::test]
async fn test_close_position_market_order_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "order": {
                "amount": 10.0,
                "api": true,
                "average_price": 50000.0,
                "creation_timestamp": 1609459200000u64,
                "direction": "sell",
                "filled_amount": 10.0,
                "instrument_name": "BTC-PERPETUAL",
                "is_liquidation": false,
                "label": "",
                "last_update_timestamp": 1609459200000u64,
                "order_id": "ETH-123456",
                "order_state": "filled",
                "order_type": "market",
                "post_only": false,
                "price": 50000.0,
                "reduce_only": true,
                "replaced": false,
                "risk_reducing": false,
                "time_in_force": "good_til_cancelled",
                "web": false
            },
            "trades": [
                {
                    "trade_id": "BTC-12345",
                    "instrument_name": "BTC-PERPETUAL",
                    "timestamp": 1609459200000u64,
                    "direction": "sell",
                    "price": 50000.0,
                    "amount": 10.0,
                    "fee": 0.0001,
                    "fee_currency": "BTC",
                    "order_id": "ETH-123456",
                    "order_type": "market",
                    "trade_seq": 1,
                    "state": "filled",
                    "index_price": 50000.0,
                    "liquidity": "T",
                    "mark_price": 50000.0,
                    "tick_direction": 0,
                    "self_trade": false,
                    "label": ""
                }
            ]
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/close_position?instrument_name=BTC-PERPETUAL&type=market",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.close_position("BTC-PERPETUAL", "market", None).await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error in test_close_position_market_order_success: {:?}", e);
    }
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.order.instrument_name, "BTC-PERPETUAL");
    assert!(response.order.reduce_only);
}

#[tokio::test]
async fn test_close_position_limit_order_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "order": {
                "amount": 10.0,
                "api": true,
                "average_price": 2500.0,
                "creation_timestamp": 1609459200000u64,
                "direction": "sell",
                "filled_amount": 10.0,
                "instrument_name": "ETH-PERPETUAL",
                "is_liquidation": false,
                "label": "",
                "last_update_timestamp": 1609459200000u64,
                "order_id": "ETH-789012",
                "order_state": "open",
                "order_type": "limit",
                "post_only": false,
                "price": 2500.0,
                "reduce_only": true,
                "replaced": false,
                "risk_reducing": false,
                "time_in_force": "good_til_cancelled",
                "web": false
            },
            "trades": []
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/close_position?instrument_name=ETH-PERPETUAL&type=limit&price=2500",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .close_position("ETH-PERPETUAL", "limit", Some(2500.0))
        .await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error in test_close_position_limit_order_success: {:?}", e);
    }
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.order.instrument_name, "ETH-PERPETUAL");
    assert_eq!(response.order.order_type, "limit");
    assert!(response.order.reduce_only);
}

#[tokio::test]
async fn test_close_position_no_position_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/close_position?instrument_name=BTC-PERPETUAL&type=market",
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": 10041,
                "message": "no_open_position"
            }
        }"#,
        )
        .create_async()
        .await;

    let result = client.close_position("BTC-PERPETUAL", "market", None).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

// =========================================================================
// Edit Order By Label Tests (Issue #14)
// =========================================================================

#[tokio::test]
async fn test_edit_order_by_label_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "order": {
                "amount": 150.0,
                "api": true,
                "average_price": 0.0,
                "creation_timestamp": 1616155547764u64,
                "direction": "buy",
                "filled_amount": 0.0,
                "instrument_name": "BTC-PERPETUAL",
                "is_liquidation": false,
                "label": "i_love_deribit",
                "last_update_timestamp": 1616155550773u64,
                "max_show": 150.0,
                "order_id": "94166",
                "order_state": "open",
                "order_type": "limit",
                "post_only": false,
                "price": 50111.0,
                "reduce_only": false,
                "replaced": true,
                "risk_reducing": false,
                "time_in_force": "good_til_cancelled",
                "web": false
            },
            "trades": []
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/edit_by_label?label=i_love_deribit&instrument_name=BTC-PERPETUAL&amount=150&price=50111",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let request = deribit_http::model::request::order::OrderRequest {
        order_id: None,
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(150.0),
        contracts: None,
        type_: None,
        label: Some("i_love_deribit".to_string()),
        price: Some(50111.0),
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

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error in test_edit_order_by_label_success: {:?}", e);
    }
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.order.instrument_name, "BTC-PERPETUAL");
    assert_eq!(response.order.label, "i_love_deribit");
    assert!(response.order.replaced);
}

#[tokio::test]
async fn test_edit_order_by_label_missing_label_error() {
    let server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let request = deribit_http::model::request::order::OrderRequest {
        order_id: None,
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(150.0),
        contracts: None,
        type_: None,
        label: None, // Missing label
        price: Some(50111.0),
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

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("label is required"));
}

#[tokio::test]
async fn test_edit_order_by_label_no_order_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/edit_by_label?label=nonexistent_label&instrument_name=BTC-PERPETUAL&amount=150",
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": 11044,
                "message": "no_order_with_label"
            }
        }"#,
        )
        .create_async()
        .await;

    let request = deribit_http::model::request::order::OrderRequest {
        order_id: None,
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(150.0),
        contracts: None,
        type_: None,
        label: Some("nonexistent_label".to_string()),
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

    let result = client.edit_order_by_label(request).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

// =========================================================================
// Get Margins Tests (Issue #15)
// =========================================================================

#[tokio::test]
async fn test_get_margins_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "buy": 0.0219949,
            "sell": 0.0,
            "min_price": 3684.8,
            "max_price": 3759.24
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/get_margins?instrument_name=BTC-PERPETUAL&amount=10000&price=3725",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_margins("BTC-PERPETUAL", 10000.0, 3725.0).await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error in test_get_margins_success: {:?}", e);
    }
    assert!(result.is_ok());
    let margins = result.unwrap();
    assert!((margins.buy - 0.0219949).abs() < 0.0001);
    assert!((margins.sell - 0.0).abs() < 0.0001);
    assert!((margins.min_price - 3684.8).abs() < 0.1);
    assert!((margins.max_price - 3759.24).abs() < 0.1);
}

#[tokio::test]
async fn test_get_margins_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    // Mock the OAuth2 authentication endpoint
    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/get_margins?instrument_name=INVALID&amount=10000&price=3725",
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": 10001,
                "message": "instrument_not_found"
            }
        }"#,
        )
        .create_async()
        .await;

    let result = client.get_margins("INVALID", 10000.0, 3725.0).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

// =========================================================================
// MMP Endpoints Tests (Issue #16)
// =========================================================================

#[tokio::test]
async fn test_get_mmp_config_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "index_name": "btc_usd",
                "mmp_group": "MassQuoteBot7",
                "interval": 60,
                "frozen_time": 0,
                "quantity_limit": 0.5,
                "delta_limit": 0.3,
                "vega_limit": 0.1,
                "max_quote_quantity": 0.4
            }
        ],
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/get_mmp_config?index_name=btc_usd")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_mmp_config(Some("btc_usd"), None, None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].index_name, "btc_usd");
    assert_eq!(configs[0].interval, 60);
}

#[tokio::test]
async fn test_get_mmp_status_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "index_name": "btc_usd",
                "frozen_until": 1744275841861u64,
                "mmp_group": "MassQuoteBot7"
            }
        ],
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/get_mmp_status?index_name=btc_usd")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_mmp_status(Some("btc_usd"), None, None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].index_name, "btc_usd");
    assert_eq!(statuses[0].frozen_until, 1744275841861);
}

#[tokio::test]
async fn test_set_mmp_config_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "index_name": "btc_usd",
            "mmp_group": "MassQuoteBot7",
            "interval": 60,
            "frozen_time": 0,
            "quantity_limit": 3.0,
            "max_quote_quantity": 2.5
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/set_mmp_config\?.*index_name=btc_usd.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let request = deribit_http::model::response::mmp::SetMmpConfigRequest {
        index_name: "btc_usd".to_string(),
        interval: 60,
        frozen_time: 0,
        quantity_limit: Some(3.0),
        delta_limit: None,
        vega_limit: None,
        max_quote_quantity: Some(2.5),
        mmp_group: Some("MassQuoteBot7".to_string()),
        block_rfq: None,
    };

    let result = client.set_mmp_config(request).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.index_name, "btc_usd");
    assert_eq!(config.interval, 60);
}

#[tokio::test]
async fn test_reset_mmp_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": "ok",
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/reset_mmp?index_name=btc_usd")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.reset_mmp("btc_usd", None, None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "ok");
}

// =========================================================================
// Get Order Margin By IDs Tests (Issue #17)
// =========================================================================

#[tokio::test]
async fn test_get_order_margin_by_ids_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "order_id": "ETH-349278",
                "initial_margin": 0.00091156,
                "initial_margin_currency": "ETH"
            },
            {
                "order_id": "ETH-349279",
                "initial_margin": 0.0,
                "initial_margin_currency": "ETH"
            }
        ],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/api/v2/private/get_order_margin_by_ids\?ids=.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .get_order_margin_by_ids(&["ETH-349278", "ETH-349279"])
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let margins = result.unwrap();
    assert_eq!(margins.len(), 2);
    assert_eq!(margins[0].order_id, "ETH-349278");
    assert!((margins[0].initial_margin - 0.00091156).abs() < 0.0001);
    assert_eq!(margins[0].initial_margin_currency, "ETH");
}

#[tokio::test]
async fn test_get_order_margin_by_ids_empty_ids_error() {
    let server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let result = client.get_order_margin_by_ids(&[]).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("ids array cannot be empty"));
}

// =========================================================================
// Get Order State By Label Tests (Issue #18)
// =========================================================================

#[tokio::test]
async fn test_get_order_state_by_label_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "time_in_force": "good_til_cancelled",
                "reduce_only": false,
                "price": 118.94,
                "post_only": false,
                "order_type": "limit",
                "order_state": "filled",
                "order_id": "ETH-331562",
                "max_show": 37.0,
                "last_update_timestamp": 1550219810944u64,
                "label": "fooBar",
                "is_liquidation": false,
                "instrument_name": "ETH-PERPETUAL",
                "filled_amount": 37.0,
                "direction": "sell",
                "creation_timestamp": 1550219749176u64,
                "average_price": 118.94,
                "api": false,
                "amount": 37.0,
                "replaced": false,
                "risk_reducing": false,
                "web": false
            }
        ],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/get_order_state_by_label?currency=ETH&label=fooBar",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_order_state_by_label("ETH", "fooBar").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let orders = result.unwrap();
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].order_id, "ETH-331562");
    assert_eq!(orders[0].order_state, "filled");
    assert_eq!(orders[0].label, "fooBar");
}

#[tokio::test]
async fn test_get_order_state_by_label_empty_result() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/get_order_state_by_label?currency=BTC&label=nonexistent",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_order_state_by_label("BTC", "nonexistent").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let orders = result.unwrap();
    assert!(orders.is_empty());
}

// =========================================================================
// Get Settlement History By Currency Tests (Issue #19)
// =========================================================================

#[tokio::test]
async fn test_get_settlement_history_by_currency_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "settlements": [
                {
                    "type": "settlement",
                    "timestamp": 1550475692526i64,
                    "session_profit_loss": 0.038358299,
                    "profit_loss": -0.001783937,
                    "position": -66.0,
                    "mark_price": 121.67,
                    "instrument_name": "ETH-22FEB19",
                    "index_price": 119.8
                }
            ],
            "continuation": "xY7T6cusbMBNpH9SNmKb94jXSBxUPojJEdCPL4YociHBUgAhWQvEP"
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_settlement_history_by_currency\?currency=BTC.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .get_settlement_history_by_currency("BTC", Some("settlement"), Some(1), None, None)
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.settlements.len(), 1);
    assert!(response.continuation.is_some());
}

// =========================================================================
// Get Settlement History By Instrument Tests (Issue #19)
// =========================================================================

#[tokio::test]
async fn test_get_settlement_history_by_instrument_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "settlements": [
                {
                    "type": "settlement",
                    "timestamp": 1550475692526i64,
                    "session_profit_loss": 0.038358299,
                    "profit_loss": -0.001783937,
                    "position": -66.0,
                    "mark_price": 121.67,
                    "instrument_name": "ETH-22FEB19",
                    "index_price": 119.8
                }
            ],
            "continuation": "xY7T6cusbMBNpH9SNmKb94jXSBxUPojJEdCPL4YociHBUgAhWQvEP"
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_settlement_history_by_instrument\?instrument_name=.*"
                    .to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .get_settlement_history_by_instrument(
            "ETH-22FEB19",
            Some("settlement"),
            Some(1),
            None,
            None,
        )
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.settlements.len(), 1);
    assert!(response.continuation.is_some());
}

// =========================================================================
// Get Trigger Order History Tests (Issue #20)
// =========================================================================

#[tokio::test]
async fn test_get_trigger_order_history_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "entries": [
                {
                    "timestamp": 1555918941451i64,
                    "trigger": "index_price",
                    "trigger_price": 5285.0,
                    "trigger_order_id": "SLIS-103",
                    "order_id": "671473",
                    "order_state": "triggered",
                    "instrument_name": "BTC-PERPETUAL",
                    "request": "trigger:order",
                    "direction": "buy",
                    "price": 5179.28,
                    "amount": 10.0
                }
            ],
            "continuation": "1555918941451.SLIS-103"
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_trigger_order_history\?currency=BTC.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .get_trigger_order_history("BTC", None, Some(10), None)
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.entries.len(), 1);
    assert!(response.continuation.is_some());
    assert_eq!(response.entries[0].trigger_order_id, "SLIS-103");
    assert_eq!(response.entries[0].direction, "buy");
}

#[tokio::test]
async fn test_get_trigger_order_history_empty() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "entries": []
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_trigger_order_history\?currency=ETH.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .get_trigger_order_history("ETH", Some("ETH-PERPETUAL"), None, None)
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.entries.is_empty());
    assert!(response.continuation.is_none());
}

// =========================================================================
// Move Positions Tests (Issue #21)
// =========================================================================

#[tokio::test]
async fn test_move_positions_success() {
    use deribit_http::model::request::position::MovePositionTrade;

    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "target_uid": 23,
                "source_uid": 3,
                "price": 35800.0,
                "instrument_name": "BTC-PERPETUAL",
                "direction": "buy",
                "amount": 110.0
            },
            {
                "target_uid": 23,
                "source_uid": 3,
                "price": 0.1223,
                "instrument_name": "BTC-28JAN22-32500-C",
                "direction": "sell",
                "amount": 0.1
            }
        ],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/api/v2/private/move_positions\?currency=BTC.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let trades = vec![
        MovePositionTrade::with_price("BTC-PERPETUAL", 110.0, 35800.0),
        MovePositionTrade::new("BTC-28JAN22-32500-C", 0.1),
    ];

    let result = client.move_positions("BTC", 3, 23, &trades).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let results = result.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].instrument_name, "BTC-PERPETUAL");
    assert_eq!(results[0].direction, "buy");
    assert_eq!(results[0].amount, 110.0);
    assert_eq!(results[1].instrument_name, "BTC-28JAN22-32500-C");
    assert_eq!(results[1].direction, "sell");
}

#[tokio::test]
async fn test_move_positions_empty() {
    use deribit_http::model::request::position::MovePositionTrade;

    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/api/v2/private/move_positions\?currency=ETH.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let trades = vec![MovePositionTrade::new("ETH-PERPETUAL", 10.0)];

    let result = client.move_positions("ETH", 1, 2, &trades).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let results = result.unwrap();
    assert!(results.is_empty());
}

// =========================================================================
// Get Account Summaries Tests (Issue #22)
// =========================================================================

#[tokio::test]
async fn test_get_account_summaries_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "id": 10,
            "email": "user@example.com",
            "system_name": "user",
            "username": "user",
            "block_rfq_self_match_prevention": true,
            "creation_timestamp": 1687352432143i64,
            "type": "main",
            "referrer_id": null,
            "login_enabled": true,
            "security_keys_enabled": false,
            "mmp_enabled": false,
            "interuser_transfers_enabled": false,
            "self_trading_reject_mode": "cancel_maker",
            "self_trading_extended_to_subaccounts": false,
            "summaries": [
                {
                    "currency": "BTC",
                    "balance": 302.60065765,
                    "equity": 302.61869214,
                    "available_funds": 301.38059622,
                    "available_withdrawal_funds": 301.35396172,
                    "initial_margin": 1.24669592,
                    "maintenance_margin": 0.8857841,
                    "margin_balance": 302.62729214
                },
                {
                    "currency": "ETH",
                    "balance": 100.0,
                    "equity": 100.0,
                    "available_funds": 99.999598,
                    "available_withdrawal_funds": 99.999597,
                    "initial_margin": 0.000402,
                    "maintenance_margin": 0.0,
                    "margin_balance": 100.0
                }
            ]
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/api/v2/private/get_account_summaries.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_account_summaries(None, Some(true)).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.summaries.len(), 2);
    assert_eq!(response.email, "user@example.com");
    assert_eq!(response.username, "user");
}

#[tokio::test]
async fn test_get_account_summaries_with_subaccount() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "id": 20,
            "email": "subaccount@example.com",
            "system_name": "subuser",
            "username": "subuser",
            "block_rfq_self_match_prevention": false,
            "creation_timestamp": 1687352432143i64,
            "type": "subaccount",
            "referrer_id": null,
            "login_enabled": false,
            "security_keys_enabled": false,
            "mmp_enabled": false,
            "interuser_transfers_enabled": false,
            "self_trading_reject_mode": "cancel_maker",
            "self_trading_extended_to_subaccounts": false,
            "summaries": [
                {
                    "currency": "BTC",
                    "balance": 10.0,
                    "equity": 10.0,
                    "available_funds": 10.0,
                    "available_withdrawal_funds": 10.0,
                    "initial_margin": 0.0,
                    "maintenance_margin": 0.0,
                    "margin_balance": 10.0
                }
            ]
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_account_summaries\?subaccount_id=20.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_account_summaries(Some(20), None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.summaries.len(), 1);
    assert_eq!(response.account_type, "subaccount");
}

// =========================================================================
// Get Subaccounts Details Tests (Issue #23)
// =========================================================================

#[tokio::test]
async fn test_get_subaccounts_details_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "uid": 3,
                "positions": [
                    {
                        "total_profit_loss": -0.000118183,
                        "size_currency": 0.004152776,
                        "size": 200,
                        "settlement_price": 48150.36,
                        "realized_profit_loss": -8.79e-7,
                        "realized_funding": -8.8e-7,
                        "open_orders_margin": 0,
                        "mark_price": 48160.55,
                        "maintenance_margin": 0.000089286,
                        "leverage": 34,
                        "kind": "future",
                        "instrument_name": "BTC-PERPETUAL",
                        "initial_margin": 0.000122508,
                        "index_price": 47897.12,
                        "floating_profit_loss": -0.00003451,
                        "estimated_liquidation_price": 2.33,
                        "direction": "buy",
                        "delta": 0.004152776,
                        "average_price": 49571.3
                    }
                ]
            },
            {
                "uid": 10,
                "positions": [
                    {
                        "total_profit_loss": 0.000037333,
                        "size_currency": -0.001308984,
                        "size": -60,
                        "settlement_price": 47886.98,
                        "realized_profit_loss": 0,
                        "open_orders_margin": 0,
                        "mark_price": 45837.07,
                        "maintenance_margin": 0.000028143,
                        "leverage": 34,
                        "kind": "future",
                        "instrument_name": "BTC-3SEP21",
                        "initial_margin": 0.000038615,
                        "index_price": 47897.12,
                        "floating_profit_loss": 0.000037333,
                        "estimated_liquidation_price": null,
                        "direction": "sell",
                        "delta": -0.001308984,
                        "average_price": 47182.76
                    }
                ]
            }
        ],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_subaccounts_details\?currency=BTC.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_subaccounts_details("BTC", None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let details = result.unwrap();
    assert_eq!(details.len(), 2);
    assert_eq!(details[0].uid, 3);
    assert_eq!(details[0].positions.len(), 1);
    assert_eq!(details[0].positions[0].instrument_name, "BTC-PERPETUAL");
    assert_eq!(details[1].uid, 10);
}

#[tokio::test]
async fn test_get_subaccounts_details_empty() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_subaccounts_details\?currency=ETH.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_subaccounts_details("ETH", Some(true)).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let details = result.unwrap();
    assert!(details.is_empty());
}

// =========================================================================
// Subaccount Management Tests (Issue #25)
// =========================================================================

#[tokio::test]
async fn test_create_subaccount_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "email": "user_AAA@email.com",
            "id": 13,
            "is_password": false,
            "login_enabled": false,
            "receive_notifications": false,
            "system_name": "user_1_4",
            "security_keys_enabled": false,
            "type": "subaccount",
            "username": "user_1_4"
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/create_subaccount")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.create_subaccount().await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let subaccount = result.unwrap();
    assert_eq!(subaccount.id, 13);
    assert_eq!(subaccount.username, "user_1_4");
    assert!(!subaccount.login_enabled);
}

#[tokio::test]
async fn test_create_subaccount_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/create_subaccount")
        .with_status(403)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 13009, "message": "not_main_account"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client.create_subaccount().await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_remove_subaccount_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": "ok",
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/remove_subaccount?subaccount_id=123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.remove_subaccount(123).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "ok");
}

#[tokio::test]
async fn test_remove_subaccount_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/remove_subaccount?subaccount_id=999")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 13004, "message": "subaccount_not_found"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client.remove_subaccount(999).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_change_subaccount_name_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": "ok",
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/change_subaccount_name?sid=7&name=new_user_name",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.change_subaccount_name(7, "new_user_name").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "ok");
}

#[tokio::test]
async fn test_change_subaccount_name_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/change_subaccount_name?sid=999&name=invalid")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 13004, "message": "subaccount_not_found"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client.change_subaccount_name(999, "invalid").await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_toggle_subaccount_login_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": "ok",
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/toggle_subaccount_login?sid=7&state=enable",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.toggle_subaccount_login(7, "enable").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "ok");
}

#[tokio::test]
async fn test_toggle_subaccount_login_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/toggle_subaccount_login?sid=999&state=enable")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 13004, "message": "subaccount_not_found"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client.toggle_subaccount_login(999, "enable").await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_set_email_for_subaccount_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": "ok",
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/set_email_for_subaccount\?sid=7&email=.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.set_email_for_subaccount(7, "user@example.com").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "ok");
}

#[tokio::test]
async fn test_set_email_for_subaccount_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/set_email_for_subaccount\?sid=999&email=.*".to_string(),
            ),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 13004, "message": "subaccount_not_found"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client
        .set_email_for_subaccount(999, "invalid@example.com")
        .await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_toggle_notifications_from_subaccount_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": "ok",
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/toggle_notifications_from_subaccount?sid=7&state=true",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.toggle_notifications_from_subaccount(7, true).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "ok");
}

#[tokio::test]
async fn test_toggle_notifications_from_subaccount_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/toggle_notifications_from_subaccount?sid=999&state=false")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 13004, "message": "subaccount_not_found"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client
        .toggle_notifications_from_subaccount(999, false)
        .await;

    mock.assert_async().await;
    assert!(result.is_err());
}

// =========================================================================
// Transfer Endpoints Tests (Issue #28)
// =========================================================================

#[tokio::test]
async fn test_get_transfers_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "count": 1,
            "data": [{
                "id": 2,
                "created_timestamp": 1550579457727_i64,
                "updated_timestamp": 1550579457727_i64,
                "currency": "BTC",
                "amount": 0.2,
                "direction": "payment",
                "other_side": "new_user_1_1",
                "state": "confirmed",
                "type": "subaccount"
            }]
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/get_transfers?currency=BTC")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_transfers("BTC", None, None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let transfers = result.unwrap();
    assert_eq!(transfers.count, 1);
    assert_eq!(transfers.len(), 1);
    assert!(!transfers.is_empty());
    assert_eq!(transfers.data[0].id, 2);
    assert_eq!(transfers.data[0].currency, "BTC");
    assert!((transfers.data[0].amount - 0.2).abs() < f64::EPSILON);
    assert_eq!(transfers.data[0].other_side, "new_user_1_1");
}

#[tokio::test]
async fn test_get_transfers_with_pagination() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "count": 0,
            "data": []
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/get_transfers?currency=ETH&count=5&offset=10",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_transfers("ETH", Some(5), Some(10)).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let transfers = result.unwrap();
    assert_eq!(transfers.count, 0);
    assert!(transfers.is_empty());
}

#[tokio::test]
async fn test_get_transfers_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/get_transfers?currency=INVALID")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"jsonrpc": "2.0", "error": {"code": 10001, "message": "invalid_currency"}, "id": 1}"#,
        )
        .create_async()
        .await;

    let result = client.get_transfers("INVALID", None, None).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_cancel_transfer_by_id_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "id": 123,
            "created_timestamp": 1550579457727_i64,
            "updated_timestamp": 1550579457800_i64,
            "currency": "BTC",
            "amount": 0.5,
            "direction": "payment",
            "other_side": "subaccount_1",
            "state": "cancelled",
            "type": "subaccount"
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/cancel_transfer_by_id?currency=BTC&id=123",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.cancel_transfer_by_id("BTC", 123).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let transfer = result.unwrap();
    assert_eq!(transfer.id, 123);
    assert!(transfer.is_cancelled());
}

#[tokio::test]
async fn test_cancel_transfer_by_id_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock("GET", "/api/v2/private/cancel_transfer_by_id?currency=BTC&id=999")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"jsonrpc": "2.0", "error": {"code": 10003, "message": "transfer_not_found"}, "id": 1}"#,
        )
        .create_async()
        .await;

    let result = client.cancel_transfer_by_id("BTC", 999).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_submit_transfer_between_subaccounts_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "id": 456,
            "created_timestamp": 1550579457727_i64,
            "updated_timestamp": 1550579457727_i64,
            "currency": "ETH",
            "amount": 12.1234,
            "direction": "payment",
            "other_side": "subaccount_20",
            "state": "confirmed",
            "type": "subaccount"
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/submit_transfer_between_subaccounts?currency=ETH&amount=12.1234&destination=20",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .submit_transfer_between_subaccounts("ETH", 12.1234, 20, None)
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let transfer = result.unwrap();
    assert_eq!(transfer.id, 456);
    assert_eq!(transfer.currency, "ETH");
    assert!((transfer.amount - 12.1234).abs() < f64::EPSILON);
    assert!(transfer.is_confirmed());
    assert!(transfer.is_payment());
}

#[tokio::test]
async fn test_submit_transfer_between_subaccounts_with_source() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "id": 789,
            "created_timestamp": 1550579457727_i64,
            "updated_timestamp": 1550579457727_i64,
            "currency": "BTC",
            "amount": 1.0,
            "direction": "payment",
            "other_side": "subaccount_20",
            "state": "confirmed",
            "type": "subaccount"
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/submit_transfer_between_subaccounts?currency=BTC&amount=1&destination=20&source=10",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .submit_transfer_between_subaccounts("BTC", 1.0, 20, Some(10))
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let transfer = result.unwrap();
    assert_eq!(transfer.id, 789);
}

#[tokio::test]
async fn test_submit_transfer_between_subaccounts_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/submit_transfer_between_subaccounts?currency=BTC&amount=1000000&destination=999",
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"jsonrpc": "2.0", "error": {"code": 10004, "message": "insufficient_funds"}, "id": 1}"#,
        )
        .create_async()
        .await;

    let result = client
        .submit_transfer_between_subaccounts("BTC", 1000000.0, 999, None)
        .await;

    mock.assert_async().await;
    assert!(result.is_err());
}

// ============================================================================
// Block RFQ endpoint tests
// ============================================================================

#[tokio::test]
async fn test_get_block_rfqs_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "continuation": null,
            "block_rfqs": [
                {
                    "block_rfq_id": 508,
                    "state": "open",
                    "role": "maker",
                    "amount": 40000,
                    "combo_id": "BTC-15NOV24",
                    "legs": [
                        {
                            "direction": "sell",
                            "instrument_name": "BTC-15NOV24",
                            "ratio": 1
                        }
                    ],
                    "creation_timestamp": 1731062457741i64,
                    "expiration_timestamp": 1731062757741i64,
                    "taker_rating": "1-2"
                }
            ]
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "/api/v2/private/get_block_rfqs?count=20&state=open&role=maker",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .get_block_rfqs(
            Some(20),
            Some(deribit_http::model::response::BlockRfqState::Open),
            Some(deribit_http::model::response::BlockRfqRole::Maker),
            None,
            None,
            None,
        )
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response.block_rfqs[0].block_rfq_id, 508);
}

#[tokio::test]
async fn test_cancel_block_rfq_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "block_rfq_id": 366,
            "state": "cancelled",
            "role": "taker",
            "amount": 100000,
            "combo_id": "BTC-FS-1NOV24_PERP",
            "legs": [
                {
                    "ratio": 1,
                    "instrument_name": "BTC-1NOV24",
                    "direction": "sell"
                }
            ],
            "creation_timestamp": 1729855159611i64,
            "expiration_timestamp": 1729855459611i64,
            "bids": [],
            "asks": [],
            "makers": []
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/cancel_block_rfq?block_rfq_id=366")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.cancel_block_rfq(366).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let rfq = result.unwrap();
    assert_eq!(rfq.block_rfq_id, 366);
    assert!(rfq.is_cancelled());
}

#[tokio::test]
async fn test_get_block_rfq_quotes_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "block_rfq_quote_id": 8,
                "block_rfq_id": 1,
                "quote_state": "open",
                "price": 74600,
                "amount": 20000,
                "direction": "buy",
                "legs": [
                    {
                        "direction": "buy",
                        "price": 74600,
                        "instrument_name": "BTC-15NOV24",
                        "ratio": 1
                    }
                ],
                "creation_timestamp": 1731076586371i64,
                "last_update_timestamp": 1731076638591i64,
                "replaced": false,
                "filled_amount": 0,
                "execution_instruction": "all_or_none"
            }
        ],
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/get_block_rfq_quotes?block_rfq_id=1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_block_rfq_quotes(Some(1), None, None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let quotes = result.unwrap();
    assert_eq!(quotes.len(), 1);
    assert_eq!(quotes[0].block_rfq_quote_id, 8);
    assert!(quotes[0].is_open());
}

#[tokio::test]
async fn test_cancel_all_block_rfq_quotes_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [],
        "id": 1
    });

    let mock = server
        .mock("GET", "/api/v2/private/cancel_all_block_rfq_quotes")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.cancel_all_block_rfq_quotes().await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let quotes = result.unwrap();
    assert!(quotes.is_empty());
}
