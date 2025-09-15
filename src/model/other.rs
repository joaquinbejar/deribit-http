/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::OptionType;
use crate::model::instrument::Instrument;
use crate::model::ticker::TickerData;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Delivery price data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceData {
    /// Date of the delivery price
    pub date: String,
    /// Delivery price value
    pub delivery_price: f64,
}

/// Greeks sub-structure for options
#[skip_serializing_none]
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
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OptionInstrumentPair {
    /// Call option instrument data, if available
    pub call: Option<OptionInstrument>,
    /// Put option instrument data, if available  
    pub put: Option<OptionInstrument>,
}

/// Parsed option instrument with ticker data
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize)]
pub struct ParsedOptionWithTicker {
    /// The instrument name (e.g., "BTC-25DEC21-50000-C")
    pub instrument_name: String,
    /// Strike price of the option
    pub strike: f64,
    /// Type of option (Call or Put)
    pub option_type: OptionType,
    /// Expiry date string
    pub expiry: String,
    /// Associated ticker data
    pub ticker: TickerData,
}

/// Sort direction options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// Ascending sort order
    #[default]
    Asc,
    /// Descending sort order
    Desc,
    /// Default sort order (platform-specific)
    Default,
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Asc => write!(f, "asc"),
            SortDirection::Desc => write!(f, "desc"),
            SortDirection::Default => write!(f, "default"),
        }
    }
}
