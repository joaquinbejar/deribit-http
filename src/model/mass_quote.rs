/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::order::OrderSide;

/// Transfer result for order-related transfers (e.g., fee rebates)
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// Transfer identifier
    pub id: String,
    /// Transfer status
    pub status: String,
}

/// Mass quote request item
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MassQuoteItem {
    /// Name of the instrument to quote
    pub instrument_name: String,
    /// Order side (buy or sell)
    pub side: OrderSide,
    /// Quote amount/quantity
    pub amount: f64,
    /// Quote price
    pub price: f64,
}

/// Quote result
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct QuoteResult {
    /// Name of the instrument that was quoted
    pub instrument_name: String,
    /// Whether the quote was successful
    pub success: bool,
    /// Error message if quote failed
    pub error: Option<String>,
}