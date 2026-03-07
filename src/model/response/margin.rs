/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Response from the get_margins endpoint
///
/// Contains margin requirements for a hypothetical order on a given instrument.
/// This is useful for estimating margin requirements before placing an order.
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MarginsResponse {
    /// Margin required when buying
    pub buy: f64,
    /// Margin required when selling
    pub sell: f64,
    /// The minimum price for the future. Any sell orders submitted lower than
    /// this price will be clamped to this minimum.
    pub min_price: f64,
    /// The maximum price for the future. Any buy orders submitted higher than
    /// this price will be clamped to this maximum.
    pub max_price: f64,
}

/// Response from the get_order_margin_by_ids endpoint
///
/// Contains initial margin requirements for an order identified by its ID.
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderMargin {
    /// Unique order identifier
    pub order_id: String,
    /// Initial margin required for the order
    pub initial_margin: f64,
    /// Currency of the initial margin
    pub initial_margin_currency: String,
}
