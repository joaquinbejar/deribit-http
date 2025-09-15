/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::instrument::Instrument;
use crate::model::OptionType;
use crate::model::ticker::TickerData;

/// Delivery price data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceData {
    /// Date of the delivery price
    pub date: String,
    /// Delivery price value
    pub delivery_price: f64,
}

/// Greeks sub-structure for options
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Greeks {
    /// Delta value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    /// Gamma value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    /// Vega value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega: Option<f64>,
    /// Theta value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Rho value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rho: Option<f64>,
}

/// Combined option instrument data with ticker information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OptionInstrument {
    /// The instrument details
    pub instrument: Instrument,
    /// Real-time ticker data for the option
    pub ticker: TickerData,
}

/// A pair of option instruments representing both call and put options for the same underlying asset
///
/// This structure groups together the call and put options for a specific underlying asset,
/// allowing for easy access to both sides of an option strategy. Both options are optional,
/// meaning you can have just a call, just a put, or both.
///
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OptionInstrumentPair {
    /// Call option instrument data, if available
    pub call: Option<OptionInstrument>,
    /// Put option instrument data, if available  
    pub put: Option<OptionInstrument>,
}


#[derive(DebugPretty, DisplaySimple, Clone, Serialize)]
pub struct ParsedOptionWithTicker {
    instrument_name: String,
    strike: f64,
    option_type: OptionType,
    expiry: String,
    ticker: TickerData,
}