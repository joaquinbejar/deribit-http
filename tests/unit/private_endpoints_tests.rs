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
