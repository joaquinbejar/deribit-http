//! Type definitions for Deribit HTTP API endpoints
//!
//! This module contains all the struct definitions used by the public and private endpoints.

use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

// Re-export types from deribit-base for convenience
pub use deribit_base::prelude::{OrderBook, OrderBookEntry, Trade};
use deribit_base::prelude::{OrderType, TimeInForce};
// =============================================================================
// PRIVATE ENDPOINT TYPES
// =============================================================================

/// Funding chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingChartData {
    /// Current interest rate
    pub current_interest: f64,
    /// Interest rate for 8 hours
    pub interest_8h: f64,
    /// List of funding data points
    pub data: Vec<FundingDataPoint>,
}

/// Funding data point structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingDataPoint {
    /// Index price
    pub index_price: f64,
    /// Interest rate for 8 hours
    pub interest_8h: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// TradingView chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TradingViewChartData {
    /// Status of the request
    pub status: String,
    /// Timestamps
    pub ticks: Vec<u64>,
    /// Opening prices
    pub open: Vec<f64>,
    /// High prices
    pub high: Vec<f64>,
    /// Low prices
    pub low: Vec<f64>,
    /// Closing prices
    pub close: Vec<f64>,
    /// Volume data
    pub volume: Vec<f64>,
    /// Cost data
    pub cost: Vec<f64>,
}

/// Transfer result structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// Transfer ID
    pub id: u64,
    /// Transfer type (subaccount, user)
    #[serde(rename = "type")]
    pub transfer_type: String,
    /// Transfer state (confirmed, prepared, etc.)
    pub state: String,
    /// Currency
    pub currency: String,
    /// Transfer amount
    pub amount: f64,
    /// Transfer direction (payment, etc.)
    pub direction: String,
    /// Other side (destination info)
    pub other_side: String,
    /// Creation timestamp
    pub created_timestamp: u64,
    /// Last update timestamp
    pub updated_timestamp: u64,
}

/// Deposits response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct DepositsResponse {
    /// Total count of deposits
    pub count: u32,
    /// List of deposit entries
    pub data: Vec<Deposit>,
}

/// Deposit structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// Deposit address
    pub address: String,
    /// Deposit amount
    pub amount: f64,
    /// Currency
    pub currency: String,
    /// Timestamp when deposit was received
    pub received_timestamp: u64,
    /// Deposit state (completed, pending, etc.)
    pub state: String,
    /// Transaction ID
    pub transaction_id: String,
    /// Timestamp when deposit was last updated
    pub updated_timestamp: u64,
}

/// Withdrawals response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalsResponse {
    /// Total count of withdrawals
    pub count: u32,
    /// List of withdrawal entries
    pub data: Vec<Withdrawal>,
}

/// Withdrawal structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    /// Withdrawal address
    pub address: String,
    /// Withdrawal amount
    pub amount: f64,
    /// Timestamp when withdrawal was confirmed (optional)
    pub confirmed_timestamp: Option<u64>,
    /// Timestamp when withdrawal was created
    pub created_timestamp: u64,
    /// Currency
    pub currency: String,
    /// Withdrawal fee
    pub fee: f64,
    /// Withdrawal ID
    pub id: u64,
    /// Priority level
    pub priority: f64,
    /// Withdrawal state (confirmed, unconfirmed, etc.)
    pub state: String,
    /// Transaction ID (optional)
    pub transaction_id: Option<String>,
    /// Timestamp when withdrawal was last updated
    pub updated_timestamp: u64,
}

/// Subaccount structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Subaccount {
    /// Subaccount email
    pub email: String,
    /// Subaccount ID
    pub id: u64,
    /// Login enabled
    pub login_enabled: bool,
    /// Portfolio information (optional)
    pub portfolio: Option<PortfolioInfo>,
    /// Receive notifications
    pub receive_notifications: bool,
    /// System name
    pub system_name: String,
    /// Time in force (optional)
    pub tif: Option<String>,
    /// Type of subaccount
    #[serde(rename = "type")]
    pub subaccount_type: String,
    /// Username
    pub username: String,
}

/// Portfolio info structure
#[derive(Clone, Serialize, Deserialize)]
pub struct PortfolioInfo {
    /// Available balance
    pub available_funds: f64,
    /// Available withdrawal balance
    pub available_withdrawal_funds: f64,
    /// Balance
    pub balance: f64,
    /// Currency
    pub currency: String,
    /// Delta total
    pub delta_total: f64,
    /// Equity
    pub equity: f64,
    /// Initial margin
    pub initial_margin: f64,
    /// Maintenance margin
    pub maintenance_margin: f64,
    /// Margin balance
    pub margin_balance: f64,
    /// Session RPL
    pub session_rpl: f64,
    /// Session UPL
    pub session_upl: f64,
    /// Total PL
    pub total_pl: f64,
}

/// Transaction log structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionLog {
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// List of transaction log entries
    pub logs: Vec<TransactionLogEntry>,
}

/// Transaction log entry structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionLogEntry {
    /// Amount involved in transaction
    pub amount: f64,
    /// Balance after transaction
    pub balance: f64,
    /// Currency
    pub currency: String,
    /// ID of the transaction
    pub id: u64,
    /// Instrument name (optional)
    pub instrument_name: Option<String>,
    /// Order ID (optional)
    pub order_id: Option<String>,
    /// Position (optional)
    pub position: Option<f64>,
    /// Side/direction (optional)
    pub side: Option<String>,
    /// Timestamp of transaction
    pub timestamp: u64,
    /// Total interest paid (optional)
    pub total_interest_pl: Option<f64>,
    /// Trade ID (optional)
    pub trade_id: Option<String>,
    /// Transaction type
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Username (optional)
    pub username: Option<String>,
}

/// Mass quote request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct MassQuoteRequest {
    pub instrument_name: String,
    pub bid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub bid_amount: Option<f64>,
    pub ask_amount: Option<f64>,
}

/// Mass quote response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct MassQuoteResponse {
    pub quotes: Vec<QuoteResult>,
}

/// Quote result structure
#[derive(Clone, Serialize, Deserialize)]
pub struct QuoteResult {
    pub instrument_name: String,
    pub success: bool,
    pub error: Option<String>,
}

/// User trade structure
#[derive(Clone, Serialize, Deserialize)]
pub struct UserTrade {
    pub amount: f64,
    pub direction: String,
    pub fee: f64,
    pub fee_currency: String,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: String,
    pub liquidity: String,
    pub mark_price: f64,
    pub matching_id: Option<String>,
    pub order_id: String,
    pub order_type: String,
    pub original_order_type: Option<String>,
    pub price: f64,
    pub self_trade: bool,
    pub state: String,
    pub tick_direction: i32,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
    pub underlying_price: Option<f64>,
}

/// Edit order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct EditOrderRequest {
    pub order_id: String,
    pub amount: Option<f64>,
    pub price: Option<f64>,
    pub advanced: Option<String>,
    pub post_only: bool,
    pub reduce_only: bool,
}


/// Buy order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct BuyOrderRequest {
    pub instrument_name: String,
    pub amount: Option<f64>,
    pub contracts: Option<u32>,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub label: Option<String>,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
    pub reduce_only: bool,
}

impl Default for BuyOrderRequest {
    fn default() -> Self {
        Self {
            instrument_name: String::new(),
            amount: None,
            contracts: None,
            order_type: OrderType::Limit,
            price: None,
            label: None,
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: false,
            reduce_only: false,
        }
    }
}

/// Sell order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct SellOrderRequest {
    pub instrument_name: String,
    pub amount: Option<f64>,
    pub contracts: Option<u32>,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub label: Option<String>,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
    pub reduce_only: bool,
}

impl Default for SellOrderRequest {
    fn default() -> Self {
        Self {
            instrument_name: String::new(),
            amount: None,
            contracts: None,
            order_type: OrderType::Limit,
            price: None,
            label: None,
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: false,
            reduce_only: false,
        }
    }
}

/// Order response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: OrderInfo,
    pub trades: Vec<TradeExecution>,
}

/// Trade execution structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    pub amount: f64,
    pub direction: String,
    pub fee: f64,
    pub fee_currency: String,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub label: String,
    pub liquidity: String,
    pub mark_price: f64,
    pub matching_id: Option<String>,
    pub order_id: String,
    pub order_type: String,
    pub original_order_type: Option<String>,
    pub price: f64,
    pub self_trade: bool,
    pub state: String,
    pub tick_direction: i32,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
    pub underlying_price: Option<f64>,
}

/// Order info structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    pub amount: f64,
    pub api: bool,
    pub average_price: f64,
    pub creation_timestamp: u64,
    pub direction: String,
    pub filled_amount: f64,
    pub instrument_name: String,
    pub is_liquidation: bool,
    pub label: String,
    pub last_update_timestamp: u64,
    pub max_show: f64,
    pub order_id: String,
    pub order_state: String,
    pub order_type: String,
    pub original_order_type: Option<String>,
    pub post_only: bool,
    pub price: f64,
    pub profit_loss: Option<f64>,
    pub reduce_only: bool,
    pub replaced: bool,
    pub risk_reducing: bool,
    pub time_in_force: String,
    pub triggered: Option<bool>,
    pub trigger: Option<String>,
    pub usd: Option<f64>,
    pub web: bool,
}

/// Account summary structure
#[derive(Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    pub available_funds: f64,
    pub available_withdrawal_funds: f64,
    pub balance: f64,
    pub currency: String,
    pub delta_total: f64,
    pub delta_total_map: std::collections::HashMap<String, f64>,
    pub deposit_address: String,
    pub equity: f64,
    pub fees: Vec<std::collections::HashMap<String, f64>>,
    pub futures_pl: f64,
    pub futures_session_rpl: f64,
    pub futures_session_upl: f64,
    pub initial_margin: f64,
    pub limits: std::collections::HashMap<String, f64>,
    pub maintenance_margin: f64,
    pub margin_balance: f64,
    pub options_delta: f64,
    pub options_gamma: f64,
    pub options_pl: f64,
    pub options_session_rpl: f64,
    pub options_session_upl: f64,
    pub options_theta: f64,
    pub options_vega: f64,
    pub portfolio_margining_enabled: bool,
    pub projected_delta_total: Option<f64>,
    pub projected_initial_margin: Option<f64>,
    pub projected_maintenance_margin: Option<f64>,
    pub session_funding: f64,
    pub session_rpl: f64,
    pub session_upl: f64,
    pub system_name: String,
    pub total_pl: f64,
    #[serde(rename = "type")]
    pub account_type: String,
}


// =============================================================================
// PUBLIC ENDPOINT TYPES
// =============================================================================

/// Currency structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Currency symbol (BTC, ETH, etc.)
    pub currency: String,
    /// Long currency name
    pub currency_long: String,
    /// Withdrawal fee
    pub fee_precision: u32,
    /// Minimum withdrawal amount
    pub min_confirmations: u32,
    /// Minimum withdrawal fee
    pub min_withdrawal_fee: f64,
    /// Withdrawal precision
    pub withdrawal_fee: f64,
    /// Withdrawal priorities
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
    /// APR for yield-generating tokens
    pub apr: Option<f64>,
}

/// Index data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct IndexData {
    /// Currency symbol
    pub currency: String,
    /// Index price
    pub index_price: f64,
    /// Index composition
    pub components: Vec<std::collections::HashMap<String, f64>>,
    /// Timestamp
    pub timestamp: u64,
    /// BTC price
    pub btc: Option<f64>,
    /// ETH price
    pub eth: Option<f64>,
    /// USDC price
    pub usdc: Option<f64>,
    /// USDT price
    pub usdt: Option<f64>,
    /// EURR price
    pub eurr: Option<f64>,
    /// EDP price
    pub edp: f64,
}

/// Withdrawal priority structure
#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalPriority {
    pub name: String,
    pub value: f64,
}

/// Index price data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct IndexPriceData {
    pub index_price: f64,
    pub estimated_delivery_price: f64,
}

/// Book summary structure
#[derive(Clone, Serialize, Deserialize)]
pub struct BookSummary {
    pub ask_price: Option<f64>,
    pub base_currency: String,
    pub bid_price: Option<f64>,
    pub creation_timestamp: u64,
    pub current_funding: Option<f64>,
    pub estimated_delivery_price: Option<f64>,
    pub funding_8h: Option<f64>,
    pub high: Option<f64>,
    pub instrument_name: String,
    pub interest_rate: Option<f64>,
    pub last: Option<f64>,
    pub low: Option<f64>,
    pub mark_price: f64,
    pub mid_price: Option<f64>,
    pub open_interest: f64,
    pub price_change: Option<f64>,
    pub quote_currency: String,
    pub underlying_index: String,
    pub underlying_price: Option<f64>,
    pub volume: f64,
    pub volume_usd: Option<f64>,
}

/// Contract size response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct ContractSizeResponse {
    pub contract_size: f64,
}

/// Test response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TestResponse {
    pub version: String,
}

/// Hello response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct HelloResponse {
    pub version: String,
}

/// Status response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub locked: bool,
    pub message: String,
    pub locked_indices: Vec<String>,
}

/// APR history response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct AprHistoryResponse {
    pub data: Vec<AprDataPoint>,
    pub continuation: Option<String>,
}

/// APR data point structure
#[derive(Clone, Serialize, Deserialize)]
pub struct AprDataPoint {
    pub apr: f64,
    pub timestamp: u64,
    pub day: i32,
}

/// Tick size step structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickSizeStep {
    pub above_price: f64,
    pub tick_size: f64,
}

/// Delivery prices response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct DeliveryPricesResponse {
    pub data: Vec<DeliveryPriceData>,
    pub records_total: u32,
}

/// Delivery price data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct DeliveryPriceData {
    pub date: String,
    pub delivery_price: f64,
}

/// Expirations response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct ExpirationsResponse {
    pub expirations: Vec<String>,
    pub future: Option<Vec<String>>,
    pub option: Option<Vec<String>>,
}

/// Funding rate data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingRateData {
    pub funding_rate: f64,
    pub index_price: f64,
    pub interest_rate: f64,
    pub prev_index_price: f64,
    pub timestamp: u64,
    pub interest_8h: f64,
    pub interest_1h: f64,
}

/// Settlements response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct SettlementsResponse {
    pub continuation: Option<String>,
    pub settlements: Vec<Settlement>,
}

/// Settlement structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub funding: Option<f64>,
    pub index_price: Option<f64>,
    pub instrument_name: Option<String>,
    pub mark_price: Option<f64>,
    pub position: Option<f64>,
    pub profit_loss: Option<f64>,
    pub session_bankrupt_cy: f64,
    pub session_tax: f64,
    pub session_tax_rate: f64,
    pub socialized_losses: f64,
    #[serde(rename = "type")]
    pub settlement_type: String,
    pub timestamp: u64,
    pub session_profit_loss: f64,
}

/// Last trades response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct LastTradesResponse {
    pub has_more: bool,
    pub trades: Vec<LastTrade>,
}

/// Last trade structure
#[derive(Clone, Serialize, Deserialize)]
pub struct LastTrade {
    pub amount: f64,
    pub direction: String,
    pub index_price: f64,
    pub instrument_name: String,
    pub iv: Option<f64>,
    pub liquid: Option<String>,
    pub price: f64,
    pub tick_direction: i32,
    pub timestamp: u64,
    pub trade_id: String,
    pub trade_seq: u64,
}

impl_json_display!(
    FundingChartData,
    FundingDataPoint,
    TradingViewChartData,
    TransferResult,
    DepositsResponse,
    Deposit,
    WithdrawalsResponse,
    Withdrawal,
    Subaccount,
    PortfolioInfo,
    TransactionLog,
    TransactionLogEntry,
    MassQuoteRequest,
    MassQuoteResponse,
    QuoteResult,
    UserTrade,
    EditOrderRequest,
    BuyOrderRequest,
    SellOrderRequest,
    OrderResponse,
    TradeExecution,
    OrderInfo,
    AccountSummary,
    Currency,
    IndexData,
    WithdrawalPriority,
    IndexPriceData,
    BookSummary,
    ContractSizeResponse,
    TestResponse,
    HelloResponse,
    StatusResponse,
    AprHistoryResponse,
    AprDataPoint,
    TickSizeStep,
    DeliveryPricesResponse,
    DeliveryPriceData,
    ExpirationsResponse,
    FundingRateData,
    SettlementsResponse,
    Settlement,
    LastTradesResponse,
    LastTrade
);

impl_json_debug_pretty!(
    FundingChartData,
    FundingDataPoint,
    TradingViewChartData,
    TransferResult,
    DepositsResponse,
    Deposit,
    WithdrawalsResponse,
    Withdrawal,
    Subaccount,
    PortfolioInfo,
    TransactionLog,
    TransactionLogEntry,
    MassQuoteRequest,
    MassQuoteResponse,
    QuoteResult,
    UserTrade,
    EditOrderRequest,
    BuyOrderRequest,
    SellOrderRequest,
    OrderResponse,
    TradeExecution,
    OrderInfo,
    AccountSummary,
    Currency,
    IndexData,
    WithdrawalPriority,
    IndexPriceData,
    BookSummary,
    ContractSizeResponse,
    TestResponse,
    HelloResponse,
    StatusResponse,
    AprHistoryResponse,
    AprDataPoint,
    TickSizeStep,
    DeliveryPricesResponse,
    DeliveryPriceData,
    ExpirationsResponse,
    FundingRateData,
    SettlementsResponse,
    Settlement,
    LastTradesResponse,
    LastTrade
);
