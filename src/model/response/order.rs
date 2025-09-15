/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::trade::TradeExecution;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Order response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    /// Order information
    pub order: OrderInfoResponse,
    /// List of trade executions for the order
    pub trades: Vec<TradeExecution>,
}

/// Types of linked orders supported by Deribit
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkedOrderType {
    /// One order triggers another (OTO)
    OneTriggersOther,
    /// One order cancels another (OCO)
    OneCancelsOther,
    /// One order triggers another and cancels a third (OTOCO)
    OneTriggersOneCancelsOther,
}

/// Order information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderInfoResponse {
    /// Order amount
    pub amount: f64,
    /// Whether order was placed via API
    pub api: bool,
    /// Average execution price
    pub average_price: f64,
    /// Order creation timestamp
    pub creation_timestamp: u64,
    /// Order direction (buy/sell)
    pub direction: String,
    /// Amount that has been filled
    pub filled_amount: f64,
    /// Instrument name
    pub instrument_name: String,
    /// Whether this is a liquidation order
    pub is_liquidation: bool,
    /// Order label
    pub label: String,
    /// Last update timestamp
    pub last_update_timestamp: u64,
    /// Maximum amount to show in order book (optional)
    pub max_show: Option<f64>,
    /// Unique order identifier
    pub order_id: String,
    /// Current order state
    pub order_state: String,
    /// Type of order
    pub order_type: String,
    /// Original order type before any modifications
    pub original_order_type: Option<String>,
    /// Whether this is a post-only order
    pub post_only: bool,
    /// Order price
    pub price: f64,
    /// Current profit/loss on the order
    pub profit_loss: Option<f64>,
    /// Whether this order only reduces position
    pub reduce_only: bool,
    /// Whether this order has been replaced
    pub replaced: bool,
    /// Whether this order reduces risk
    pub risk_reducing: bool,
    /// Time in force specification
    pub time_in_force: String,
    /// Whether the order has been triggered
    pub triggered: Option<bool>,
    /// Trigger condition for the order
    pub trigger: Option<String>,
    /// USD value of the order
    pub usd: Option<f64>,
    /// Whether order was placed via web interface
    pub web: bool,
}
