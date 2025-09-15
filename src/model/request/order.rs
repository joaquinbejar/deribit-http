/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::order::OrderType;
use crate::model::response::order::LinkedOrderType;
use crate::model::trigger::{TriggerFillCondition, Trigger};
use crate::model::types::TimeInForce;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Order request structure for placing orders on Deribit
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    /// Unique order identifier
    pub order_id: Option<String>,
    /// Name of the instrument to trade
    pub instrument_name: String,
    /// Order amount (for futures and perpetuals)
    pub amount: Option<f64>,
    /// Number of contracts (for options)
    pub contracts: Option<f64>,
    /// Order type (market, limit, etc.)
    #[serde(rename = "type")]
    pub type_: Option<OrderType>,
    /// User-defined label for the order
    pub label: Option<String>,
    /// Limit price for the order
    pub price: Option<f64>,
    /// Time in force specification
    pub time_in_force: Option<TimeInForce>,
    /// Amount to display in the order book
    pub display_amount: Option<f64>,
    /// Whether the order should only be posted (not taken)
    pub post_only: Option<bool>,
    /// Whether to reject if the order would be posted only
    pub reject_post_only: Option<bool>,
    /// Whether this order only reduces position
    pub reduce_only: Option<bool>,
    /// Trigger price for conditional orders
    pub trigger_price: Option<f64>,
    /// Trigger offset for conditional orders
    pub trigger_offset: Option<f64>,
    /// Trigger type for conditional orders
    pub trigger: Option<Trigger>,
    /// Advanced order type (USD or implied volatility)
    pub advanced: Option<AdvancedOrderType>,
    /// Market maker protection flag
    pub mmp: Option<bool>,
    /// Order validity timestamp (Unix timestamp)
    pub valid_until: Option<i64>,
    /// Type of linked order (OTO, OCO, OTOCO)
    pub linked_order_type: Option<LinkedOrderType>,
    /// Trigger fill condition for linked orders
    pub trigger_fill_condition: Option<TriggerFillCondition>,
    /// Configuration for OTOCO (One-Triggers-One-Cancels-Other) orders
    pub otoco_config: Option<Vec<String>>,
}

/// Advanced order type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdvancedOrderType {
    /// USD denomination
    Usd,
    /// Implied volatility
    Implv,
}
