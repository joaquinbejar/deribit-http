/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Funding chart data structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct FundingChartData {
    /// Current interest rate
    pub current_interest: f64,
    /// 8h interest rate
    pub interest_8h: f64,
    /// Historical funding data points
    pub data: Vec<FundingDataPoint>,
}

impl FundingChartData {
    /// Create new funding chart data
    pub fn new() -> Self {
        Self {
            current_interest: 0.0,
            interest_8h: 0.0,
            data: Vec::new(),
        }
    }
}

impl Default for FundingChartData {
    fn default() -> Self {
        Self::new()
    }
}

/// Funding data point structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct FundingDataPoint {
    /// Index price at the time
    pub index_price: f64,
    /// 8h interest rate
    pub interest_8h: f64,
    /// Timestamp of the data point
    pub timestamp: u64,
}

impl FundingDataPoint {
    /// Create new funding data point
    pub fn new(index_price: f64, interest_8h: f64, timestamp: u64) -> Self {
        Self {
            index_price,
            interest_8h,
            timestamp,
        }
    }
}

/// Funding rate data structure for historical funding rates
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct FundingRateData {
    /// Timestamp of the funding event
    pub timestamp: u64,
    /// Index price at the time
    pub index_price: f64,
    /// 8h interest rate
    pub interest_8h: f64,
    /// 1h interest rate
    pub interest_1h: f64,
    /// Previous index price
    pub prev_index_price: f64,
}

impl FundingRateData {
    /// Create new funding rate data
    pub fn new(
        timestamp: u64,
        index_price: f64,
        interest_8h: f64,
        interest_1h: f64,
        prev_index_price: f64,
    ) -> Self {
        Self {
            timestamp,
            index_price,
            interest_8h,
            interest_1h,
            prev_index_price,
        }
    }
}
