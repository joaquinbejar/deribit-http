/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Index data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct IndexData {
    /// BTC component (optional)
    pub btc: Option<f64>,
    /// ETH component (optional)
    pub eth: Option<f64>,
    /// USDC component (optional)
    pub usdc: Option<f64>,
    /// USDT component (optional)
    pub usdt: Option<f64>,
    /// EURR component (optional)
    pub eurr: Option<f64>,
    /// EDP (Estimated Delivery Price)
    pub edp: f64,
}

/// Index price data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct IndexPriceData {
    /// Current index price
    pub index_price: f64,
    /// Estimated delivery price
    pub estimated_delivery_price: f64,
}
