/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::currency::CurrencyExpirations;
use crate::model::other::DeliveryPriceData;
use crate::model::settlement::Settlement;
use crate::model::trade::{LastTrade, UserTrade};
use crate::model::transaction::TransactionLogEntry;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

pub type UserTradeResponse = Vec<UserTrade>;

/// Contract size response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ContractSizeResponse {
    /// Contract size value
    pub contract_size: f64,
}

/// Test response for connectivity checks
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TestResponse {
    /// Version information
    pub version: String,
}

/// Status response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// Whether the system is locked (optional)
    pub locked: Option<bool>,
    /// Status message (optional)
    pub message: Option<String>,
    /// List of locked indices (optional)
    pub locked_indices: Option<Vec<String>>,
    /// Additional fields that might be present in the API response
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// APR history response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AprHistoryResponse {
    /// List of APR data points
    pub data: Vec<AprDataPoint>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
}

/// Hello response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HelloResponse {
    /// Version string
    pub version: String,
}

/// Delivery prices response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DeliveryPricesResponse {
    /// List of delivery price data
    pub data: Vec<DeliveryPriceData>,
    /// Total number of records
    pub records_total: u32,
}

/// APR data point
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AprDataPoint {
    /// Annual percentage rate
    pub apr: f64,
    /// Timestamp of the data point (optional)
    pub timestamp: Option<u64>,
    /// Day of the data point
    pub day: i32,
}

/// Expirations response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ExpirationsResponse {
    /// Direct future expirations (when currency="any")
    pub future: Option<Vec<String>>,
    /// Direct option expirations (when currency="any")
    pub option: Option<Vec<String>>,
    /// Map of currency to their expirations (when specific currency)
    #[serde(flatten)]
    pub currencies: std::collections::HashMap<String, CurrencyExpirations>,
}

/// Last trades response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct LastTradesResponse {
    /// Whether there are more trades available
    pub has_more: bool,
    /// List of recent trades
    pub trades: Vec<LastTrade>,
}

/// Settlements response structure
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct SettlementsResponse {
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// List of settlement events
    pub settlements: Vec<Settlement>,
}

impl SettlementsResponse {
    /// Create a new settlements response
    pub fn new(settlements: Vec<Settlement>) -> Self {
        Self {
            continuation: None,
            settlements,
        }
    }

    /// Create settlements response with continuation token
    pub fn with_continuation(
        settlements: Vec<crate::model::settlement::Settlement>,
        continuation: String,
    ) -> Self {
        Self {
            continuation: Some(continuation),
            settlements,
        }
    }

    /// Check if there are more results
    pub fn has_more(&self) -> bool {
        self.continuation.is_some()
    }
}

/// Paginated transaction log response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, Default)]
pub struct TransactionLogResponse {
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// List of transaction log entries
    pub logs: Vec<TransactionLogEntry>,
}

/// Transfer result for order-related transfers (e.g., fee rebates)
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TransferResultResponse {
    /// Transfer identifier
    pub id: String,
    /// Transfer status
    pub status: String,
}

/// Account summary information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AccountSummaryResponse {
    /// Account currency (kept as Currencies enum for compatibility)
    pub currency: String,
    /// Total balance
    pub balance: f64,
    /// Account equity
    pub equity: f64,
    /// Available funds for trading
    pub available_funds: f64,
    /// Margin balance
    pub margin_balance: f64,
    /// Unrealized profit and loss
    pub unrealized_pnl: f64,
    /// Realized profit and loss
    pub realized_pnl: f64,
    /// Total profit and loss
    pub total_pl: f64,
    /// Session funding
    pub session_funding: f64,
    /// Session realized P&L
    pub session_rpl: f64,
    /// Session unrealized P&L
    pub session_upl: f64,
    /// Maintenance margin requirement
    pub maintenance_margin: f64,
    /// Initial margin requirement
    pub initial_margin: f64,
    /// Available withdrawal funds
    pub available_withdrawal_funds: Option<f64>,
    /// Cross collateral enabled
    pub cross_collateral_enabled: Option<bool>,
    /// Delta total
    pub delta_total: Option<f64>,
    /// Futures profit and loss
    pub futures_pl: Option<f64>,
    /// Futures session realized profit and loss
    pub futures_session_rpl: Option<f64>,
    /// Futures session unrealized profit and loss
    pub futures_session_upl: Option<f64>,
    /// Options delta
    pub options_delta: Option<f64>,
    /// Options gamma
    pub options_gamma: Option<f64>,
    /// Options profit and loss
    pub options_pl: Option<f64>,
    /// Options session realized profit and loss
    pub options_session_rpl: Option<f64>,
    /// Options session unrealized profit and loss
    pub options_session_upl: Option<f64>,
    /// Options theta
    pub options_theta: Option<f64>,
    /// Options vega
    pub options_vega: Option<f64>,
    /// Portfolio margin enabled
    pub portfolio_margining_enabled: Option<bool>,
    /// Projected delta total
    pub projected_delta_total: Option<f64>,
    /// Projected initial margin
    pub projected_initial_margin: Option<f64>,
    /// Projected maintenance margin
    pub projected_maintenance_margin: Option<f64>,
    /// System name
    pub system_name: Option<String>,
    /// Type of account
    #[serde(rename = "type")]
    pub account_type: String,
    // Additional fields from deribit-http types.rs
    /// Delta total map (currency -> delta)
    pub delta_total_map: std::collections::HashMap<String, f64>,
    /// Deposit address
    pub deposit_address: String,
    /// Fees structure
    pub fees: Vec<std::collections::HashMap<String, f64>>,
    /// Account limits
    pub limits: std::collections::HashMap<String, f64>,
}

impl AccountSummaryResponse {
    /// Calculate margin utilization as percentage
    pub fn margin_utilization(&self) -> f64 {
        if self.equity != 0.0 {
            (self.initial_margin / self.equity) * 100.0
        } else {
            0.0
        }
    }

    /// Calculate available margin
    pub fn available_margin(&self) -> f64 {
        self.equity - self.initial_margin
    }

    /// Check if account is at risk (high margin utilization)
    pub fn is_at_risk(&self, threshold: f64) -> bool {
        self.margin_utilization() > threshold
    }

    /// Calculate return on equity
    pub fn return_on_equity(&self) -> f64 {
        if self.equity != 0.0 {
            (self.total_pl / self.equity) * 100.0
        } else {
            0.0
        }
    }
}
