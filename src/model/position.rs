/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::types::Direction;

/// Position structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Average price of the position
    pub average_price: f64,
    /// Average price in USD
    pub average_price_usd: Option<f64>,
    /// Delta (price sensitivity) of the position
    pub delta: Option<f64>,
    /// Direction of the position (buy/sell)
    pub direction: Direction,
    /// Estimated liquidation price
    pub estimated_liquidation_price: Option<f64>,
    /// Floating profit/loss
    pub floating_profit_loss: Option<f64>,
    /// Floating profit/loss in USD
    pub floating_profit_loss_usd: Option<f64>,
    /// Gamma (delta sensitivity) of the position
    pub gamma: Option<f64>,
    /// Current index price
    pub index_price: Option<f64>,
    /// Initial margin requirement
    pub initial_margin: Option<f64>,
    /// Name of the instrument
    pub instrument_name: String,
    /// Interest value
    pub interest_value: Option<f64>,
    /// Instrument kind (future, option, etc.)
    pub kind: Option<String>,
    /// Leverage used for the position
    pub leverage: Option<i32>,
    /// Maintenance margin requirement
    pub maintenance_margin: Option<f64>,
    /// Current mark price
    pub mark_price: Option<f64>,
    /// Margin used by open orders
    pub open_orders_margin: Option<f64>,
    /// Realized funding payments
    pub realized_funding: Option<f64>,
    /// Realized profit/loss
    pub realized_profit_loss: Option<f64>,
    /// Settlement price
    pub settlement_price: Option<f64>,
    /// Position size
    pub size: f64,
    /// Position size in currency units
    pub size_currency: Option<f64>,
    /// Theta (time decay) of the position
    pub theta: Option<f64>,
    /// Total profit/loss
    pub total_profit_loss: Option<f64>,
    /// Vega (volatility sensitivity) of the position
    pub vega: Option<f64>,
    /// Unrealized profit/loss
    pub unrealized_profit_loss: Option<f64>,
}