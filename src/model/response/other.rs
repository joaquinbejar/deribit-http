/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use pretty_simple_display::{DebugPretty, DisplaySimple};



/// Trading limit structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TradingLimit {
    pub total: RateLimit,
}

/// Account limits structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AccountLimits {
    pub limits_per_currency: bool,
    pub non_matching_engine: RateLimit,
    pub matching_engine: MatchingEngineLimit,
}

/// Rate limit structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub burst: u32,
    pub rate: u32,
}

/// Matching engine limits
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MatchingEngineLimit {
    pub trading: TradingLimit,
    pub spot: RateLimit,
    pub quotes: RateLimit,
    pub max_quotes: RateLimit,
    pub guaranteed_quotes: RateLimit,
    pub cancel_all: RateLimit,
}

/// Response type for user trades, containing a vector of user trade data
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
#[skip_serializing_none]
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
#[skip_serializing_none]
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
#[skip_serializing_none]
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
#[skip_serializing_none]
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
#[skip_serializing_none]
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
#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AccountSummaryResponse {
    /// Account id
    pub id: u64,
    /// User email
    pub email: String,
    /// System generated user nickname
    pub system_name: String,
    /// Account name (given by user)
    pub username: String,
    /// When Block RFQ Self Match Prevention is enabled
    pub block_rfq_self_match_prevention: bool,
    /// Time at which the account was created (milliseconds since the Unix epoch)
    pub creation_timestamp: u64,
    /// Account type
    #[serde(rename = "type")]
    pub account_type: String,
    /// Optional identifier of the referrer
    pub referrer_id: Option<String>,
    /// Whether account is loginable using email and password
    pub login_enabled: bool,
    /// Whether Security Key authentication is enabled
    pub security_keys_enabled: bool,
    /// Whether MMP is enabled
    pub mmp_enabled: bool,
    /// true when the inter-user transfers are enabled for user
    pub interuser_transfers_enabled: bool,
    /// Self trading rejection behavior - reject_taker or cancel_maker
    pub self_trading_reject_mode: String,
    /// true if self trading rejection behavior is applied to trades between subaccounts
    pub self_trading_extended_to_subaccounts: bool,
    /// Aggregated list of per-currency account summaries
    pub summaries: Vec<AccountResult>,
}

/// Account summary information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AccountResult {
    /// Currency of the summary
    pub currency: String,
    /// The account's balance
    pub balance: f64,
    /// The account's current equity
    pub equity: f64,
    /// The account's available funds
    pub available_funds: f64,
    /// The account's margin balance
    pub margin_balance: f64,
    /// Profit and loss
    pub total_pl: Option<f64>,
    /// Session realized profit and loss
    pub session_rpl: Option<f64>,
    /// Session unrealized profit and loss
    pub session_upl: Option<f64>,
    /// The maintenance margin
    pub maintenance_margin: f64,
    /// The account's initial margin
    pub initial_margin: f64,
    /// The account's available to withdrawal funds
    pub available_withdrawal_funds: Option<f64>,
    /// When true cross collateral is enabled for user
    pub cross_collateral_enabled: Option<bool>,
    /// The sum of position deltas
    pub delta_total: Option<f64>,
    /// Futures profit and Loss
    pub futures_pl: Option<f64>,
    /// Futures session realized profit and Loss
    pub futures_session_rpl: Option<f64>,
    /// Futures session unrealized profit and Loss
    pub futures_session_upl: Option<f64>,
    /// Options summary delta
    pub options_delta: Option<f64>,
    /// Options summary gamma
    pub options_gamma: Option<f64>,
    /// Options profit and Loss
    pub options_pl: Option<f64>,
    /// Options session realized profit and Loss
    pub options_session_rpl: Option<f64>,
    /// Options session unrealized profit and Loss
    pub options_session_upl: Option<f64>,
    /// Options summary theta
    pub options_theta: Option<f64>,
    /// Options summary vega
    pub options_vega: Option<f64>,
    /// true when portfolio margining is enabled for user
    pub portfolio_margining_enabled: Option<bool>,
    /// The sum of position deltas without positions that will expire during closest expiration
    pub projected_delta_total: Option<f64>,
    /// Projected initial margin
    pub projected_initial_margin: Option<f64>,
    /// Projected maintenance margin
    pub projected_maintenance_margin: Option<f64>,
    /// Delta total map (currency -> delta)
    pub delta_total_map: Option<std::collections::HashMap<String, f64>>,
    /// The deposit address for the account (if available)
    pub deposit_address: Option<String>,
    /// List of fee objects for all currency pairs and instrument types
    pub fees: Option<Vec<FeeStructure>>,
    /// Account limits structure
    pub limits: Option<AccountLimits>,
    /// Name of user's currently enabled margin model
    pub margin_model: Option<String>,
    /// Map of options' gammas per index
    pub options_gamma_map: Option<std::collections::HashMap<String, f64>>,
    /// Map of options' thetas per index
    pub options_theta_map: Option<std::collections::HashMap<String, f64>>,
    /// Map of options' vegas per index
    pub options_vega_map: Option<std::collections::HashMap<String, f64>>,
    /// Options value
    pub options_value: Option<f64>,
    /// The account's balance reserved in active spot orders
    pub spot_reserve: Option<f64>,
    /// Estimated Liquidation Ratio
    pub estimated_liquidation_ratio: Option<f64>,
    /// Estimated liquidation ratio map
    pub estimated_liquidation_ratio_map: Option<std::collections::HashMap<String, f64>>,
    /// The account's fee balance (it can be used to pay for fees)
    pub fee_balance: Option<f64>,
    /// The account's balance reserved in other orders
    pub additional_reserve: Option<f64>,

    // Additional fields for cross-collateral users
    /// Optional field returned with value true when user has non block chain equity
    pub has_non_block_chain_equity: Option<bool>,
    /// The account's total margin balance in all cross collateral currencies, expressed in USD
    pub total_margin_balance_usd: Option<f64>,
    /// The account's total delta total in all cross collateral currencies, expressed in USD
    pub total_delta_total_usd: Option<f64>,
    /// The account's total initial margin in all cross collateral currencies, expressed in USD
    pub total_initial_margin_usd: Option<f64>,
    /// The account's total maintenance margin in all cross collateral currencies, expressed in USD
    pub total_maintenance_margin_usd: Option<f64>,
    /// The account's total equity in all cross collateral currencies, expressed in USD
    pub total_equity_usd: Option<f64>,
    /// System name for the account
    pub system_name: Option<String>,
    /// Account type
    pub account_type: Option<String>,
}

