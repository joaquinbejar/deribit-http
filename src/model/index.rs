/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Index data
#[skip_serializing_none]
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

/// Index chart data point representing a single price observation.
///
/// The Deribit API returns chart data as arrays of `[timestamp, price]` tuples.
/// This struct provides a typed representation with proper field names.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IndexChartDataPoint {
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp: u64,
    /// Average index price at that timestamp
    pub price: f64,
}

impl IndexChartDataPoint {
    /// Creates a new index chart data point.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Timestamp in milliseconds since Unix epoch
    /// * `price` - Average index price at that timestamp
    #[must_use]
    pub fn new(timestamp: u64, price: f64) -> Self {
        Self { timestamp, price }
    }
}

impl Serialize for IndexChartDataPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as [timestamp, price] tuple to match API format
        (self.timestamp, self.price).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IndexChartDataPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize from [timestamp, price] tuple
        let (timestamp_f64, price): (f64, f64) = Deserialize::deserialize(deserializer)?;
        // Convert timestamp from f64 to u64 (API returns it as a number)
        let timestamp = timestamp_f64 as u64;
        Ok(Self { timestamp, price })
    }
}
