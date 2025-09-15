/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// TradingView chart data structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TradingViewChartData {
    /// Status of the data
    pub status: String,
    /// Array of timestamps
    pub ticks: Vec<u64>,
    /// Array of open prices
    pub open: Vec<f64>,
    /// Array of high prices
    pub high: Vec<f64>,
    /// Array of low prices
    pub low: Vec<f64>,
    /// Array of close prices
    pub close: Vec<f64>,
    /// Array of volumes
    pub volume: Vec<f64>,
    /// Array of costs
    pub cost: Vec<f64>,
}

impl TradingViewChartData {
    /// Create new TradingView chart data
    pub fn new() -> Self {
        Self {
            status: "ok".to_string(),
            ticks: Vec::new(),
            open: Vec::new(),
            high: Vec::new(),
            low: Vec::new(),
            close: Vec::new(),
            volume: Vec::new(),
            cost: Vec::new(),
        }
    }

    /// Add a new candle to the data
    #[allow(clippy::too_many_arguments)]
    pub fn add_candle(
        &mut self,
        timestamp: u64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        cost: f64,
    ) {
        self.ticks.push(timestamp);
        self.open.push(open);
        self.high.push(high);
        self.low.push(low);
        self.close.push(close);
        self.volume.push(volume);
        self.cost.push(cost);
    }
}

impl Default for TradingViewChartData {
    fn default() -> Self {
        Self::new()
    }
}