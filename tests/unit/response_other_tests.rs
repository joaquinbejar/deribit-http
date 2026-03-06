use deribit_http::model::fee::FeeStructure;

use deribit_http::model::response::other::*;
use deribit_http::model::settlement::Settlement;
use deribit_http::model::trade::LastTrade;
use deribit_http::model::transaction::TransactionLogEntry;
use serde_json;

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
    use deribit_http::model::fee::{DefaultFee, FeeValue};
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
        non_matching_engine: RateLimit { burst: 10, rate: 5 },
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
            quotes: RateLimit { burst: 15, rate: 8 },
            max_quotes: RateLimit {
                burst: 25,
                rate: 12,
            },
            guaranteed_quotes: RateLimit { burst: 5, rate: 2 },
            cancel_all: RateLimit { burst: 10, rate: 5 },
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

// Tests for MarkPriceHistoryPoint
#[test]
fn test_mark_price_history_point_creation() {
    let point = MarkPriceHistoryPoint {
        timestamp: 1608142381229,
        mark_price: 0.5165791606037885,
    };

    assert_eq!(point.timestamp, 1608142381229);
    assert!((point.mark_price - 0.5165791606037885).abs() < f64::EPSILON);
}

#[test]
fn test_mark_price_history_point_from_tuple() {
    let tuple: (u64, f64) = (1608142381229, 0.5165791606037885);
    let point = MarkPriceHistoryPoint::from(tuple);

    assert_eq!(point.timestamp, 1608142381229);
    assert!((point.mark_price - 0.5165791606037885).abs() < f64::EPSILON);
}

#[test]
fn test_mark_price_history_point_into_tuple() {
    let point = MarkPriceHistoryPoint {
        timestamp: 1608142381229,
        mark_price: 0.5165791606037885,
    };
    let tuple: (u64, f64) = point.into();

    assert_eq!(tuple.0, 1608142381229);
    assert!((tuple.1 - 0.5165791606037885).abs() < f64::EPSILON);
}

#[test]
fn test_mark_price_history_point_deserialization_from_array() {
    let json = "[1608142381229, 0.5165791606037885]";
    let point: MarkPriceHistoryPoint = serde_json::from_str(json).unwrap();

    assert_eq!(point.timestamp, 1608142381229);
    assert!((point.mark_price - 0.5165791606037885).abs() < f64::EPSILON);
}

#[test]
fn test_mark_price_history_point_serialization_to_array() {
    let point = MarkPriceHistoryPoint {
        timestamp: 1608142381229,
        mark_price: 0.5165791606037885,
    };
    let serialized = serde_json::to_string(&point).unwrap();

    assert!(serialized.contains("1608142381229"));
    assert!(serialized.contains("0.5165791606037885"));
}

#[test]
fn test_mark_price_history_point_vec_deserialization() {
    let json = r#"[
        [1608142381229, 0.5165791606037885],
        [1608142380231, 0.5165737855432504],
        [1608142379227, 0.5165768236356326]
    ]"#;
    let points: Vec<MarkPriceHistoryPoint> = serde_json::from_str(json).unwrap();

    assert_eq!(points.len(), 3);
    assert_eq!(points[0].timestamp, 1608142381229);
    assert_eq!(points[1].timestamp, 1608142380231);
    assert_eq!(points[2].timestamp, 1608142379227);
}

#[test]
fn test_mark_price_history_point_empty_vec_deserialization() {
    let json = "[]";
    let points: Vec<MarkPriceHistoryPoint> = serde_json::from_str(json).unwrap();

    assert!(points.is_empty());
}

#[test]
fn test_mark_price_history_point_clone() {
    let point = MarkPriceHistoryPoint {
        timestamp: 1608142381229,
        mark_price: 0.5165791606037885,
    };
    let cloned = point.clone();

    assert_eq!(point.timestamp, cloned.timestamp);
    assert!((point.mark_price - cloned.mark_price).abs() < f64::EPSILON);
}

#[test]
fn test_mark_price_history_point_equality() {
    let point1 = MarkPriceHistoryPoint {
        timestamp: 1608142381229,
        mark_price: 0.5165791606037885,
    };
    let point2 = MarkPriceHistoryPoint {
        timestamp: 1608142381229,
        mark_price: 0.5165791606037885,
    };

    assert_eq!(point1, point2);
}

// Tests for IndexNameInfo
#[test]
fn test_index_name_info_creation() {
    let info = IndexNameInfo {
        name: "btc_usdc".to_string(),
        future_combo_enabled: Some(true),
        option_combo_enabled: Some(false),
    };

    assert_eq!(info.name, "btc_usdc");
    assert_eq!(info.future_combo_enabled, Some(true));
    assert_eq!(info.option_combo_enabled, Some(false));
}

#[test]
fn test_index_name_info_minimal() {
    let info = IndexNameInfo {
        name: "btc_eth".to_string(),
        future_combo_enabled: None,
        option_combo_enabled: None,
    };

    assert_eq!(info.name, "btc_eth");
    assert!(info.future_combo_enabled.is_none());
    assert!(info.option_combo_enabled.is_none());
}

#[test]
fn test_index_name_info_serialization() {
    let info = IndexNameInfo {
        name: "eth_usdc".to_string(),
        future_combo_enabled: Some(true),
        option_combo_enabled: Some(true),
    };

    let serialized = serde_json::to_string(&info).unwrap();
    assert!(serialized.contains("eth_usdc"));
    assert!(serialized.contains("future_combo_enabled"));
    assert!(serialized.contains("option_combo_enabled"));
}

#[test]
fn test_index_name_info_serialization_skips_none() {
    let info = IndexNameInfo {
        name: "sol_usdc".to_string(),
        future_combo_enabled: None,
        option_combo_enabled: None,
    };

    let serialized = serde_json::to_string(&info).unwrap();
    assert!(serialized.contains("sol_usdc"));
    assert!(!serialized.contains("future_combo_enabled"));
    assert!(!serialized.contains("option_combo_enabled"));
}

#[test]
fn test_index_name_info_deserialization_full() {
    let json = r#"{
        "name": "btc_usdc",
        "future_combo_enabled": true,
        "option_combo_enabled": false
    }"#;

    let info: IndexNameInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.name, "btc_usdc");
    assert_eq!(info.future_combo_enabled, Some(true));
    assert_eq!(info.option_combo_enabled, Some(false));
}

#[test]
fn test_index_name_info_deserialization_minimal() {
    let json = r#"{"name": "btc_eth"}"#;

    let info: IndexNameInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.name, "btc_eth");
    assert!(info.future_combo_enabled.is_none());
    assert!(info.option_combo_enabled.is_none());
}

#[test]
fn test_index_name_info_vec_deserialization() {
    let json = r#"[
        {"name": "btc_eth", "future_combo_enabled": true, "option_combo_enabled": false},
        {"name": "btc_usdc", "future_combo_enabled": false, "option_combo_enabled": true},
        {"name": "eth_usdc"}
    ]"#;

    let infos: Vec<IndexNameInfo> = serde_json::from_str(json).unwrap();
    assert_eq!(infos.len(), 3);
    assert_eq!(infos[0].name, "btc_eth");
    assert_eq!(infos[1].name, "btc_usdc");
    assert_eq!(infos[2].name, "eth_usdc");
    assert!(infos[2].future_combo_enabled.is_none());
}

#[test]
fn test_index_name_info_clone() {
    let info = IndexNameInfo {
        name: "btc_usdc".to_string(),
        future_combo_enabled: Some(true),
        option_combo_enabled: Some(false),
    };
    let cloned = info.clone();

    assert_eq!(info.name, cloned.name);
    assert_eq!(info.future_combo_enabled, cloned.future_combo_enabled);
    assert_eq!(info.option_combo_enabled, cloned.option_combo_enabled);
}

#[test]
fn test_index_name_info_equality() {
    let info1 = IndexNameInfo {
        name: "btc_usdc".to_string(),
        future_combo_enabled: Some(true),
        option_combo_enabled: Some(false),
    };
    let info2 = IndexNameInfo {
        name: "btc_usdc".to_string(),
        future_combo_enabled: Some(true),
        option_combo_enabled: Some(false),
    };

    assert_eq!(info1, info2);
}

#[test]
fn test_index_name_info_empty_vec() {
    let json = "[]";
    let infos: Vec<IndexNameInfo> = serde_json::from_str(json).unwrap();

    assert!(infos.is_empty());
}

// Tests for TradeVolume
#[test]
fn test_trade_volume_basic_creation() {
    let vol = TradeVolume {
        currency: "BTC".to_string(),
        calls_volume: 145.0,
        puts_volume: 48.0,
        futures_volume: 6.25578452,
        spot_volume: 11.1,
        calls_volume_7d: None,
        puts_volume_7d: None,
        futures_volume_7d: None,
        spot_volume_7d: None,
        calls_volume_30d: None,
        puts_volume_30d: None,
        futures_volume_30d: None,
        spot_volume_30d: None,
    };

    assert_eq!(vol.currency, "BTC");
    assert!((vol.calls_volume - 145.0).abs() < f64::EPSILON);
    assert!((vol.puts_volume - 48.0).abs() < f64::EPSILON);
    assert!((vol.futures_volume - 6.25578452).abs() < 1e-8);
    assert!((vol.spot_volume - 11.1).abs() < f64::EPSILON);
}

#[test]
fn test_trade_volume_extended_creation() {
    let vol = TradeVolume {
        currency: "ETH".to_string(),
        calls_volume: 37.4,
        puts_volume: 122.65,
        futures_volume: 374.392173,
        spot_volume: 57.7,
        calls_volume_7d: Some(75.6),
        puts_volume_7d: Some(356.9),
        futures_volume_7d: Some(213.8841),
        spot_volume_7d: Some(64.8),
        calls_volume_30d: Some(547.3),
        puts_volume_30d: Some(785.5),
        futures_volume_30d: Some(998.2128),
        spot_volume_30d: Some(310.5),
    };

    assert_eq!(vol.currency, "ETH");
    assert_eq!(vol.calls_volume_7d, Some(75.6));
    assert_eq!(vol.puts_volume_30d, Some(785.5));
}

#[test]
fn test_trade_volume_deserialization_basic() {
    let json = r#"{
        "currency": "BTC",
        "calls_volume": 145,
        "puts_volume": 48,
        "futures_volume": 6.25578452,
        "spot_volume": 11.1
    }"#;

    let vol: TradeVolume = serde_json::from_str(json).unwrap();
    assert_eq!(vol.currency, "BTC");
    assert!((vol.calls_volume - 145.0).abs() < f64::EPSILON);
    assert!(vol.calls_volume_7d.is_none());
    assert!(vol.futures_volume_30d.is_none());
}

#[test]
fn test_trade_volume_deserialization_extended() {
    let json = r#"{
        "currency": "ETH",
        "calls_volume": 37.4,
        "puts_volume": 122.65,
        "futures_volume": 374.392173,
        "spot_volume": 57.7,
        "calls_volume_7d": 75.6,
        "puts_volume_7d": 356.9,
        "futures_volume_7d": 213.8841,
        "spot_volume_7d": 64.8,
        "calls_volume_30d": 547.3,
        "puts_volume_30d": 785.5,
        "futures_volume_30d": 998.2128,
        "spot_volume_30d": 310.5
    }"#;

    let vol: TradeVolume = serde_json::from_str(json).unwrap();
    assert_eq!(vol.currency, "ETH");
    assert_eq!(vol.calls_volume_7d, Some(75.6));
    assert_eq!(vol.futures_volume_30d, Some(998.2128));
}

#[test]
fn test_trade_volume_serialization_skips_none() {
    let vol = TradeVolume {
        currency: "USDC".to_string(),
        calls_volume: 10.0,
        puts_volume: 20.0,
        futures_volume: 30.0,
        spot_volume: 40.0,
        calls_volume_7d: None,
        puts_volume_7d: None,
        futures_volume_7d: None,
        spot_volume_7d: None,
        calls_volume_30d: None,
        puts_volume_30d: None,
        futures_volume_30d: None,
        spot_volume_30d: None,
    };

    let serialized = serde_json::to_string(&vol).unwrap();
    assert!(serialized.contains("USDC"));
    assert!(!serialized.contains("calls_volume_7d"));
    assert!(!serialized.contains("futures_volume_30d"));
}

#[test]
fn test_trade_volume_vec_deserialization() {
    let json = r#"[
        {
            "currency": "BTC",
            "calls_volume": 145,
            "puts_volume": 48,
            "futures_volume": 6.25578452,
            "spot_volume": 11.1
        },
        {
            "currency": "ETH",
            "calls_volume": 37.4,
            "puts_volume": 122.65,
            "futures_volume": 374.392173,
            "spot_volume": 57.7
        }
    ]"#;

    let volumes: Vec<TradeVolume> = serde_json::from_str(json).unwrap();
    assert_eq!(volumes.len(), 2);
    assert_eq!(volumes[0].currency, "BTC");
    assert_eq!(volumes[1].currency, "ETH");
}

#[test]
fn test_trade_volume_empty_vec() {
    let json = "[]";
    let volumes: Vec<TradeVolume> = serde_json::from_str(json).unwrap();

    assert!(volumes.is_empty());
}

#[test]
fn test_trade_volume_clone() {
    let vol = TradeVolume {
        currency: "BTC".to_string(),
        calls_volume: 145.0,
        puts_volume: 48.0,
        futures_volume: 6.25578452,
        spot_volume: 11.1,
        calls_volume_7d: Some(75.6),
        puts_volume_7d: None,
        futures_volume_7d: None,
        spot_volume_7d: None,
        calls_volume_30d: None,
        puts_volume_30d: None,
        futures_volume_30d: None,
        spot_volume_30d: None,
    };
    let cloned = vol.clone();

    assert_eq!(vol.currency, cloned.currency);
    assert_eq!(vol.calls_volume_7d, cloned.calls_volume_7d);
}

#[test]
fn test_trade_volume_equality() {
    let vol1 = TradeVolume {
        currency: "BTC".to_string(),
        calls_volume: 145.0,
        puts_volume: 48.0,
        futures_volume: 6.25578452,
        spot_volume: 11.1,
        calls_volume_7d: None,
        puts_volume_7d: None,
        futures_volume_7d: None,
        spot_volume_7d: None,
        calls_volume_30d: None,
        puts_volume_30d: None,
        futures_volume_30d: None,
        spot_volume_30d: None,
    };
    let vol2 = TradeVolume {
        currency: "BTC".to_string(),
        calls_volume: 145.0,
        puts_volume: 48.0,
        futures_volume: 6.25578452,
        spot_volume: 11.1,
        calls_volume_7d: None,
        puts_volume_7d: None,
        futures_volume_7d: None,
        spot_volume_7d: None,
        calls_volume_30d: None,
        puts_volume_30d: None,
        futures_volume_30d: None,
        spot_volume_30d: None,
    };

    assert_eq!(vol1, vol2);
}
