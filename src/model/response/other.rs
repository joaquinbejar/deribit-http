/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::prelude::*;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Trading limit structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Default, Serialize, Deserialize)]
pub struct TradingLimit {
    /// Total rate limit for trading operations
    #[serde(default)]
    pub total: RateLimit,
}

/// Account limits structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Default, Serialize, Deserialize)]
pub struct AccountLimits {
    /// Whether limits are applied per currency
    #[serde(default)]
    pub limits_per_currency: bool,
    /// Rate limits for non-matching engine operations
    #[serde(default)]
    pub non_matching_engine: RateLimit,
    /// Rate limits for matching engine operations
    #[serde(default)]
    pub matching_engine: MatchingEngineLimit,
}

/// Rate limit structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Default, Serialize, Deserialize)]
pub struct RateLimit {
    /// Maximum burst capacity for rate limiting
    #[serde(default)]
    pub burst: u32,
    /// Rate limit per time unit
    #[serde(default)]
    pub rate: u32,
}

/// Matching engine limits
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Default, Serialize, Deserialize)]
pub struct MatchingEngineLimit {
    /// Trading limits configuration
    #[serde(default)]
    pub trading: TradingLimit,
    /// Spot trading rate limits
    #[serde(default)]
    pub spot: RateLimit,
    /// Quote request rate limits
    #[serde(default)]
    pub quotes: RateLimit,
    /// Maximum quotes rate limits
    #[serde(default)]
    pub max_quotes: RateLimit,
    /// Guaranteed quotes rate limits
    #[serde(default)]
    pub guaranteed_quotes: RateLimit,
    /// Cancel all orders rate limits
    #[serde(default)]
    pub cancel_all: RateLimit,
}

/// Response type for user trades, containing a vector of user trade data
pub type UserTradeResponse = Vec<UserTrade>;

/// Response type for user trades with pagination info (used by instrument-specific endpoints)
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct UserTradeWithPaginationResponse {
    /// List of user trades
    pub trades: Vec<UserTrade>,
    /// Whether there are more trades available
    pub has_more: bool,
}

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
    /// Continuation token for pagination. NULL when no continuation.
    pub continuation: Option<u64>,
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

/// Shared account-level fields returned by both singular and plural account summary endpoints.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account id
    pub id: u64,
    /// User email
    pub email: String,
    /// System generated user nickname
    pub system_name: Option<String>,
    /// Account name (given by user)
    pub username: Option<String>,
    /// When Block RFQ Self Match Prevention is enabled
    pub block_rfq_self_match_prevention: Option<bool>,
    /// Time at which the account was created (milliseconds since the Unix epoch)
    pub creation_timestamp: Option<u64>,
    /// Account type
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    /// Optional identifier of the referrer
    pub referrer_id: Option<String>,
    /// Whether account is loginable using email and password
    pub login_enabled: Option<bool>,
    /// Whether Security Key authentication is enabled
    pub security_keys_enabled: Option<bool>,
    /// Whether MMP is enabled
    pub mmp_enabled: Option<bool>,
    /// true when the inter-user transfers are enabled for user
    pub interuser_transfers_enabled: Option<bool>,
    /// Self trading rejection behavior - reject_taker or cancel_maker
    pub self_trading_reject_mode: Option<String>,
    /// true if self trading rejection behavior is applied to trades between subaccounts
    pub self_trading_extended_to_subaccounts: Option<bool>,
}

/// Account summary response containing user account information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AccountSummaryResponse {
    /// Account id
    #[serde(default)]
    pub id: u64,
    /// User email
    #[serde(default)]
    pub email: String,
    /// System generated user nickname
    #[serde(default)]
    pub system_name: String,
    /// Account name (given by user)
    #[serde(default)]
    pub username: String,
    /// When Block RFQ Self Match Prevention is enabled
    #[serde(default)]
    pub block_rfq_self_match_prevention: bool,
    /// Time at which the account was created (milliseconds since the Unix epoch)
    #[serde(default)]
    pub creation_timestamp: u64,
    /// Account type
    #[serde(rename = "type", default)]
    pub account_type: String,
    /// Optional identifier of the referrer
    pub referrer_id: Option<String>,
    /// Whether account is loginable using email and password
    #[serde(default)]
    pub login_enabled: bool,
    /// Whether Security Key authentication is enabled
    #[serde(default)]
    pub security_keys_enabled: bool,
    /// Whether MMP is enabled
    #[serde(default)]
    pub mmp_enabled: bool,
    /// true when the inter-user transfers are enabled for user
    #[serde(default)]
    pub interuser_transfers_enabled: bool,
    /// Self trading rejection behavior - reject_taker or cancel_maker
    #[serde(default)]
    pub self_trading_reject_mode: String,
    /// true if self trading rejection behavior is applied to trades between subaccounts
    #[serde(default)]
    pub self_trading_extended_to_subaccounts: bool,
    /// Aggregated list of per-currency account summaries
    #[serde(default)]
    pub summaries: Vec<AccountResult>,
}

/// Response from `get_account_summaries` (plural, all currencies).
///
/// Returns account-level fields with a `summaries` array containing
/// per-currency financial data.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AccountSummariesResponse {
    /// Account-level fields (id, email, type, etc.)
    #[serde(flatten)]
    pub account: AccountInfo,
    /// Per-currency account summaries
    pub summaries: Vec<AccountResult>,
}

/// Mark price history data point
///
/// Represents a single data point in mark price history.
/// The API returns data as `[timestamp_ms, mark_price]` arrays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "(u64, f64)", into = "(u64, f64)")]
pub struct MarkPriceHistoryPoint {
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp: u64,
    /// Mark price value
    pub mark_price: f64,
}

impl From<(u64, f64)> for MarkPriceHistoryPoint {
    fn from((timestamp, mark_price): (u64, f64)) -> Self {
        Self {
            timestamp,
            mark_price,
        }
    }
}

impl From<MarkPriceHistoryPoint> for (u64, f64) {
    fn from(point: MarkPriceHistoryPoint) -> Self {
        (point.timestamp, point.mark_price)
    }
}

/// Index name information with extended details
///
/// Represents an index with optional combo trading availability flags.
/// Returned by `get_supported_index_names` when `extended=true`.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexNameInfo {
    /// Index name identifier (e.g., "btc_eth", "btc_usdc")
    pub name: String,
    /// Whether future combo creation is enabled for this index
    pub future_combo_enabled: Option<bool>,
    /// Whether option combo creation is enabled for this index
    pub option_combo_enabled: Option<bool>,
}

/// Aggregated trade volume by currency
///
/// Contains 24-hour trade volumes for different instrument types.
/// When `extended=true`, also includes 7-day and 30-day volumes.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeVolume {
    /// Currency (e.g., "BTC", "ETH", "USDC")
    pub currency: String,
    /// Total 24h trade volume for call options
    pub calls_volume: f64,
    /// Total 24h trade volume for put options
    pub puts_volume: f64,
    /// Total 24h trade volume for futures
    pub futures_volume: f64,
    /// Total 24h trade volume for spot
    pub spot_volume: f64,
    /// Total 7d trade volume for call options (extended only)
    pub calls_volume_7d: Option<f64>,
    /// Total 7d trade volume for put options (extended only)
    pub puts_volume_7d: Option<f64>,
    /// Total 7d trade volume for futures (extended only)
    pub futures_volume_7d: Option<f64>,
    /// Total 7d trade volume for spot (extended only)
    pub spot_volume_7d: Option<f64>,
    /// Total 30d trade volume for call options (extended only)
    pub calls_volume_30d: Option<f64>,
    /// Total 30d trade volume for put options (extended only)
    pub puts_volume_30d: Option<f64>,
    /// Total 30d trade volume for futures (extended only)
    pub futures_volume_30d: Option<f64>,
    /// Total 30d trade volume for spot (extended only)
    pub spot_volume_30d: Option<f64>,
}

/// A single volatility index candle
///
/// Represents OHLC data for a volatility index at a specific timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolatilityIndexCandle {
    /// Timestamp in milliseconds since UNIX epoch
    pub timestamp: u64,
    /// Open value
    pub open: f64,
    /// High value
    pub high: f64,
    /// Low value
    pub low: f64,
    /// Close value
    pub close: f64,
}

/// Response from get_volatility_index_data
///
/// Contains volatility index candles and optional continuation token.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolatilityIndexData {
    /// Candles as OHLC data
    #[serde(deserialize_with = "deserialize_candles")]
    pub data: Vec<VolatilityIndexCandle>,
    /// Continuation token for pagination (use as end_timestamp for next request)
    pub continuation: Option<u64>,
}

/// Deserialize candles from array of arrays format
fn deserialize_candles<'de, D>(deserializer: D) -> Result<Vec<VolatilityIndexCandle>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let raw: Vec<Vec<serde_json::Value>> = Vec::deserialize(deserializer)?;
    let mut candles = Vec::with_capacity(raw.len());

    for arr in raw {
        if arr.len() != 5 {
            return Err(D::Error::custom(format!(
                "expected 5 elements in candle array, got {}",
                arr.len()
            )));
        }
        let timestamp = arr[0]
            .as_u64()
            .ok_or_else(|| D::Error::custom("invalid timestamp"))?;
        let open = arr[1]
            .as_f64()
            .ok_or_else(|| D::Error::custom("invalid open"))?;
        let high = arr[2]
            .as_f64()
            .ok_or_else(|| D::Error::custom("invalid high"))?;
        let low = arr[3]
            .as_f64()
            .ok_or_else(|| D::Error::custom("invalid low"))?;
        let close = arr[4]
            .as_f64()
            .ok_or_else(|| D::Error::custom("invalid close"))?;

        candles.push(VolatilityIndexCandle {
            timestamp,
            open,
            high,
            low,
            close,
        });
    }

    Ok(candles)
}

/// Account summary information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AccountResult {
    /// Currency of the summary
    #[serde(default)]
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
