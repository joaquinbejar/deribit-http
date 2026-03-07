/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Market Maker Protection (MMP) configuration
///
/// Contains the MMP parameters for a specific index and optional MMP group.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MmpConfig {
    /// Index identifier (e.g., "btc_usd", "eth_usd")
    pub index_name: String,
    /// MMP group name (for Mass Quotes)
    pub mmp_group: Option<String>,
    /// Monitoring window duration in seconds
    pub interval: u32,
    /// Time in seconds that MMP remains active after being triggered
    /// (0 = manual reset required)
    pub frozen_time: u32,
    /// Quantity limit for MMP
    pub quantity_limit: Option<f64>,
    /// Delta limit for MMP
    pub delta_limit: Option<f64>,
    /// Vega limit for MMP
    pub vega_limit: Option<f64>,
    /// Maximum quote quantity per side per order book
    pub max_quote_quantity: Option<f64>,
}

/// Market Maker Protection (MMP) status
///
/// Contains the current MMP status for a triggered index or MMP group.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MmpStatus {
    /// Index identifier (e.g., "btc_usd", "eth_usd")
    pub index_name: String,
    /// Timestamp (milliseconds since UNIX epoch) until the user is frozen
    /// (0 = frozen until manual reset)
    pub frozen_until: u64,
    /// MMP group name (for Mass Quotes, optional)
    pub mmp_group: Option<String>,
}

/// Request to set MMP configuration
///
/// Used to configure Market Maker Protection for a specific index.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Default, Serialize, Deserialize)]
pub struct SetMmpConfigRequest {
    /// Index identifier (e.g., "btc_usd", "eth_usd")
    pub index_name: String,
    /// Monitoring window duration in seconds (0 = remove MMP configuration)
    pub interval: u32,
    /// Time in seconds that MMP remains active after being triggered
    /// (0 = manual reset required)
    pub frozen_time: u32,
    /// Quantity limit for MMP
    pub quantity_limit: Option<f64>,
    /// Delta limit for MMP
    pub delta_limit: Option<f64>,
    /// Vega limit for MMP
    pub vega_limit: Option<f64>,
    /// Maximum quote quantity per side per order book (required)
    pub max_quote_quantity: Option<f64>,
    /// MMP group name (for Mass Quotes)
    pub mmp_group: Option<String>,
    /// If true, configure MMP for Block RFQ
    pub block_rfq: Option<bool>,
}
