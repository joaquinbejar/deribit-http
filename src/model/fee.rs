/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Fee structure for different trading types
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct FeeStructure {
    /// The currency pair this fee applies to
    pub index_name: String,
    /// Instrument type (e.g., future, perpetual, option)
    pub kind: String,
    /// Fee values
    pub value: FeeValue,
}

/// Fee values for different fee types
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct FeeValue {
    /// Default fee structure
    pub default: DefaultFee,
    /// Block trade fee (if applicable)
    pub block_trade: Option<f64>,
    /// Settlement fee
    pub settlement: Option<f64>,
}

/// Default fee structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DefaultFee {
    /// Fee calculation type (e.g., fixed, relative)
    #[serde(rename = "type")]
    pub fee_type: String,
    /// Taker fee
    pub taker: f64,
    /// Maker fee
    pub maker: f64,
}
