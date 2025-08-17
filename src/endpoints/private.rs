//! Private endpoints for authenticated API calls

use crate::DeribitHttpClient;
use crate::error::HttpError;
use crate::model::http_types::ApiResponse;
use serde::{Deserialize, Serialize};

/// Private endpoints implementation
impl DeribitHttpClient {
    /// Get subaccounts
    ///
    /// Retrieves the list of subaccounts associated with the main account.
    ///
    /// # Arguments
    ///
    /// * `with_portfolio` - Include portfolio information (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let subaccounts = client.get_subaccounts(Some(true)).await?;
    /// // tracing::info!("Found {} subaccounts", subaccounts.len());
    /// ```
    pub async fn get_subaccounts(
        &self,
        with_portfolio: Option<bool>,
    ) -> Result<Vec<Subaccount>, HttpError> {
        let mut query_params = Vec::new();

        if let Some(with_portfolio) = with_portfolio {
            query_params.push(("with_portfolio".to_string(), with_portfolio.to_string()));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            "?".to_string()
                + &query_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&")
        };

        let url = format!(
            "{}/private/get_subaccounts{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get subaccounts failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<Subaccount>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No subaccounts data in response".to_string())
        })
    }

    /// Get transaction log
    ///
    /// Retrieves transaction log entries for the account.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    /// * `start_timestamp` - Start timestamp in milliseconds (optional)
    /// * `end_timestamp` - End timestamp in milliseconds (optional)
    /// * `count` - Number of requested items (optional, default 10)
    /// * `continuation` - Continuation token for pagination (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let log = client.get_transaction_log("BTC", None, None, Some(20), None).await?;
    /// // tracing::info!("Found {} transaction log entries", log.logs.len());
    /// ```
    pub async fn get_transaction_log(
        &self,
        currency: &str,
        start_timestamp: Option<u64>,
        end_timestamp: Option<u64>,
        count: Option<u32>,
        continuation: Option<&str>,
    ) -> Result<TransactionLog, HttpError> {
        let mut query_params = vec![("currency".to_string(), currency.to_string())];

        if let Some(start_timestamp) = start_timestamp {
            query_params.push(("start_timestamp".to_string(), start_timestamp.to_string()));
        }

        if let Some(end_timestamp) = end_timestamp {
            query_params.push(("end_timestamp".to_string(), end_timestamp.to_string()));
        }

        if let Some(count) = count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(continuation) = continuation {
            query_params.push(("continuation".to_string(), continuation.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/private/get_transaction_log?{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get transaction log failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TransactionLog> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No transaction log data in response".to_string())
        })
    }

    /// Get deposits
    ///
    /// Retrieves the latest user deposits.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    /// * `count` - Number of requested items (optional, default 10)
    /// * `offset` - Offset for pagination (optional, default 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let deposits = client.get_deposits("BTC", Some(20), Some(0)).await?;
    /// // tracing::info!("Found {} deposits", deposits.data.len());
    /// ```
    pub async fn get_deposits(
        &self,
        currency: &str,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<DepositsResponse, HttpError> {
        let mut query_params = vec![("currency".to_string(), currency.to_string())];

        if let Some(count) = count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(offset) = offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}/private/get_deposits?{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get deposits failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<DepositsResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No deposits data in response".to_string()))
    }

    /// Get withdrawals
    ///
    /// Retrieves the latest user withdrawals.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    /// * `count` - Number of requested items (optional, default 10)
    /// * `offset` - Offset for pagination (optional, default 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let withdrawals = client.get_withdrawals("BTC", Some(20), Some(0)).await?;
    /// // tracing::info!("Found {} withdrawals", withdrawals.data.len());
    /// ```
    pub async fn get_withdrawals(
        &self,
        currency: &str,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<WithdrawalsResponse, HttpError> {
        let mut query_params = vec![("currency".to_string(), currency.to_string())];

        if let Some(count) = count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(offset) = offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/private/get_withdrawals?{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get withdrawals failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<WithdrawalsResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No withdrawals data in response".to_string())
        })
    }

    /// Submit transfer to subaccount
    ///
    /// Transfers funds to a subaccount.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    /// * `amount` - Amount of funds to be transferred
    /// * `destination` - ID of destination subaccount
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let transfer = client.submit_transfer_to_subaccount("BTC", 0.001, 123).await?;
    /// // tracing::info!("Transfer ID: {}", transfer.id);
    /// ```
    pub async fn submit_transfer_to_subaccount(
        &self,
        currency: &str,
        amount: f64,
        destination: u64,
    ) -> Result<TransferResult, HttpError> {
        let query_params = [
            ("currency".to_string(), currency.to_string()),
            ("amount".to_string(), amount.to_string()),
            ("destination".to_string(), destination.to_string()),
        ];

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/private/submit_transfer_to_subaccount?{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Submit transfer to subaccount failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TransferResult> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No transfer result in response".to_string()))
    }

    /// Submit transfer to user
    ///
    /// Transfers funds to another user.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    /// * `amount` - Amount of funds to be transferred
    /// * `destination` - Destination wallet address from address book
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let transfer = client.submit_transfer_to_user("ETH", 0.1, "0x1234...").await?;
    /// // tracing::info!("Transfer ID: {}", transfer.id);
    /// ```
    pub async fn submit_transfer_to_user(
        &self,
        currency: &str,
        amount: f64,
        destination: &str,
    ) -> Result<TransferResult, HttpError> {
        let query_params = [
            ("currency".to_string(), currency.to_string()),
            ("amount".to_string(), amount.to_string()),
            ("destination".to_string(), destination.to_string()),
        ];

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/private/submit_transfer_to_user?{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Submit transfer to user failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TransferResult> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No transfer result in response".to_string()))
    }
}

/// Funding chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingChartData {
    /// Current interest rate
    pub current_interest: f64,
    /// Interest rate for 8 hours
    pub interest_8h: f64,
    /// List of funding data points
    pub data: Vec<FundingDataPoint>,
}

/// Funding data point structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingDataPoint {
    /// Index price
    pub index_price: f64,
    /// Interest rate for 8 hours
    pub interest_8h: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// TradingView chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TradingViewChartData {
    /// Status of the request
    pub status: String,
    /// Timestamps
    pub ticks: Vec<u64>,
    /// Opening prices
    pub open: Vec<f64>,
    /// High prices
    pub high: Vec<f64>,
    /// Low prices
    pub low: Vec<f64>,
    /// Closing prices
    pub close: Vec<f64>,
    /// Volume data
    pub volume: Vec<f64>,
    /// Cost data
    pub cost: Vec<f64>,
}

/// Transfer result structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// Transfer ID
    pub id: u64,
    /// Transfer type (subaccount, user)
    #[serde(rename = "type")]
    pub transfer_type: String,
    /// Transfer state (confirmed, prepared, etc.)
    pub state: String,
    /// Currency
    pub currency: String,
    /// Transfer amount
    pub amount: f64,
    /// Transfer direction (payment, etc.)
    pub direction: String,
    /// Other side (destination info)
    pub other_side: String,
    /// Creation timestamp
    pub created_timestamp: u64,
    /// Last update timestamp
    pub updated_timestamp: u64,
}

/// Deposits response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct DepositsResponse {
    /// Total count of deposits
    pub count: u32,
    /// List of deposit entries
    pub data: Vec<Deposit>,
}

/// Deposit structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// Deposit address
    pub address: String,
    /// Deposit amount
    pub amount: f64,
    /// Currency
    pub currency: String,
    /// Timestamp when deposit was received
    pub received_timestamp: u64,
    /// Deposit state (completed, pending, etc.)
    pub state: String,
    /// Transaction ID
    pub transaction_id: String,
    /// Timestamp when deposit was last updated
    pub updated_timestamp: u64,
}

/// Withdrawals response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalsResponse {
    /// Total count of withdrawals
    pub count: u32,
    /// List of withdrawal entries
    pub data: Vec<Withdrawal>,
}

/// Withdrawal structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    /// Withdrawal address
    pub address: String,
    /// Withdrawal amount
    pub amount: f64,
    /// Timestamp when withdrawal was confirmed (optional)
    pub confirmed_timestamp: Option<u64>,
    /// Timestamp when withdrawal was created
    pub created_timestamp: u64,
    /// Currency
    pub currency: String,
    /// Withdrawal fee
    pub fee: f64,
    /// Withdrawal ID
    pub id: u64,
    /// Priority level
    pub priority: f64,
    /// Withdrawal state (confirmed, unconfirmed, etc.)
    pub state: String,
    /// Transaction ID (optional)
    pub transaction_id: Option<String>,
    /// Timestamp when withdrawal was last updated
    pub updated_timestamp: u64,
}

/// Subaccount structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Subaccount {
    /// Account ID
    pub id: u64,
    /// Email address
    pub email: String,
    /// Username
    pub username: String,
    /// System name
    pub system_name: String,
    /// Account type (main, subaccount)
    #[serde(rename = "type")]
    pub account_type: String,
    /// Whether login is enabled
    pub login_enabled: bool,
    /// Whether password is set
    pub is_password: bool,
    /// Whether to receive notifications
    pub receive_notifications: bool,
    /// Margin model
    pub margin_model: String,
    /// Whether security keys are enabled
    pub security_keys_enabled: bool,
    /// Security keys assignments
    pub security_keys_assignments: Vec<String>,
    /// Portfolio information (optional)
    pub portfolio: Option<std::collections::HashMap<String, PortfolioInfo>>,
}

/// Portfolio information structure
#[derive(Clone, Serialize, Deserialize)]
pub struct PortfolioInfo {
    /// Currency
    pub currency: String,
    /// Balance
    pub balance: f64,
    /// Available funds
    pub available_funds: f64,
    /// Available withdrawal funds
    pub available_withdrawal_funds: f64,
    /// Equity
    pub equity: f64,
    /// Initial margin
    pub initial_margin: f64,
    /// Maintenance margin
    pub maintenance_margin: f64,
    /// Margin balance
    pub margin_balance: f64,
    /// Additional reserve
    pub additional_reserve: f64,
    /// Spot reserve
    pub spot_reserve: f64,
}

/// Transaction log structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionLog {
    /// List of transaction log entries
    pub logs: Vec<TransactionLogEntry>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
}

/// Transaction log entry structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionLogEntry {
    /// Entry ID
    pub id: u64,
    /// User ID
    pub user_id: u64,
    /// Username
    pub username: String,
    /// User sequence number
    pub user_seq: u64,
    /// Transaction type
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Timestamp
    pub timestamp: u64,
    /// Currency
    pub currency: String,
    /// Balance change
    pub change: f64,
    /// Cash flow
    pub cashflow: f64,
    /// Balance after transaction
    pub balance: f64,
    /// Equity after transaction
    pub equity: f64,
    /// Commission
    pub commission: Option<f64>,
    /// Side (buy/sell/-)
    pub side: String,
    /// Price
    pub price: Option<f64>,
    /// Position
    pub position: Option<f64>,
    /// Amount
    pub amount: Option<f64>,
    /// Instrument name
    pub instrument_name: Option<String>,
    /// Order ID
    pub order_id: Option<String>,
    /// Trade ID
    pub trade_id: Option<String>,
    /// Interest P&L
    pub interest_pl: Option<f64>,
    /// Total interest P&L
    pub total_interest_pl: Option<f64>,
    /// Session unrealized P&L
    pub session_upl: Option<f64>,
    /// Session realized P&L
    pub session_rpl: Option<f64>,
    /// Price currency
    pub price_currency: Option<String>,
    /// User role (maker/taker)
    pub user_role: Option<String>,
    /// Additional information
    pub info: Option<serde_json::Value>,
}

/// User trade structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Instrument name
    pub instrument_name: String,
    /// Last trade price
    pub last_price: f64,
    /// Best bid price
    pub best_bid_price: Option<f64>,
    /// Best ask price
    pub best_ask_price: Option<f64>,
    /// Best bid amount
    pub best_bid_amount: Option<f64>,
    /// Best ask amount
    pub best_ask_amount: Option<f64>,
    /// Current funding rate
    pub current_funding: Option<f64>,
    /// Estimated delivery price
    pub estimated_delivery_price: Option<f64>,
    /// 8h funding rate
    pub funding_8h: Option<f64>,
    /// Index price
    pub index_price: Option<f64>,
    /// Interest value
    pub interest_value: Option<f64>,
    /// Mark price
    pub mark_price: Option<f64>,
    /// Maximum price
    pub max_price: Option<f64>,
    /// Minimum price
    pub min_price: Option<f64>,
    /// Open interest
    pub open_interest: Option<f64>,
    /// Settlement price
    pub settlement_price: Option<f64>,
    /// Instrument state
    pub state: Option<String>,
    /// Statistics
    pub stats: Option<TickerStats>,
    /// Timestamp
    pub timestamp: u64,
}

/// Ticker statistics structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerStats {
    /// 24h high price
    pub high: Option<f64>,
    /// 24h low price
    pub low: Option<f64>,
    /// Price change percentage
    pub price_change: Option<f64>,
    /// 24h volume
    pub volume: Option<f64>,
    /// 24h volume in USD
    pub volume_usd: Option<f64>,
}

/// Order book structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Instrument name
    pub instrument_name: String,
    /// Bids (price, amount)
    pub bids: Vec<[f64; 2]>,
    /// Asks (price, amount)
    pub asks: Vec<[f64; 2]>,
    /// Timestamp
    pub timestamp: u64,
    /// Change ID
    pub change_id: Option<u64>,
}

/// Instrument information
#[derive(Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument name
    pub instrument_name: String,
    /// Instrument kind
    pub kind: String,
    /// Base currency
    pub base_currency: String,
    /// Quote currency
    pub quote_currency: String,
    /// Settlement currency
    pub settlement_currency: String,
    /// Contract size
    pub contract_size: f64,
    /// Minimum trade amount
    pub min_trade_amount: f64,
    /// Tick size
    pub tick_size: f64,
    /// Is active
    pub is_active: bool,
    /// Expiration timestamp
    pub expiration_timestamp: Option<u64>,
    /// Creation timestamp
    pub creation_timestamp: u64,
}

/// Trade information
#[derive(Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub trade_id: String,
    /// Instrument name
    pub instrument_name: String,
    /// Trade price
    pub price: f64,
    /// Trade amount
    pub amount: f64,
    /// Trade direction
    pub direction: String,
    /// Timestamp
    pub timestamp: u64,
    /// Index price
    pub index_price: Option<f64>,
}

/// Trading endpoints
impl DeribitHttpClient {
    /// Place a buy order
    ///
    /// Places a buy order for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `request` - The buy order request parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::{DeribitHttpClient, BuyOrderRequest, OrderType};
    ///
    /// let client = DeribitHttpClient::new(true);
    /// let request = BuyOrderRequest {
    ///     instrument_name: "BTC-PERPETUAL".to_string(),
    ///     amount: Some(100.0),
    ///     contracts: None,
    ///     order_type: OrderType::Market,
    ///     price: None,
    ///     label: Some("test_order".to_string()),
    ///     ..Default::default()
    /// };
    /// // let result = client.buy_order(request).await?;
    /// // println!("Order placed: {}", result.order.order_id);
    /// ```
    pub async fn buy_order(&self, request: BuyOrderRequest) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![("instrument_name".to_string(), request.instrument_name)];

        // Add amount or contracts (one is required)
        if let Some(amount) = request.amount {
            query_params.push(("amount".to_string(), amount.to_string()));
        } else if let Some(contracts) = request.contracts {
            query_params.push(("contracts".to_string(), contracts.to_string()));
        } else {
            return Err(HttpError::InvalidResponse(
                "Either amount or contracts must be specified".to_string(),
            ));
        }

        query_params.push(("type".to_string(), request.order_type.as_str().to_string()));

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(label) = request.label {
            query_params.push(("label".to_string(), label));
        }

        query_params.push((
            "time_in_force".to_string(),
            request.time_in_force.as_str().to_string(),
        ));

        if request.post_only {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if request.reduce_only {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}/private/buy?{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Buy order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order data in response".to_string()))
    }

    /// Place a sell order
    ///
    /// Places a sell order for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `request` - The sell order request parameters
    pub async fn sell_order(&self, request: SellOrderRequest) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![("instrument_name".to_string(), request.instrument_name)];

        // Add amount or contracts (one is required)
        if let Some(amount) = request.amount {
            query_params.push(("amount".to_string(), amount.to_string()));
        } else if let Some(contracts) = request.contracts {
            query_params.push(("contracts".to_string(), contracts.to_string()));
        } else {
            return Err(HttpError::InvalidResponse(
                "Either amount or contracts must be specified".to_string(),
            ));
        }

        query_params.push(("type".to_string(), request.order_type.as_str().to_string()));

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(label) = request.label {
            query_params.push(("label".to_string(), label));
        }

        query_params.push((
            "time_in_force".to_string(),
            request.time_in_force.as_str().to_string(),
        ));

        if request.post_only {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if request.reduce_only {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}/private/sell?{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Sell order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order data in response".to_string()))
    }

    /// Cancel an order
    ///
    /// Cancels an existing order by order ID.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order to cancel
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let result = client.cancel_order("BTC-12345").await?;
    /// // println!("Order cancelled: {}", result.order_id);
    /// ```
    pub async fn cancel_order(&self, order_id: &str) -> Result<OrderInfo, HttpError> {
        let url = format!(
            "{}/private/cancel?order_id={}",
            self.base_url(),
            urlencoding::encode(order_id)
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderInfo> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order data in response".to_string()))
    }

    /// Get account summary
    ///
    /// Retrieves account summary information including balance, margin, and other account details.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency to get summary for (BTC, ETH, USDC, etc.)
    /// * `extended` - Whether to include extended information
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let summary = client.get_account_summary("BTC", Some(true)).await?;
    /// // println!("Balance: {}", summary.balance);
    /// ```
    pub async fn get_account_summary(
        &self,
        currency: &str,
        extended: Option<bool>,
    ) -> Result<AccountSummary, HttpError> {
        let mut query_params = vec![("currency".to_string(), currency.to_string())];

        if let Some(extended) = extended {
            query_params.push(("extended".to_string(), extended.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/private/get_account_summary?{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get account summary failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<AccountSummary> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No account summary data in response".to_string())
        })
    }

    /// Get positions
    ///
    /// Retrieves user positions for the specified currency and kind.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency filter (BTC, ETH, USDC, etc.) - optional
    /// * `kind` - Kind filter (future, option, spot, etc.) - optional
    /// * `subaccount_id` - Subaccount ID - optional
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let positions = client.get_positions(Some("BTC"), Some("future"), None).await?;
    /// // println!("Found {} positions", positions.len());
    /// ```
    pub async fn get_positions(
        &self,
        currency: Option<&str>,
        kind: Option<&str>,
        subaccount_id: Option<i32>,
    ) -> Result<Vec<Position>, HttpError> {
        let mut query_params = Vec::new();

        if let Some(currency) = currency {
            query_params.push(("currency".to_string(), currency.to_string()));
        }

        if let Some(kind) = kind {
            query_params.push(("kind".to_string(), kind.to_string()));
        }

        if let Some(subaccount_id) = subaccount_id {
            query_params.push(("subaccount_id".to_string(), subaccount_id.to_string()));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            "?".to_string()
                + &query_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&")
        };

        let url = format!("{}/private/get_positions{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get positions failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<Position>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No positions data in response".to_string()))
    }

    /// Edit an existing order
    ///
    /// Modifies price, amount and/or other properties of an existing order.
    ///
    /// # Arguments
    ///
    /// * `request` - The edit order request parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::{DeribitHttpClient, EditOrderRequest};
    ///
    /// let client = DeribitHttpClient::new(true);
    /// let request = EditOrderRequest {
    ///     order_id: "BTC-12345".to_string(),
    ///     amount: Some(200.0),
    ///     price: Some(45000.0),
    ///     ..Default::default()
    /// };
    /// // let result = client.edit_order(request).await?;
    /// // tracing::info!("Order edited: {}", result.order.order_id);
    /// ```
    pub async fn edit_order(&self, request: EditOrderRequest) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![("order_id".to_string(), request.order_id)];

        if let Some(amount) = request.amount {
            query_params.push(("amount".to_string(), amount.to_string()));
        }

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(ref advanced) = request.advanced {
            query_params.push(("advanced".to_string(), advanced.clone()));
        }

        if request.post_only {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if request.reduce_only {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}/private/edit?{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Edit order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order data in response".to_string()))
    }

    /// Cancel all orders
    ///
    /// Cancels all user orders and trigger orders within all currencies and instrument kinds.
    ///
    /// # Arguments
    ///
    /// * `detailed` - Whether to return detailed response
    /// * `freeze_quotes` - Whether to reject incoming quotes for 1 second after cancelling
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let cancelled_count = client.cancel_all_orders(Some(false), Some(false)).await?;
    /// // tracing::info!("Cancelled {} orders", cancelled_count);
    /// ```
    pub async fn cancel_all_orders(
        &self,
        detailed: Option<bool>,
        freeze_quotes: Option<bool>,
    ) -> Result<u32, HttpError> {
        let mut query_params = Vec::new();

        if let Some(detailed) = detailed {
            query_params.push(("detailed".to_string(), detailed.to_string()));
        }

        if let Some(freeze_quotes) = freeze_quotes {
            query_params.push(("freeze_quotes".to_string(), freeze_quotes.to_string()));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            "?".to_string()
                + &query_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&")
        };

        let url = format!("{}/private/cancel_all{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel all orders failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<u32> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No cancel count in response".to_string()))
    }

    /// Get open orders
    ///
    /// Retrieves list of user's open orders across many currencies.
    ///
    /// # Arguments
    ///
    /// * `kind` - Instrument kind filter (optional)
    /// * `order_type` - Order type filter (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let orders = client.get_open_orders(Some("future"), None).await?;
    /// // tracing::info!("Found {} open orders", orders.len());
    /// ```
    pub async fn get_open_orders(
        &self,
        kind: Option<&str>,
        order_type: Option<&str>,
    ) -> Result<Vec<OrderInfo>, HttpError> {
        let mut query_params = Vec::new();

        if let Some(kind) = kind {
            query_params.push(("kind".to_string(), kind.to_string()));
        }

        if let Some(order_type) = order_type {
            query_params.push(("type".to_string(), order_type.to_string()));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            "?".to_string()
                + &query_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&")
        };

        let url = format!(
            "{}/private/get_open_orders{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get open orders failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderInfo>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No orders data in response".to_string()))
    }

    /// Get order history
    ///
    /// Retrieves history of orders that have been partially or fully filled.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    /// * `kind` - Instrument kind filter (optional)
    /// * `count` - Number of requested items (optional, default 20)
    /// * `offset` - Offset for pagination (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let history = client.get_order_history("BTC", Some("future"), Some(10), None).await?;
    /// // tracing::info!("Found {} historical orders", history.len());
    /// ```
    pub async fn get_order_history(
        &self,
        currency: &str,
        kind: Option<&str>,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<OrderInfo>, HttpError> {
        let mut query_params = vec![("currency".to_string(), currency.to_string())];

        if let Some(kind) = kind {
            query_params.push(("kind".to_string(), kind.to_string()));
        }

        if let Some(count) = count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(offset) = offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/private/get_order_history_by_currency?{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get order history failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderInfo>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No order history data in response".to_string())
        })
    }

    /// Get user trades by instrument
    ///
    /// Retrieves the latest user trades that have occurred for a specific instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `start_seq` - Start sequence number (optional)
    /// * `end_seq` - End sequence number (optional)
    /// * `count` - Number of requested items (optional, default 10)
    /// * `include_old` - Include trades older than 7 days (optional)
    /// * `sorting` - Direction of results sorting (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let trades = client.get_user_trades_by_instrument("BTC-PERPETUAL", None, None, Some(20), None, None).await?;
    /// // tracing::info!("Found {} user trades", trades.len());
    /// ```
    pub async fn get_user_trades_by_instrument(
        &self,
        instrument_name: &str,
        start_seq: Option<u64>,
        end_seq: Option<u64>,
        count: Option<u32>,
        include_old: Option<bool>,
        sorting: Option<&str>,
    ) -> Result<Vec<UserTrade>, HttpError> {
        let mut query_params = vec![("instrument_name".to_string(), instrument_name.to_string())];

        if let Some(start_seq) = start_seq {
            query_params.push(("start_seq".to_string(), start_seq.to_string()));
        }

        if let Some(end_seq) = end_seq {
            query_params.push(("end_seq".to_string(), end_seq.to_string()));
        }

        if let Some(count) = count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(include_old) = include_old {
            query_params.push(("include_old".to_string(), include_old.to_string()));
        }

        if let Some(sorting) = sorting {
            query_params.push(("sorting".to_string(), sorting.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}/private/get_user_trades_by_instrument?{}",
            self.base_url(),
            query_string
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get user trades failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<UserTrade>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No user trades data in response".to_string())
        })
    }
}

/// User trade structure
#[derive(Clone, Serialize, Deserialize)]
pub struct UserTrade {
    /// Trade ID
    pub trade_id: String,
    /// Trade sequence number
    pub trade_seq: u64,
    /// Order ID
    pub order_id: String,
    /// Order type
    pub order_type: String,
    /// Instrument name
    pub instrument_name: String,
    /// Trade direction
    pub direction: String,
    /// Trade amount
    pub amount: f64,
    /// Trade price
    pub price: f64,
    /// Timestamp
    pub timestamp: u64,
    /// Fee
    pub fee: f64,
    /// Fee currency
    pub fee_currency: String,
    /// Liquidity (M=maker, T=taker)
    pub liquidity: String,
    /// Index price at trade time
    pub index_price: f64,
    /// Mark price at trade time
    pub mark_price: f64,
    /// Tick direction
    pub tick_direction: i32,
    /// Matching ID
    pub matching_id: Option<String>,
    /// Label
    pub label: String,
    /// Self trade flag
    pub self_trade: bool,
}

/// Edit order request structure
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct EditOrderRequest {
    /// Order ID to edit
    pub order_id: String,
    /// New amount (optional)
    pub amount: Option<f64>,
    /// New price (optional)
    pub price: Option<f64>,
    /// Advanced parameter (optional)
    pub advanced: Option<String>,
    /// Post-only flag
    pub post_only: bool,
    /// Reduce-only flag
    pub reduce_only: bool,
}

/// Order type enumeration
#[derive(Clone, Serialize, Deserialize)]
pub enum OrderType {
    /// Limit order - order with specified price
    #[serde(rename = "limit")]
    Limit,
    /// Market order - order executed at current market price
    #[serde(rename = "market")]
    Market,
    /// Stop limit order - limit order triggered when price reaches stop price
    #[serde(rename = "stop_limit")]
    StopLimit,
    /// Stop market order - market order triggered when price reaches stop price
    #[serde(rename = "stop_market")]
    StopMarket,
    /// Take limit order - limit order for profit taking
    #[serde(rename = "take_limit")]
    TakeLimit,
    /// Take market order - market order for profit taking
    #[serde(rename = "take_market")]
    TakeMarket,
    /// Market limit order - market order with price protection
    #[serde(rename = "market_limit")]
    MarketLimit,
    /// Trailing stop order - stop order that follows price movement
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl OrderType {
    /// Convert order type to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::StopLimit => "stop_limit",
            OrderType::StopMarket => "stop_market",
            OrderType::TakeLimit => "take_limit",
            OrderType::TakeMarket => "take_market",
            OrderType::MarketLimit => "market_limit",
            OrderType::TrailingStop => "trailing_stop",
        }
    }
}

/// Time in force enumeration
#[derive(Clone, Serialize, Deserialize, Default)]
pub enum TimeInForce {
    /// Good till cancelled - order remains active until explicitly cancelled
    #[serde(rename = "good_til_cancelled")]
    #[default]
    GoodTilCancelled,
    /// Good till day - order expires at end of trading day
    #[serde(rename = "good_til_day")]
    GoodTilDay,
    /// Fill or kill - order must be filled immediately and completely or cancelled
    #[serde(rename = "fill_or_kill")]
    FillOrKill,
    /// Immediate or cancel - order must be filled immediately, partial fills allowed
    #[serde(rename = "immediate_or_cancel")]
    ImmediateOrCancel,
}

impl TimeInForce {
    /// Convert time in force to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::GoodTilCancelled => "good_til_cancelled",
            TimeInForce::GoodTilDay => "good_til_day",
            TimeInForce::FillOrKill => "fill_or_kill",
            TimeInForce::ImmediateOrCancel => "immediate_or_cancel",
        }
    }
}

/// Buy order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct BuyOrderRequest {
    /// Instrument name
    pub instrument_name: String,
    /// Order amount (alternative to contracts)
    pub amount: Option<f64>,
    /// Order size in contracts (alternative to amount)
    pub contracts: Option<f64>,
    /// Order type
    pub order_type: OrderType,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// User-defined label
    pub label: Option<String>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Post-only flag
    pub post_only: bool,
    /// Reduce-only flag
    pub reduce_only: bool,
}

impl Default for BuyOrderRequest {
    fn default() -> Self {
        Self {
            instrument_name: String::new(),
            amount: None,
            contracts: None,
            order_type: OrderType::Limit,
            price: None,
            label: None,
            time_in_force: TimeInForce::default(),
            post_only: false,
            reduce_only: false,
        }
    }
}

/// Sell order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct SellOrderRequest {
    /// Instrument name
    pub instrument_name: String,
    /// Order amount (alternative to contracts)
    pub amount: Option<f64>,
    /// Order size in contracts (alternative to amount)
    pub contracts: Option<f64>,
    /// Order type
    pub order_type: OrderType,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// User-defined label
    pub label: Option<String>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Post-only flag
    pub post_only: bool,
    /// Reduce-only flag
    pub reduce_only: bool,
}

impl Default for SellOrderRequest {
    fn default() -> Self {
        Self {
            instrument_name: String::new(),
            amount: None,
            contracts: None,
            order_type: OrderType::Limit,
            price: None,
            label: None,
            time_in_force: TimeInForce::default(),
            post_only: false,
            reduce_only: false,
        }
    }
}

/// Order response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    /// List of trades executed
    pub trades: Vec<TradeExecution>,
    /// Order information
    pub order: OrderInfo,
}

/// Trade execution information
#[derive(Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    /// Trade sequence number
    pub trade_seq: u64,
    /// Trade ID
    pub trade_id: String,
    /// Timestamp
    pub timestamp: u64,
    /// Tick direction
    pub tick_direction: i32,
    /// Trade state
    pub state: String,
    /// Reduce only flag
    pub reduce_only: bool,
    /// Trade price
    pub price: f64,
    /// Post only flag
    pub post_only: bool,
    /// Order type
    pub order_type: String,
    /// Order ID
    pub order_id: String,
    /// Matching ID
    pub matching_id: Option<String>,
    /// Mark price
    pub mark_price: f64,
    /// Liquidity (T=taker, M=maker)
    pub liquidity: String,
    /// Order label
    pub label: String,
    /// Instrument name
    pub instrument_name: String,
    /// Index price
    pub index_price: f64,
    /// Fee currency
    pub fee_currency: String,
    /// Fee amount
    pub fee: f64,
    /// Trade direction
    pub direction: String,
    /// Trade amount
    pub amount: f64,
}

/// Order information structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    /// Web order flag
    pub web: bool,
    /// Time in force
    pub time_in_force: String,
    /// Replaced flag
    pub replaced: bool,
    /// Reduce only flag
    pub reduce_only: bool,
    /// Order price
    pub price: f64,
    /// Post only flag
    pub post_only: bool,
    /// Order type
    pub order_type: String,
    /// Order state
    pub order_state: String,
    /// Order ID
    pub order_id: String,
    /// Maximum show amount
    pub max_show: f64,
    /// Last update timestamp
    pub last_update_timestamp: u64,
    /// Order label
    pub label: String,
    /// Rebalance flag
    pub is_rebalance: bool,
    /// Liquidation flag
    pub is_liquidation: bool,
    /// Instrument name
    pub instrument_name: String,
    /// Filled amount
    pub filled_amount: f64,
    /// Order direction
    pub direction: String,
    /// Creation timestamp
    pub creation_timestamp: u64,
    /// Average price
    pub average_price: f64,
    /// API flag
    pub api: bool,
    /// Order amount
    pub amount: f64,
}

// Implement Display and Debug traits using macros from deribit-base
deribit_base::impl_json_display!(TickerData);
deribit_base::impl_json_debug_pretty!(TickerData);

deribit_base::impl_json_display!(TickerStats);
deribit_base::impl_json_debug_pretty!(TickerStats);

deribit_base::impl_json_display!(OrderBook);
deribit_base::impl_json_debug_pretty!(OrderBook);

deribit_base::impl_json_display!(Instrument);
deribit_base::impl_json_debug_pretty!(Instrument);

deribit_base::impl_json_display!(Trade);
deribit_base::impl_json_debug_pretty!(Trade);

deribit_base::impl_json_display!(OrderType);
deribit_base::impl_json_debug_pretty!(OrderType);

deribit_base::impl_json_display!(TimeInForce);
deribit_base::impl_json_debug_pretty!(TimeInForce);

deribit_base::impl_json_display!(BuyOrderRequest);
deribit_base::impl_json_debug_pretty!(BuyOrderRequest);

deribit_base::impl_json_display!(SellOrderRequest);
deribit_base::impl_json_debug_pretty!(SellOrderRequest);

deribit_base::impl_json_display!(OrderResponse);
deribit_base::impl_json_debug_pretty!(OrderResponse);

deribit_base::impl_json_display!(TradeExecution);
deribit_base::impl_json_debug_pretty!(TradeExecution);

deribit_base::impl_json_display!(OrderInfo);
deribit_base::impl_json_debug_pretty!(OrderInfo);

/// Account summary structure
#[derive(Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    /// Account balance
    pub balance: f64,
    /// Available funds
    pub available_funds: f64,
    /// Available withdrawal funds
    pub available_withdrawal_funds: Option<f64>,
    /// Margin balance
    pub margin_balance: Option<f64>,
    /// Initial margin
    pub initial_margin: Option<f64>,
    /// Maintenance margin
    pub maintenance_margin: Option<f64>,
    /// Projected initial margin
    pub projected_initial_margin: Option<f64>,
    /// Projected maintenance margin
    pub projected_maintenance_margin: Option<f64>,
    /// Total profit and loss
    pub total_pl: f64,
    /// Session unrealized profit and loss
    pub session_upl: Option<f64>,
    /// Futures session unrealized profit and loss
    pub futures_session_upl: Option<f64>,
    /// Options session unrealized profit and loss
    pub options_session_upl: Option<f64>,
    /// Futures session realized profit and loss
    pub futures_session_rpl: Option<f64>,
    /// Options session realized profit and loss
    pub options_session_rpl: Option<f64>,
    /// Futures profit and loss
    pub futures_pl: Option<f64>,
    /// Options profit and loss
    pub options_pl: Option<f64>,
    /// Currency
    pub currency: String,
    /// Account type
    #[serde(rename = "type")]
    pub account_type: String,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// System name
    pub system_name: String,
    /// Account ID
    pub id: u64,
    /// Equity
    pub equity: f64,
    /// Fee balance
    pub fee_balance: f64,
    /// Estimated liquidation ratio
    pub estimated_liquidation_ratio: Option<f64>,
    /// Options delta
    pub options_delta: Option<f64>,
    /// Options gamma
    pub options_gamma: Option<f64>,
    /// Options theta
    pub options_theta: Option<f64>,
    /// Options vega
    pub options_vega: Option<f64>,
    /// Options value
    pub options_value: Option<f64>,
    /// Projected delta total
    pub projected_delta_total: Option<f64>,
    /// Creation timestamp
    pub creation_timestamp: u64,
}

/// Currency information structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Annual percentage rate for yield-generating tokens (USDE, STETH)
    pub apr: Option<f64>,
    /// The type of the currency
    pub coin_type: String,
    /// The abbreviation of the currency used throughout the API
    pub currency: String,
    /// The full name for the currency
    pub currency_long: String,
    /// Fee precision
    pub fee_precision: i32,
    /// Whether the currency is part of the cross collateral pool
    pub in_cross_collateral_pool: Option<bool>,
    /// Minimum number of blockchain confirmations before deposit is accepted
    pub min_confirmations: i32,
    /// The minimum transaction fee paid for withdrawals
    pub min_withdrawal_fee: f64,
    /// The total transaction fee paid for withdrawals
    pub withdrawal_fee: f64,
    /// Withdrawal priority options
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
}

/// Index data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct IndexData {
    /// Bitcoin index price (only for BTC currency)
    #[serde(rename = "BTC")]
    pub btc: Option<f64>,
    /// Ethereum index price (only for ETH currency)
    #[serde(rename = "ETH")]
    pub eth: Option<f64>,
    /// USDC index price (only for USDC currency)
    #[serde(rename = "USDC")]
    pub usdc: Option<f64>,
    /// USDT index price (only for USDT currency)
    #[serde(rename = "USDT")]
    pub usdt: Option<f64>,
    /// EURR index price (only for EURR currency)
    #[serde(rename = "EURR")]
    pub eurr: Option<f64>,
    /// Estimated delivery price for the currency
    pub edp: f64,
}

/// Withdrawal priority structure
#[derive(Clone, Serialize, Deserialize)]
pub struct WithdrawalPriority {
    /// Priority name (e.g., "very_low", "very_high")
    pub name: String,
    /// Priority value
    pub value: f64,
}

/// Position structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    /// Average price
    pub average_price: f64,
    /// Delta
    pub delta: f64,
    /// Direction (buy/sell)
    pub direction: String,
    /// Estimated liquidation price
    pub estimated_liquidation_price: Option<f64>,
    /// Floating profit and loss
    pub floating_profit_loss: f64,
    /// Index price
    pub index_price: f64,
    /// Initial margin
    pub initial_margin: f64,
    /// Instrument name
    pub instrument_name: String,
    /// Interest value
    pub interest_value: Option<f64>,
    /// Kind (future, option, spot)
    pub kind: String,
    /// Leverage
    pub leverage: Option<f64>,
    /// Maintenance margin
    pub maintenance_margin: f64,
    /// Mark price
    pub mark_price: f64,
    /// Open orders margin
    pub open_orders_margin: f64,
    /// Realized funding
    pub realized_funding: f64,
    /// Realized profit and loss
    pub realized_profit_loss: f64,
    /// Settlement price
    pub settlement_price: f64,
    /// Position size
    pub size: f64,
    /// Size in currency
    pub size_currency: f64,
    /// Total profit and loss
    pub total_profit_loss: f64,
}

deribit_base::impl_json_display!(AccountSummary);
deribit_base::impl_json_debug_pretty!(AccountSummary);

deribit_base::impl_json_display!(Position);
deribit_base::impl_json_debug_pretty!(Position);

deribit_base::impl_json_display!(EditOrderRequest);
deribit_base::impl_json_debug_pretty!(EditOrderRequest);

deribit_base::impl_json_display!(UserTrade);
deribit_base::impl_json_debug_pretty!(UserTrade);

deribit_base::impl_json_display!(Subaccount);
deribit_base::impl_json_debug_pretty!(Subaccount);

deribit_base::impl_json_display!(PortfolioInfo);
deribit_base::impl_json_debug_pretty!(PortfolioInfo);

deribit_base::impl_json_display!(TransactionLog);
deribit_base::impl_json_debug_pretty!(TransactionLog);

deribit_base::impl_json_display!(TransactionLogEntry);
deribit_base::impl_json_debug_pretty!(TransactionLogEntry);

deribit_base::impl_json_display!(DepositsResponse);
deribit_base::impl_json_debug_pretty!(DepositsResponse);

deribit_base::impl_json_display!(Deposit);
deribit_base::impl_json_debug_pretty!(Deposit);

deribit_base::impl_json_display!(WithdrawalsResponse);
deribit_base::impl_json_debug_pretty!(WithdrawalsResponse);

deribit_base::impl_json_display!(Withdrawal);
deribit_base::impl_json_debug_pretty!(Withdrawal);

deribit_base::impl_json_display!(TransferResult);
deribit_base::impl_json_debug_pretty!(TransferResult);

deribit_base::impl_json_display!(FundingChartData);
deribit_base::impl_json_debug_pretty!(FundingChartData);

deribit_base::impl_json_display!(FundingDataPoint);
deribit_base::impl_json_debug_pretty!(FundingDataPoint);

deribit_base::impl_json_display!(TradingViewChartData);
deribit_base::impl_json_debug_pretty!(TradingViewChartData);

deribit_base::impl_json_display!(Currency);
deribit_base::impl_json_debug_pretty!(Currency);

deribit_base::impl_json_display!(WithdrawalPriority);
deribit_base::impl_json_debug_pretty!(WithdrawalPriority);

deribit_base::impl_json_display!(IndexData);
deribit_base::impl_json_debug_pretty!(IndexData);
