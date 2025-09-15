/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::instrument::InstrumentKind;
use crate::model::order::OrderSide;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Trade execution
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    /// Trade amount
    pub amount: f64,
    /// Trade direction (buy/sell)
    pub direction: String,
    /// Trading fee paid
    pub fee: f64,
    /// Currency of the trading fee
    pub fee_currency: String,
    /// Index price at execution time
    pub index_price: f64,
    /// Name of the traded instrument
    pub instrument_name: String,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// User-defined label for the trade
    pub label: String,
    /// Liquidity type (maker/taker)
    pub liquidity: String,
    /// Mark price at execution time
    pub mark_price: f64,
    /// Matching engine identifier
    pub matching_id: Option<String>,
    /// Order ID that generated this trade
    pub order_id: String,
    /// Type of the order that generated this trade
    pub order_type: String,
    /// Original order type before modifications
    pub original_order_type: Option<String>,
    /// Execution price
    pub price: f64,
    /// Whether this was a self trade
    pub self_trade: bool,
    /// Current state of the trade
    pub state: String,
    /// Price tick direction (1=up, -1=down, 0=no change)
    pub tick_direction: i32,
    /// Execution timestamp
    pub timestamp: u64,
    /// Unique trade identifier
    pub trade_id: String,
    /// Trade sequence number
    pub trade_seq: u64,
    /// Underlying asset price (for derivatives)
    pub underlying_price: Option<f64>,
}

/// User trade information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct UserTrade {
    /// Trade amount
    pub amount: f64,
    /// Trade direction (buy/sell)
    pub direction: String,
    /// Trading fee paid
    pub fee: f64,
    /// Currency of the trading fee
    pub fee_currency: String,
    /// Index price at execution time
    pub index_price: f64,
    /// Name of the traded instrument
    pub instrument_name: String,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// User-defined label for the trade
    pub label: String,
    /// Liquidity type (maker/taker)
    pub liquidity: String,
    /// Mark price at execution time
    pub mark_price: f64,
    /// Matching engine identifier
    pub matching_id: Option<String>,
    /// Order ID that generated this trade
    pub order_id: String,
    /// Type of the order that generated this trade
    pub order_type: String,
    /// Original order type before modifications
    pub original_order_type: Option<String>,
    /// Execution price
    pub price: f64,
    /// Whether this was a self trade
    pub self_trade: bool,
    /// Current state of the trade
    pub state: String,
    /// Price tick direction (1=up, -1=down, 0=no change)
    pub tick_direction: i32,
    /// Execution timestamp
    pub timestamp: u64,
    /// Unique trade identifier
    pub trade_id: String,
    /// Trade sequence number
    pub trade_seq: u64,
    /// Underlying asset price (for derivatives)
    pub underlying_price: Option<f64>,
}

/// Last trade
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct LastTrade {
    /// Trade amount
    pub amount: f64,
    /// Trade direction (buy/sell)
    pub direction: String,
    /// Index price at execution time
    pub index_price: f64,
    /// Name of the traded instrument
    pub instrument_name: String,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// Liquidity information
    pub liquid: Option<String>,
    /// Execution price
    pub price: f64,
    /// Price tick direction (1=up, -1=down, 0=no change)
    pub tick_direction: i32,
    /// Execution timestamp
    pub timestamp: u64,
    /// Unique trade identifier
    pub trade_id: String,
    /// Trade sequence number
    pub trade_seq: u64,
}

/// Liquidity type enumeration
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Liquidity {
    /// Maker (provided liquidity)
    #[serde(rename = "M")]
    Maker,
    /// Taker (consumed liquidity)
    #[serde(rename = "T")]
    Taker,
    /// Mixed (both maker and taker in same trade)
    #[serde(rename = "MT")]
    Mixed,
}

/// Trade execution information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Unique trade identifier
    pub trade_id: String,
    /// Instrument name
    pub instrument_name: String,
    /// Order ID that generated this trade
    pub order_id: String,
    /// Trade direction (buy/sell)
    pub direction: OrderSide,
    /// Trade amount
    pub amount: f64,
    /// Execution price
    pub price: f64,
    /// Trade timestamp
    pub timestamp: i64,
    /// Fee amount
    pub fee: f64,
    /// Fee currency
    pub fee_currency: String,
    /// Liquidity type (maker/taker)
    pub liquidity: Liquidity,
    /// Mark price at time of trade
    pub mark_price: f64,
    /// Index price at time of trade
    pub index_price: f64,
    /// Instrument kind
    pub instrument_kind: Option<InstrumentKind>,
    /// Trade sequence number
    pub trade_seq: Option<u64>,
    /// User role in the trade
    pub user_role: Option<String>,
    /// Whether this is a block trade
    pub block_trade: Option<bool>,
    /// Underlying price (for options)
    pub underlying_price: Option<f64>,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// Label associated with the order
    pub label: Option<String>,
    /// Profit and loss from this trade
    pub profit_loss: Option<f64>,
    /// Tick direction
    pub tick_direction: Option<i32>,
    /// Whether this trade was self-traded
    pub self_trade: Option<bool>,
}

impl Trade {
    /// Calculate the notional value of the trade
    pub fn notional_value(&self) -> f64 {
        self.amount * self.price
    }

    /// Check if this was a maker trade
    pub fn is_maker(&self) -> bool {
        matches!(self.liquidity, Liquidity::Maker | Liquidity::Mixed)
    }

    /// Check if this was a taker trade
    pub fn is_taker(&self) -> bool {
        matches!(self.liquidity, Liquidity::Taker | Liquidity::Mixed)
    }

    /// Check if this is a buy trade
    pub fn is_buy(&self) -> bool {
        self.direction == OrderSide::Buy
    }

    /// Check if this is a sell trade
    pub fn is_sell(&self) -> bool {
        self.direction == OrderSide::Sell
    }

    /// Get fee as percentage of notional
    pub fn fee_percentage(&self) -> f64 {
        if self.notional_value() != 0.0 {
            (self.fee / self.notional_value()) * 100.0
        } else {
            0.0
        }
    }
}

/// Trade statistics
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TradeStats {
    /// Total number of trades
    pub count: u64,
    /// Total volume
    pub volume: f64,
    /// Total fees paid
    pub total_fees: f64,
    /// Average price
    pub avg_price: f64,
    /// Profit and loss
    pub pnl: f64,
    /// Number of winning trades
    pub winning_trades: u64,
    /// Number of losing trades
    pub losing_trades: u64,
}

impl TradeStats {
    /// Create empty trade statistics
    pub fn new() -> Self {
        Self {
            count: 0,
            volume: 0.0,
            total_fees: 0.0,
            avg_price: 0.0,
            pnl: 0.0,
            winning_trades: 0,
            losing_trades: 0,
        }
    }

    /// Calculate win rate as percentage
    pub fn win_rate(&self) -> f64 {
        if self.count > 0 {
            (self.winning_trades as f64 / self.count as f64) * 100.0
        } else {
            0.0
        }
    }
}

impl Default for TradeStats {
    fn default() -> Self {
        Self::new()
    }
}
