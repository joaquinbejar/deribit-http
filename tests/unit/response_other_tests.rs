use deribit_http::model::response::other::*;
use deribit_http::model::settlement::Settlement;
use deribit_http::model::trade::{LastTrade, UserTrade};
use deribit_http::model::transaction::TransactionLogEntry;
use deribit_http::model::other::DeliveryPriceData;
use deribit_http::model::fee::FeeStructure;
use serde_json;

fn create_mock_user_trade() -> UserTrade {
    UserTrade {
        amount: 1.0,
        api: Some(true),
        contracts: Some(10.0),
        direction: "buy".to_string(),
        fee: 0.25,
        fee_currency: "BTC".to_string(),
        index_price: 50000.0,
        instrument_name: "BTC-PERPETUAL".to_string(),
        iv: None,
        label: Some("test_label".to_string()),
        liquidity: "T".to_string(),
        mark_price: 50100.0,
        matching_id: Some("match123".to_string()),
        mmp: Some(false),
        order_id: "67890".to_string(),
        order_type: "limit".to_string(),
        original_order_type: Some("limit".to_string()),
        post_only: Some(false),
        price: 50000.0,
        profit_loss: Some(100.0),
        reduce_only: Some(false),
        risk_reducing: Some(false),
        self_trade: false,
        state: "filled".to_string(),
        tick_direction: 1,
        timestamp: 1234567890,
        trade_id: "trade_123".to_string(),
        trade_seq: 123456,
        underlying_price: Some(49900.0),
        user_id: Some(12345),
    }
}

fn create_mock_delivery_price_data() -> DeliveryPriceData {
    DeliveryPriceData {
        date: "2022-01-01".to_string(),
        delivery_price: 50000.0,
    }
}

// Mock data creation functions
fn create_mock_last_trade() -> LastTrade {
    LastTrade {
        amount: 1.0,
        direction: "buy".to_string(),
        index_price: 49900.0,
        instrument_name: "BTC-PERPETUAL".to_string(),
        iv: None,
        liquid: Some("T".to_string()),
        price: 50000.0,
        tick_direction: 1,
        timestamp: 1234567890,
        trade_id: "trade_123".to_string(),
        trade_seq: 123456,
    }
}

fn create_mock_settlement() -> Settlement {
    use deribit_http::model::settlement::SettlementType;
    Settlement {
        settlement_type: SettlementType::Settlement,
        timestamp: 1640995200000,
        instrument_name: Some("BTC-PERPETUAL".to_string()),
        position_size: Some(1.5),
        mark_price: Some(50000.0),
        index_price: Some(49900.0),
        profit_loss: None,
        funding: Some(0.01),
        session_profit_loss: Some(100.0),
        session_bankrupt_cy: Some(0.0),
        session_tax: Some(0.0),
        session_tax_rate: Some(0.0),
        socialized_losses: Some(0.0),
        additional_fields: std::collections::HashMap::new(),
    }
}

fn create_mock_transaction_log_entry() -> TransactionLogEntry {
    TransactionLogEntry {
        id: 12345,
        currency: "BTC".to_string(),
        amount: Some(100.0),
        balance: 1095.0,
        timestamp: 1640995200000,
        transaction_type: "deposit".to_string(),
        info: Some(serde_json::json!({"test": "data"})),
        change: 100.0,
        cashflow: 95.0,
        user_id: 12345,
        trade_id: Some("trade_456".to_string()),
        order_id: Some("order_789".to_string()),
        position: Some(1.5),
        side: None,
        contracts: Some(1.0),
        interest_pl: Some(0.0),
        user_role: None,
        fee_role: Some("maker".to_string()),
        index_price: Some(44900.0),
        price: Some(45000.0),
        user_seq: 1001,
        settlement_price: Some(45100.0),
        price_currency: Some("USD".to_string()),
        equity: 5000.0,
        total_interest_pl: Some(0.0),
        session_upl: Some(50.0),
        profit_as_cashflow: Some(false),
        commission: Some(0.5),
        session_rpl: Some(25.0),
        mark_price: Some(44950.0),
        block_rfq_id: Some(123),
        ip: Some("192.168.1.1".to_string()),
        username: "user123".to_string(),
        instrument_name: Some("BTC-PERPETUAL".to_string()),
    }
}

fn create_mock_fee_structure() -> FeeStructure {
    use deribit_http::model::fee::{FeeValue, DefaultFee};
    FeeStructure {
        index_name: "BTC-PERPETUAL".to_string(),
        kind: "perpetual".to_string(),
        value: FeeValue {
            default: DefaultFee {
                fee_type: "relative".to_string(),
                taker: 0.0005,
                maker: 0.0001,
            },
            block_trade: Some(0.0003),
            settlement: Some(0.0),
        },
    }
}

fn create_mock_account_limits() -> AccountLimits {
    AccountLimits {
        limits_per_currency: false,
        non_matching_engine: RateLimit {
            burst: 10,
            rate: 5,
        },
        matching_engine: MatchingEngineLimit {
            trading: TradingLimit {
                total: RateLimit {
                    burst: 100,
                    rate: 50,
                },
            },
            spot: RateLimit {
                burst: 20,
                rate: 10,
            },
            quotes: RateLimit {
                burst: 15,
                rate: 8,
            },
            max_quotes: RateLimit {
                burst: 25,
                rate: 12,
            },
            guaranteed_quotes: RateLimit {
                burst: 5,
                rate: 2,
            },
            cancel_all: RateLimit {
                burst: 10,
                rate: 5,
            },
        },
    }
}

fn create_mock_account_result() -> AccountResult {
    AccountResult {
        currency: "BTC".to_string(),
        balance: 1.5,
        equity: 1.6,
        available_funds: 1.4,
        margin_balance: 1.5,
        total_pl: Some(0.1),
        session_rpl: Some(0.05),
        session_upl: Some(0.05),
        maintenance_margin: 0.1,
        initial_margin: 0.15,
        available_withdrawal_funds: Some(1.3),
        cross_collateral_enabled: Some(false),
        delta_total: Some(1.0),
        futures_pl: Some(0.08),
        futures_session_rpl: Some(0.04),
        futures_session_upl: Some(0.04),
        options_delta: Some(0.2),
        options_gamma: Some(0.01),
        options_pl: Some(0.02),
        options_session_rpl: Some(0.01),
        options_session_upl: Some(0.01),
        options_theta: Some(-0.005),
        options_vega: Some(0.1),
        portfolio_margining_enabled: Some(false),
        projected_delta_total: Some(0.9),
        projected_initial_margin: Some(0.14),
        projected_maintenance_margin: Some(0.09),
        delta_total_map: Some(std::collections::HashMap::new()),
        deposit_address: Some("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string()),
        fees: Some(vec![create_mock_fee_structure()]),
        limits: Some(create_mock_account_limits()),
        margin_model: Some("cross_collateral".to_string()),
        options_gamma_map: Some(std::collections::HashMap::new()),
        options_theta_map: Some(std::collections::HashMap::new()),
        options_vega_map: Some(std::collections::HashMap::new()),
        options_value: Some(0.5),
        spot_reserve: Some(0.1),
        estimated_liquidation_ratio: Some(0.8),
        estimated_liquidation_ratio_map: Some(std::collections::HashMap::new()),
        fee_balance: Some(0.001),
        additional_reserve: Some(0.05),
        has_non_block_chain_equity: Some(false),
        total_margin_balance_usd: Some(75000.0),
        total_delta_total_usd: Some(50000.0),
        total_initial_margin_usd: Some(7500.0),
        total_maintenance_margin_usd: Some(5000.0),
        total_equity_usd: Some(80000.0),
        system_name: Some("user_12345".to_string()),
        account_type: Some("main".to_string()),
    }
}

// Tests for LastTradesResponse
#[test]
fn test_last_trades_response_creation() {
    let trades = vec![create_mock_last_trade()];
    let response = LastTradesResponse {
        has_more: false,
        trades,
    };
    
    assert!(!response.has_more);
    assert_eq!(response.trades.len(), 1);
    assert_eq!(response.trades[0].trade_id, "trade_123");
}

#[test]
fn test_last_trades_response_serialization() {
    let trades = vec![create_mock_last_trade()];
    let response = LastTradesResponse {
        has_more: true,
        trades,
    };
    
    let serialized = serde_json::to_string(&response).unwrap();
    assert!(serialized.contains("has_more"));
    assert!(serialized.contains("trades"));
    assert!(serialized.contains("trade_123"));
}

#[test]
fn test_last_trades_response_deserialization() {
    let json = r#"{
        "has_more": false,
        "trades": [{
            "trade_seq": 12345,
            "trade_id": "trade_123",
            "timestamp": 1640995200000,
            "tick_direction": 1,
            "price": 50000.0,
            "mark_price": 50100.0,
            "instrument_name": "BTC-PERPETUAL",
            "index_price": 49900.0,
            "direction": "buy",
            "amount": 1.5
        }]
    }"#;
    
    let response: LastTradesResponse = serde_json::from_str(json).unwrap();
    assert!(!response.has_more);
    assert_eq!(response.trades.len(), 1);
    assert_eq!(response.trades[0].trade_id, "trade_123");
}

#[test]
fn test_last_trades_response_clone() {
    let trades = vec![create_mock_last_trade()];
    let response = LastTradesResponse {
        has_more: true,
        trades,
    };
    
    let cloned = response.clone();
    assert_eq!(response.has_more, cloned.has_more);
    assert_eq!(response.trades.len(), cloned.trades.len());
}

// Tests for SettlementsResponse
#[test]
fn test_settlements_response_new() {
    let settlements = vec![create_mock_settlement()];
    let response = SettlementsResponse::new(settlements);
    
    assert!(response.continuation.is_none());
    assert_eq!(response.settlements.len(), 1);
    assert!(!response.has_more());
}

#[test]
fn test_settlements_response_with_continuation() {
    let settlements = vec![create_mock_settlement()];
    let continuation = "next_page_token".to_string();
    let response = SettlementsResponse::with_continuation(settlements, continuation.clone());
    
    assert_eq!(response.continuation, Some(continuation));
    assert_eq!(response.settlements.len(), 1);
    assert!(response.has_more());
}

#[test]
fn test_settlements_response_has_more() {
    let settlements = vec![create_mock_settlement()];
    
    // Without continuation
    let response1 = SettlementsResponse::new(settlements.clone());
    assert!(!response1.has_more());
    
    // With continuation
    let response2 = SettlementsResponse::with_continuation(settlements, "token".to_string());
    assert!(response2.has_more());
}

#[test]
fn test_settlements_response_serialization() {
    let settlements = vec![create_mock_settlement()];
    let response = SettlementsResponse::with_continuation(settlements, "token".to_string());
    
    let serialized = serde_json::to_string(&response).unwrap();
    assert!(serialized.contains("continuation"));
    assert!(serialized.contains("settlements"));
    assert!(serialized.contains("token"));
}

#[test]
fn test_settlements_response_clone() {
    let settlements = vec![create_mock_settlement()];
    let response = SettlementsResponse::with_continuation(settlements, "token".to_string());
    
    let cloned = response.clone();
    assert_eq!(response.continuation, cloned.continuation);
    assert_eq!(response.settlements.len(), cloned.settlements.len());
}

// Tests for TransactionLogResponse
#[test]
fn test_transaction_log_response_default() {
    let response = TransactionLogResponse::default();
    
    assert!(response.continuation.is_none());
    assert!(response.logs.is_empty());
}

#[test]
fn test_transaction_log_response_with_data() {
    let logs = vec![create_mock_transaction_log_entry()];
    let response = TransactionLogResponse {
        continuation: Some(12345),
        logs,
    };
    
    assert_eq!(response.continuation, Some(12345));
    assert_eq!(response.logs.len(), 1);
    assert_eq!(response.logs[0].username, "user123");
}

#[test]
fn test_transaction_log_response_serialization() {
    let logs = vec![create_mock_transaction_log_entry()];
    let response = TransactionLogResponse {
        continuation: Some(12345),
        logs,
    };
    
    let serialized = serde_json::to_string(&response).unwrap();
    assert!(serialized.contains("continuation"));
    assert!(serialized.contains("logs"));
    assert!(serialized.contains("user123"));
}

#[test]
fn test_transaction_log_response_clone() {
    let logs = vec![create_mock_transaction_log_entry()];
    let response = TransactionLogResponse {
        continuation: Some(12345),
        logs,
    };
    
    let cloned = response.clone();
    assert_eq!(response.continuation, cloned.continuation);
    assert_eq!(response.logs.len(), cloned.logs.len());
}

// Tests for TransferResultResponse
#[test]
fn test_transfer_result_response_creation() {
    let response = TransferResultResponse {
        id: "transfer_123".to_string(),
        status: "completed".to_string(),
    };
    
    assert_eq!(response.id, "transfer_123");
    assert_eq!(response.status, "completed");
}

#[test]
fn test_transfer_result_response_serialization() {
    let response = TransferResultResponse {
        id: "transfer_123".to_string(),
        status: "pending".to_string(),
    };
    
    let serialized = serde_json::to_string(&response).unwrap();
    assert!(serialized.contains("transfer_123"));
    assert!(serialized.contains("pending"));
}

#[test]
fn test_transfer_result_response_deserialization() {
    let json = r#"{
        "id": "transfer_456",
        "status": "failed"
    }"#;
    
    let response: TransferResultResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, "transfer_456");
    assert_eq!(response.status, "failed");
}

#[test]
fn test_transfer_result_response_clone() {
    let response = TransferResultResponse {
        id: "transfer_789".to_string(),
        status: "processing".to_string(),
    };
    
    let cloned = response.clone();
    assert_eq!(response.id, cloned.id);
    assert_eq!(response.status, cloned.status);
}

// Tests for AccountSummaryResponse
#[test]
fn test_account_summary_response_creation() {
    let summaries = vec![create_mock_account_result()];
    let response = AccountSummaryResponse {
        id: 12345,
        email: "user@example.com".to_string(),
        system_name: "user_12345".to_string(),
        username: "testuser".to_string(),
        block_rfq_self_match_prevention: false,
        creation_timestamp: 1640995200000,
        account_type: "main".to_string(),
        referrer_id: Some("ref_123".to_string()),
        login_enabled: true,
        security_keys_enabled: false,
        mmp_enabled: false,
        interuser_transfers_enabled: true,
        self_trading_reject_mode: "reject_taker".to_string(),
        self_trading_extended_to_subaccounts: false,
        summaries,
    };
    
    assert_eq!(response.id, 12345);
    assert_eq!(response.email, "user@example.com");
    assert_eq!(response.summaries.len(), 1);
    assert!(response.login_enabled);
}

#[test]
fn test_account_summary_response_serialization() {
    let summaries = vec![create_mock_account_result()];
    let response = AccountSummaryResponse {
        id: 12345,
        email: "user@example.com".to_string(),
        system_name: "user_12345".to_string(),
        username: "testuser".to_string(),
        block_rfq_self_match_prevention: false,
        creation_timestamp: 1640995200000,
        account_type: "main".to_string(),
        referrer_id: None,
        login_enabled: true,
        security_keys_enabled: false,
        mmp_enabled: false,
        interuser_transfers_enabled: true,
        self_trading_reject_mode: "reject_taker".to_string(),
        self_trading_extended_to_subaccounts: false,
        summaries,
    };
    
    let serialized = serde_json::to_string(&response).unwrap();
    assert!(serialized.contains("user@example.com"));
    assert!(serialized.contains("testuser"));
    assert!(serialized.contains("summaries"));
    assert!(serialized.contains("type"));
}

#[test]
fn test_account_summary_response_clone() {
    let summaries = vec![create_mock_account_result()];
    let response = AccountSummaryResponse {
        id: 12345,
        email: "user@example.com".to_string(),
        system_name: "user_12345".to_string(),
        username: "testuser".to_string(),
        block_rfq_self_match_prevention: false,
        creation_timestamp: 1640995200000,
        account_type: "main".to_string(),
        referrer_id: None,
        login_enabled: true,
        security_keys_enabled: false,
        mmp_enabled: false,
        interuser_transfers_enabled: true,
        self_trading_reject_mode: "reject_taker".to_string(),
        self_trading_extended_to_subaccounts: false,
        summaries,
    };
    
    let cloned = response.clone();
    assert_eq!(response.id, cloned.id);
    assert_eq!(response.email, cloned.email);
    assert_eq!(response.summaries.len(), cloned.summaries.len());
}

// Tests for AccountResult
#[test]
fn test_account_result_creation() {
    let result = create_mock_account_result();
    
    assert_eq!(result.currency, "BTC");
    assert_eq!(result.balance, 1.5);
    assert_eq!(result.equity, 1.6);
    assert!(result.cross_collateral_enabled.is_some());
}

#[test]
fn test_account_result_serialization() {
    let result = create_mock_account_result();
    
    let serialized = serde_json::to_string(&result).unwrap();
    assert!(serialized.contains("BTC"));
    assert!(serialized.contains("balance"));
    assert!(serialized.contains("equity"));
    assert!(serialized.contains("margin_balance"));
}

#[test]
fn test_account_result_clone() {
    let result = create_mock_account_result();
    
    let cloned = result.clone();
    assert_eq!(result.currency, cloned.currency);
    assert_eq!(result.balance, cloned.balance);
    assert_eq!(result.equity, cloned.equity);
}

// Tests for FeeStructure
#[test]
fn test_fee_structure_creation() {
    let fee = create_mock_fee_structure();
    
    assert_eq!(fee.index_name, "BTC-PERPETUAL");
    assert_eq!(fee.kind, "perpetual");
    assert_eq!(fee.value.default.taker, 0.0005);
}

#[test]
fn test_fee_structure_serialization() {
    let fee = create_mock_fee_structure();
    
    let serialized = serde_json::to_string(&fee).unwrap();
    assert!(serialized.contains("BTC-PERPETUAL"));
    assert!(serialized.contains("perpetual"));
    assert!(serialized.contains("0.0005"));
}

// Tests for AccountLimits
#[test]
fn test_account_limits_creation() {
    let limits = create_mock_account_limits();
    
    assert_eq!(limits.non_matching_engine.burst, 10);
    assert_eq!(limits.matching_engine.spot.rate, 10);
}

#[test]
fn test_account_limits_serialization() {
    let limits = create_mock_account_limits();
    
    let serialized = serde_json::to_string(&limits).unwrap();
    assert!(serialized.contains("non_matching_engine"));
    assert!(serialized.contains("matching_engine"));
}