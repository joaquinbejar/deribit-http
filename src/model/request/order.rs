/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::order::OrderType;
use crate::model::response::order::LinkedOrderType;
use crate::model::trigger::{TriggerFillCondition, TriggerType};
use crate::model::types::TimeInForce;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub order_id: Option<String>,
    /// Name of the instrument to trade
    pub instrument_name: String,
    /// Amount/quantity to buy
    pub amount: Option<f64>,
    pub contracts: Option<f64>,
    /// Type of order to place
    #[serde(rename = "type")]
    pub type_: Option<OrderType>,
    /// User-defined label for the order
    pub label: Option<String>,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// Time in force specification
    pub time_in_force: Option<TimeInForce>,
    pub display_amount: Option<f64>,
    /// Whether this is a post-only order
    pub post_only: Option<bool>,
    pub reject_post_only: Option<bool>,
    /// Whether this order only reduces position
    pub reduce_only: Option<bool>,
    pub trigger_price: Option<f64>,
    pub trigger_offset: Option<f64>,
    pub trigger: Option<TriggerType>,
    pub advanced: Option<AdvancedOrderType>,
    pub mmp: Option<bool>,
    pub valid_until: Option<i64>,
    pub linked_order_type: Option<LinkedOrderType>,
    pub trigger_fill_condition: Option<TriggerFillCondition>,
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
