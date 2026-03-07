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
