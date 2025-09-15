/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Transaction type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TransactionType {
    /// Deposit transaction
    Deposit,
    /// Withdrawal transaction
    Withdrawal,
    /// Trade transaction (default)
    #[default]
    Trade,
    /// Transfer transaction
    Transfer,
    /// Fee transaction
    Fee,
    /// Funding transaction
    Funding,
    /// Bonus transaction
    Bonus,
    /// Dividend transaction
    Dividend,
    /// Liquidation transaction
    Liquidation,
    /// Insurance transaction
    Insurance,
}

/// Generic transaction log entry
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TransactionLogEntry {
    /// Unique identifier
    pub id: u64,
    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,
    /// It represents the requested order size. For perpetual and inverse futures the amount is in USD units.
    /// For options and linear futures it is the underlying base currency coin.
    pub amount: Option<f64>,
    /// Cash balance after the transaction
    pub balance: f64,
    /// The timestamp (milliseconds since the Unix epoch)
    pub timestamp: u64,
    /// Transaction category/type. Common types: trade, deposit, withdrawal, settlement, delivery, transfer, swap, correction
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Additional information regarding transaction. Strongly dependent on the log entry type
    pub info: Option<serde_json::Value>,
    /// Change in cash balance. For trades: fees and options premium paid/received.
    /// For settlement: Futures session PNL and perpetual session funding.
    pub change: f64,
    /// For futures and perpetual contracts: Realized session PNL (since last settlement).
    /// For options: the amount paid or received for the options traded.
    pub cashflow: f64,
    /// Unique user identifier
    pub user_id: u64,
    /// Unique (per currency) trade identifier
    pub trade_id: Option<String>,
    /// Unique order identifier
    pub order_id: Option<String>,
    /// Updated position size after the transaction
    pub position: Option<f64>,
    /// One of: short or long in case of settlements, close sell or close buy in case of deliveries,
    /// open sell, open buy, close sell, close buy in case of trades
    pub side: Option<TransactionSide>,
    /// It represents the order size in contract units. (Optional, may be absent in historical data).
    pub contracts: Option<f64>,
    /// Actual funding rate of trades and settlements on perpetual instruments
    pub interest_pl: Option<f64>,
    /// Trade role of the user: maker or taker
    pub user_role: Option<UserRole>,
    /// Fee role of the user: maker or taker. Can be different from trade role when iceberg order was involved.
    pub fee_role: Option<String>,
    /// The index price for the instrument during the delivery
    pub index_price: Option<f64>,
    /// Settlement/delivery price or the price level of the traded contracts
    pub price: Option<f64>,
    /// Sequential identifier of user transaction
    pub user_seq: u64,
    /// The settlement price for the instrument during the delivery
    pub settlement_price: Option<f64>,
    /// Currency symbol associated with the price field value
    pub price_currency: Option<String>,
    /// Updated equity value after the transaction
    pub equity: f64,
    /// Total session funding rate
    pub total_interest_pl: Option<f64>,
    /// Session unrealized profit and loss
    pub session_upl: Option<f64>,
    /// Indicator informing whether the cashflow is waiting for settlement or not
    pub profit_as_cashflow: Option<bool>,
    /// Commission paid so far (in base currency)
    pub commission: Option<f64>,
    /// Session realized profit and loss
    pub session_rpl: Option<f64>,
    /// Market price during the trade
    pub mark_price: Option<f64>,
    /// ID of the Block RFQ - when trade was part of the Block RFQ
    pub block_rfq_id: Option<u64>,
    /// The IP address from which the trade was initiated
    pub ip: Option<String>,
    /// System name or user defined subaccount alias
    pub username: String,
    /// Unique instrument identifier
    pub instrument_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionSide {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "open buy")]
    OpenBuy,
    #[serde(rename = "open sell")]
    OpenSell,
    #[serde(rename = "close buy")]
    CloseBuy,
    #[serde(rename = "close sell")]
    CloseSell,
    #[serde(rename = "-")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Maker,
    Taker,
}



#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, Default)]
pub struct TransactionLogRequest {
    pub currency: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub query: Option<String>,
    pub count: Option<u64>,
    pub subaccount_id: Option<u64>,
    pub continuation: Option<u64>,
}