use deribit_http::model::instrument::InstrumentKind;
use deribit_http::model::order::OrderSide;
use deribit_http::model::trade::{
    ClientInfo, LastTrade, Liquidity, Trade, TradeAllocation, TradeExecution, TradeStats, UserTrade,
};
use serde_json;

// Helper functions to create mock data
fn create_mock_trade_execution() -> TradeExecution {
    TradeExecution {
        amount: 1.5,
        direction: "buy".to_string(),
        fee: 0.0005,
        fee_currency: "BTC".to_string(),
        index_price: 49900.0,
        instrument_name: "BTC-PERPETUAL".to_string(),
        iv: Some(0.8),
        label: "test_trade".to_string(),
        liquidity: "M".to_string(),
        mark_price: 50000.0,
        matching_id: Some("match_123".to_string()),
        order_id: "order_456".to_string(),
        order_type: "limit".to_string(),
        original_order_type: Some("limit".to_string()),
        price: 50000.0,
        self_trade: false,
        state: "filled".to_string(),
        tick_direction: 1,
        timestamp: 1640995200000,
        trade_id: "trade_789".to_string(),
        trade_seq: 123456,
        underlying_price: Some(49950.0),
    }
}

fn create_mock_user_trade() -> UserTrade {
    UserTrade {
        amount: 1.5,
        api: Some(true),
        contracts: Some(1.5),
        direction: "buy".to_string(),
        fee: 0.0005,
        fee_currency: "BTC".to_string(),
        index_price: 49900.0,
        instrument_name: "BTC-PERPETUAL".to_string(),
        iv: Some(0.8),
        label: Some("test_trade".to_string()),
        liquidity: "M".to_string(),
        mark_price: 50000.0,
        matching_id: Some("match_123".to_string()),
        mmp: Some(false),
        order_id: "order_456".to_string(),
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
        timestamp: 1640995200000,
        trade_id: "trade_789".to_string(),
        trade_seq: 123456,
        underlying_price: Some(49950.0),
        user_id: Some(12345),
    }
}

fn create_mock_last_trade() -> LastTrade {
    LastTrade {
        amount: 1.5,
        direction: "buy".to_string(),
        index_price: 49900.0,
        instrument_name: "BTC-PERPETUAL".to_string(),
        iv: Some(0.8),
        liquid: Some("M".to_string()),
        price: 50000.0,
        tick_direction: 1,
        timestamp: 1640995200000,
        trade_id: "trade_789".to_string(),
        trade_seq: 123456,
    }
}

fn create_mock_trade() -> Trade {
    Trade {
        trade_id: "trade_789".to_string(),
        instrument_name: "BTC-PERPETUAL".to_string(),
        order_id: "order_456".to_string(),
        direction: OrderSide::Buy,
        amount: 1.5,
        price: 50000.0,
        timestamp: 1640995200000,
        fee: 0.0005,
        fee_currency: "BTC".to_string(),
        liquidity: Liquidity::Maker,
        mark_price: 50000.0,
        index_price: 49900.0,
        instrument_kind: Some(InstrumentKind::Future),
        trade_seq: Some(123456),
        user_role: Some("maker".to_string()),
        block_trade: Some(false),
        underlying_price: Some(49950.0),
        iv: Some(0.8),
        label: Some("test_trade".to_string()),
        profit_loss: Some(100.0),
        tick_direction: Some(1),
        self_trade: Some(false),
    }
}

fn create_mock_client_info() -> ClientInfo {
    ClientInfo {
        client_id: 1001,
        client_link_id: 2001,
        name: "Test Client".to_string(),
    }
}

fn create_mock_trade_allocation() -> TradeAllocation {
    TradeAllocation {
        amount: 1.0,
        client_info: Some(create_mock_client_info()),
        fee: 0.0003,
        user_id: 12345,
    }
}

// Tests for TradeExecution
#[test]
fn test_trade_execution_creation() {
    let trade = create_mock_trade_execution();
    assert_eq!(trade.amount, 1.5);
    assert_eq!(trade.direction, "buy");
    assert_eq!(trade.price, 50000.0);
    assert_eq!(trade.trade_id, "trade_789");
    assert!(!trade.self_trade);
}

#[test]
fn test_trade_execution_serialization() {
    let trade = create_mock_trade_execution();
    let serialized = serde_json::to_string(&trade).unwrap();
    assert!(serialized.contains("trade_789"));
    assert!(serialized.contains("BTC-PERPETUAL"));
    assert!(serialized.contains("50000"));
}

#[test]
fn test_trade_execution_deserialization() {
    let json = r#"{
        "amount": 1.5,
        "direction": "buy",
        "fee": 0.0005,
        "fee_currency": "BTC",
        "index_price": 49900.0,
        "instrument_name": "BTC-PERPETUAL",
        "label": "test_trade",
        "liquidity": "M",
        "mark_price": 50000.0,
        "order_id": "order_456",
        "order_type": "limit",
        "price": 50000.0,
        "self_trade": false,
        "state": "filled",
        "tick_direction": 1,
        "timestamp": 1640995200000,
        "trade_id": "trade_789",
        "trade_seq": 123456
    }"#;

    let deserialized: TradeExecution = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.trade_id, "trade_789");
    assert_eq!(deserialized.amount, 1.5);
    assert_eq!(deserialized.price, 50000.0);
}

#[test]
fn test_trade_execution_clone() {
    let trade = create_mock_trade_execution();
    let cloned = trade.clone();
    assert_eq!(trade.trade_id, cloned.trade_id);
    assert_eq!(trade.amount, cloned.amount);
}

// Tests for UserTrade
#[test]
fn test_user_trade_creation() {
    let trade = create_mock_user_trade();
    assert_eq!(trade.amount, 1.5);
    assert_eq!(trade.direction, "buy");
    assert_eq!(trade.user_id, Some(12345));
    assert_eq!(trade.mmp, Some(false));
}

#[test]
fn test_user_trade_serialization() {
    let trade = create_mock_user_trade();
    let serialized = serde_json::to_string(&trade).unwrap();
    assert!(serialized.contains("user_id"));
    assert!(serialized.contains("12345"));
    assert!(serialized.contains("mmp"));
}

#[test]
fn test_user_trade_deserialization() {
    let json = r#"{
        "amount": 1.5,
        "direction": "buy",
        "fee": 0.0005,
        "fee_currency": "BTC",
        "index_price": 49900.0,
        "instrument_name": "BTC-PERPETUAL",
        "liquidity": "M",
        "mark_price": 50000.0,
        "order_id": "order_456",
        "order_type": "limit",
        "price": 50000.0,
        "self_trade": false,
        "state": "filled",
        "tick_direction": 1,
        "timestamp": 1640995200000,
        "trade_id": "trade_789",
        "trade_seq": 123456,
        "user_id": 12345
    }"#;

    let deserialized: UserTrade = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.user_id, Some(12345));
    assert_eq!(deserialized.trade_id, "trade_789");
}

#[test]
fn test_user_trade_clone() {
    let trade = create_mock_user_trade();
    let cloned = trade.clone();
    assert_eq!(trade.user_id, cloned.user_id);
    assert_eq!(trade.trade_id, cloned.trade_id);
}

// Tests for LastTrade
#[test]
fn test_last_trade_creation() {
    let trade = create_mock_last_trade();
    assert_eq!(trade.amount, 1.5);
    assert_eq!(trade.direction, "buy");
    assert_eq!(trade.trade_id, "trade_789");
    assert_eq!(trade.liquid, Some("M".to_string()));
}

#[test]
fn test_last_trade_serialization() {
    let trade = create_mock_last_trade();
    let serialized = serde_json::to_string(&trade).unwrap();
    assert!(serialized.contains("trade_789"));
    assert!(serialized.contains("liquid"));
}

#[test]
fn test_last_trade_deserialization() {
    let json = r#"{
        "amount": 1.5,
        "direction": "buy",
        "index_price": 49900.0,
        "instrument_name": "BTC-PERPETUAL",
        "price": 50000.0,
        "tick_direction": 1,
        "timestamp": 1640995200000,
        "trade_id": "trade_789",
        "trade_seq": 123456
    }"#;

    let deserialized: LastTrade = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.trade_id, "trade_789");
    assert_eq!(deserialized.amount, 1.5);
}

#[test]
fn test_last_trade_clone() {
    let trade = create_mock_last_trade();
    let cloned = trade.clone();
    assert_eq!(trade.trade_id, cloned.trade_id);
    assert_eq!(trade.amount, cloned.amount);
}

// Tests for Liquidity enum
#[test]
fn test_liquidity_serialization() {
    assert_eq!(serde_json::to_string(&Liquidity::Maker).unwrap(), r#""M""#);
    assert_eq!(serde_json::to_string(&Liquidity::Taker).unwrap(), r#""T""#);
    assert_eq!(serde_json::to_string(&Liquidity::Mixed).unwrap(), r#""MT""#);
}

#[test]
fn test_liquidity_deserialization() {
    assert_eq!(
        serde_json::from_str::<Liquidity>(r#""M""#).unwrap(),
        Liquidity::Maker
    );
    assert_eq!(
        serde_json::from_str::<Liquidity>(r#""T""#).unwrap(),
        Liquidity::Taker
    );
    assert_eq!(
        serde_json::from_str::<Liquidity>(r#""MT""#).unwrap(),
        Liquidity::Mixed
    );
}

#[test]
fn test_liquidity_clone() {
    let liquidity = Liquidity::Maker;
    let cloned = liquidity.clone();
    assert_eq!(liquidity, cloned);
}

// Tests for Trade
#[test]
fn test_trade_creation() {
    let trade = create_mock_trade();
    assert_eq!(trade.trade_id, "trade_789");
    assert_eq!(trade.direction, OrderSide::Buy);
    assert_eq!(trade.liquidity, Liquidity::Maker);
    assert_eq!(trade.amount, 1.5);
}

#[test]
fn test_trade_serialization() {
    let trade = create_mock_trade();
    let serialized = serde_json::to_string(&trade).unwrap();
    assert!(serialized.contains("trade_789"));
    assert!(serialized.contains("Buy"));
    assert!(serialized.contains("M"));
}

#[test]
fn test_trade_deserialization() {
    let json = r#"{
        "trade_id": "trade_789",
        "instrument_name": "BTC-PERPETUAL",
        "order_id": "order_456",
        "direction": "Buy",
        "amount": 1.5,
        "price": 50000.0,
        "timestamp": 1640995200000,
        "fee": 0.0005,
        "fee_currency": "BTC",
        "liquidity": "M",
        "mark_price": 50000.0,
        "index_price": 49900.0
    }"#;

    let deserialized: Trade = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.trade_id, "trade_789");
    assert_eq!(deserialized.direction, OrderSide::Buy);
    assert_eq!(deserialized.liquidity, Liquidity::Maker);
}

#[test]
fn test_trade_clone() {
    let trade = create_mock_trade();
    let cloned = trade.clone();
    assert_eq!(trade.trade_id, cloned.trade_id);
    assert_eq!(trade.direction, cloned.direction);
}

// Tests for Trade methods
#[test]
fn test_trade_notional_value() {
    let trade = create_mock_trade();
    let notional = trade.notional_value();
    assert_eq!(notional, 75000.0); // 1.5 * 50000.0
}

#[test]
fn test_trade_is_maker() {
    let mut trade = create_mock_trade();
    trade.liquidity = Liquidity::Maker;
    assert!(trade.is_maker());

    trade.liquidity = Liquidity::Mixed;
    assert!(trade.is_maker());

    trade.liquidity = Liquidity::Taker;
    assert!(!trade.is_maker());
}

#[test]
fn test_trade_is_taker() {
    let mut trade = create_mock_trade();
    trade.liquidity = Liquidity::Taker;
    assert!(trade.is_taker());

    trade.liquidity = Liquidity::Mixed;
    assert!(trade.is_taker());

    trade.liquidity = Liquidity::Maker;
    assert!(!trade.is_taker());
}

#[test]
fn test_trade_is_buy() {
    let mut trade = create_mock_trade();
    trade.direction = OrderSide::Buy;
    assert!(trade.is_buy());
    assert!(!trade.is_sell());
}

#[test]
fn test_trade_is_sell() {
    let mut trade = create_mock_trade();
    trade.direction = OrderSide::Sell;
    assert!(trade.is_sell());
    assert!(!trade.is_buy());
}

#[test]
fn test_trade_fee_percentage() {
    let trade = create_mock_trade();
    let fee_pct = trade.fee_percentage();
    let expected = (0.0005 / 75000.0) * 100.0; // (fee / notional) * 100
    assert!((fee_pct - expected).abs() < 1e-10);
}

#[test]
fn test_trade_fee_percentage_zero_notional() {
    let mut trade = create_mock_trade();
    trade.amount = 0.0;
    let fee_pct = trade.fee_percentage();
    assert_eq!(fee_pct, 0.0);
}

// Tests for TradeStats
#[test]
fn test_trade_stats_new() {
    let stats = TradeStats::new();
    assert_eq!(stats.count, 0);
    assert_eq!(stats.volume, 0.0);
    assert_eq!(stats.total_fees, 0.0);
    assert_eq!(stats.avg_price, 0.0);
    assert_eq!(stats.pnl, 0.0);
    assert_eq!(stats.winning_trades, 0);
    assert_eq!(stats.losing_trades, 0);
}

#[test]
fn test_trade_stats_default() {
    let stats = TradeStats::default();
    assert_eq!(stats.count, 0);
    assert_eq!(stats.volume, 0.0);
}

#[test]
fn test_trade_stats_win_rate() {
    let mut stats = TradeStats::new();
    stats.count = 10;
    stats.winning_trades = 7;
    stats.losing_trades = 3;

    let win_rate = stats.win_rate();
    assert_eq!(win_rate, 70.0);
}

#[test]
fn test_trade_stats_win_rate_zero_trades() {
    let stats = TradeStats::new();
    let win_rate = stats.win_rate();
    assert_eq!(win_rate, 0.0);
}

#[test]
fn test_trade_stats_serialization() {
    let stats = TradeStats {
        count: 5,
        volume: 1000.0,
        total_fees: 0.5,
        avg_price: 50000.0,
        pnl: 250.0,
        winning_trades: 3,
        losing_trades: 2,
    };

    let serialized = serde_json::to_string(&stats).unwrap();
    assert!(serialized.contains("count"));
    assert!(serialized.contains("5"));
    assert!(serialized.contains("pnl"));
}

#[test]
fn test_trade_stats_clone() {
    let stats = TradeStats::new();
    let cloned = stats.clone();
    assert_eq!(stats.count, cloned.count);
    assert_eq!(stats.volume, cloned.volume);
}

// Tests for ClientInfo
#[test]
fn test_client_info_creation() {
    let client_info = create_mock_client_info();
    assert_eq!(client_info.client_id, 1001);
    assert_eq!(client_info.client_link_id, 2001);
    assert_eq!(client_info.name, "Test Client");
}

#[test]
fn test_client_info_serialization() {
    let client_info = create_mock_client_info();
    let serialized = serde_json::to_string(&client_info).unwrap();
    assert!(serialized.contains("client_id"));
    assert!(serialized.contains("1001"));
    assert!(serialized.contains("Test Client"));
}

#[test]
fn test_client_info_clone() {
    let client_info = create_mock_client_info();
    let cloned = client_info.clone();
    assert_eq!(client_info.client_id, cloned.client_id);
    assert_eq!(client_info.name, cloned.name);
}

// Tests for TradeAllocation
#[test]
fn test_trade_allocation_creation() {
    let allocation = create_mock_trade_allocation();
    assert_eq!(allocation.amount, 1.0);
    assert_eq!(allocation.fee, 0.0003);
    assert_eq!(allocation.user_id, 12345);
    assert!(allocation.client_info.is_some());
}

#[test]
fn test_trade_allocation_serialization() {
    let allocation = create_mock_trade_allocation();
    let serialized = serde_json::to_string(&allocation).unwrap();
    assert!(serialized.contains("amount"));
    assert!(serialized.contains("user_id"));
    assert!(serialized.contains("client_info"));
}

#[test]
fn test_trade_allocation_clone() {
    let allocation = create_mock_trade_allocation();
    let cloned = allocation.clone();
    assert_eq!(allocation.amount, cloned.amount);
    assert_eq!(allocation.user_id, cloned.user_id);
}

// Edge cases and error handling
#[test]
fn test_trade_with_minimal_data() {
    let trade = Trade {
        trade_id: "minimal".to_string(),
        instrument_name: "TEST".to_string(),
        order_id: "order_1".to_string(),
        direction: OrderSide::Buy,
        amount: 0.0,
        price: 0.0,
        timestamp: 0,
        fee: 0.0,
        fee_currency: "USD".to_string(),
        liquidity: Liquidity::Taker,
        mark_price: 0.0,
        index_price: 0.0,
        instrument_kind: None,
        trade_seq: None,
        user_role: None,
        block_trade: None,
        underlying_price: None,
        iv: None,
        label: None,
        profit_loss: None,
        tick_direction: None,
        self_trade: None,
    };

    assert_eq!(trade.notional_value(), 0.0);
    assert_eq!(trade.fee_percentage(), 0.0);
    assert!(trade.is_taker());
    assert!(!trade.is_maker());
}

#[test]
fn test_trade_execution_with_self_trade() {
    let mut trade = create_mock_trade_execution();
    trade.self_trade = true;
    assert!(trade.self_trade);
}

#[test]
fn test_user_trade_with_none_values() {
    let mut trade = create_mock_user_trade();
    trade.api = None;
    trade.contracts = None;
    trade.label = None;
    trade.mmp = None;
    trade.user_id = None;

    let serialized = serde_json::to_string(&trade).unwrap();
    assert!(!serialized.contains("api"));
    assert!(!serialized.contains("contracts"));
    assert!(!serialized.contains("label"));
    assert!(!serialized.contains("mmp"));
    assert!(!serialized.contains("user_id"));
}

#[test]
fn test_last_trade_without_iv() {
    let mut trade = create_mock_last_trade();
    trade.iv = None;
    trade.liquid = None;

    let serialized = serde_json::to_string(&trade).unwrap();
    assert!(!serialized.contains("iv"));
    assert!(!serialized.contains("liquid"));
}
