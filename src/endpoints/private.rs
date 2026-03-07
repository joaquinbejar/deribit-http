//! Private endpoints for authenticated API calls

use crate::DeribitHttpClient;
use crate::constants::endpoints::*;
use crate::error::HttpError;
use crate::model::account::Subaccount;
use crate::model::position::Position;
use crate::model::request::mass_quote::MassQuoteRequest;
use crate::model::request::order::OrderRequest;
use crate::model::request::trade::TradesRequest;
use crate::model::response::api_response::ApiResponse;
use crate::model::response::deposit::DepositsResponse;
use crate::model::response::margin::{MarginsResponse, OrderMargin};
use crate::model::response::mass_quote::MassQuoteResponse;
use crate::model::response::mmp::{MmpConfig, MmpStatus, SetMmpConfigRequest};
use crate::model::response::order::{OrderInfoResponse, OrderResponse};
use crate::model::response::other::{
    AccountSummaryResponse, SettlementsResponse, TransactionLogResponse, TransferResultResponse,
};
use crate::model::response::withdrawal::WithdrawalsResponse;
use crate::model::{
    TransactionLogRequest, UserTradeResponseByOrder, UserTradeWithPaginationResponse,
};
use crate::prelude::Trigger;

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
    /// let client = DeribitHttpClient::new();
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

        let url = format!("{}{}{}", self.base_url(), GET_SUBACCOUNTS, query_string);

        let response = self.make_authenticated_request(&url).await?;

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

        // Debug: Get raw response text first
        let response_text = response.text().await.map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to read response text: {}", e))
        })?;

        tracing::debug!("Raw API response: {}", response_text);

        let api_response: ApiResponse<Vec<Subaccount>> = serde_json::from_str(&response_text)
            .map_err(|e| {
                HttpError::InvalidResponse(format!(
                    "Failed to parse JSON: {} - Raw response: {}",
                    e, response_text
                ))
            })?;

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
    /// * `request` - A `TransactionLogRequest` struct containing:
    ///   * `currency` - Currency symbol (BTC, ETH, etc.)
    ///   * `start_timestamp` - Start timestamp in milliseconds (optional)
    ///   * `end_timestamp` - End timestamp in milliseconds (optional)
    ///   * `count` - Number of requested items (optional, default 10)
    ///   * `continuation` - Continuation token for pagination (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    /// use crate::model::TransactionLogRequest;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let request = TransactionLogRequest { currency: "BTC".into(), ..Default::default() };
    /// // let log = client.get_transaction_log(request).await?;
    /// ```
    pub async fn get_transaction_log(
        &self,
        request: TransactionLogRequest,
    ) -> Result<TransactionLogResponse, HttpError> {
        let mut query_params = vec![("currency".to_string(), request.currency.to_string())];

        query_params.push((
            "start_timestamp".to_string(),
            request.start_timestamp.to_string(),
        ));
        query_params.push((
            "end_timestamp".to_string(),
            request.end_timestamp.to_string(),
        ));

        if let Some(query) = request.query {
            query_params.push(("query".to_string(), query));
        }

        if let Some(count) = request.count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(subaccount_id) = request.subaccount_id {
            query_params.push(("subaccount_id".to_string(), subaccount_id.to_string()));
        }

        if let Some(continuation) = request.continuation {
            query_params.push(("continuation".to_string(), continuation.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_TRANSACTION_LOG,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

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

        let api_response: ApiResponse<TransactionLogResponse> = response
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
    /// let client = DeribitHttpClient::new();
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

        let url = format!("{}{}?{}", self.base_url(), GET_DEPOSITS, query_string);

        let response = self.make_authenticated_request(&url).await?;

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
    /// let client = DeribitHttpClient::new();
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

        let url = format!("{}{}?{}", self.base_url(), GET_WITHDRAWALS, query_string);

        let response = self.make_authenticated_request(&url).await?;

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
    /// let client = DeribitHttpClient::new();
    /// // let transfer = client.submit_transfer_to_subaccount("BTC", 0.001, 123).await?;
    /// // tracing::info!("Transfer ID: {}", transfer.id);
    /// ```
    pub async fn submit_transfer_to_subaccount(
        &self,
        currency: &str,
        amount: f64,
        destination: u64,
    ) -> Result<TransferResultResponse, HttpError> {
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
            "{}{}?{}",
            self.base_url(),
            SUBMIT_TRANSFER_TO_SUBACCOUNT,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

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

        let api_response: ApiResponse<TransferResultResponse> = response
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
    /// let client = DeribitHttpClient::new();
    /// // let transfer = client.submit_transfer_to_user("ETH", 0.1, "0x1234...").await?;
    /// // tracing::info!("Transfer ID: {}", transfer.id);
    /// ```
    pub async fn submit_transfer_to_user(
        &self,
        currency: &str,
        amount: f64,
        destination: &str,
    ) -> Result<TransferResultResponse, HttpError> {
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
            "{}{}?{}",
            self.base_url(),
            SUBMIT_TRANSFER_TO_USER,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

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

        let api_response: ApiResponse<TransferResultResponse> = response
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

    /// Place a buy order
    ///
    /// Places a buy order for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `request` - The buy order request parameters
    ///
    pub async fn buy_order(&self, request: OrderRequest) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![
            ("instrument_name".to_string(), request.instrument_name),
            (
                "amount".to_string(),
                request
                    .amount
                    .map_or_else(|| "0".to_string(), |a| a.to_string()),
            ),
        ];

        if let Some(order_type) = request.type_ {
            query_params.push(("type".to_string(), order_type.as_str().to_string()));
        }

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(label) = request.label {
            query_params.push(("label".to_string(), label));
        }

        if let Some(time_in_force) = request.time_in_force {
            query_params.push((
                "time_in_force".to_string(),
                time_in_force.as_str().to_string(),
            ));
        }

        if let Some(post_only) = request.post_only
            && post_only
        {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if let Some(reduce_only) = request.reduce_only
            && reduce_only
        {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        if let Some(trigger_price) = request.trigger_price {
            query_params.push(("trigger_price".to_string(), trigger_price.to_string()));
        }

        if let Some(trigger) = request.trigger {
            let trigger_str = match trigger {
                Trigger::IndexPrice => "index_price",
                Trigger::MarkPrice => "mark_price",
                Trigger::LastPrice => "last_price",
            };
            query_params.push(("trigger".to_string(), trigger_str.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), BUY, query_string);

        let response = self.make_authenticated_request(&url).await?;

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

        // Debug: capture raw response text first
        let response_text = response
            .text()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        tracing::debug!("Raw API response: {}", response_text);

        let api_response: ApiResponse<OrderResponse> = serde_json::from_str(&response_text)
            .map_err(|e| {
                HttpError::InvalidResponse(format!(
                    "Failed to parse JSON: {} - Raw response: {}",
                    e, response_text
                ))
            })?;

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
    pub async fn sell_order(&self, request: OrderRequest) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![
            ("instrument_name".to_string(), request.instrument_name),
            ("amount".to_string(), request.amount.unwrap().to_string()),
        ];

        if let Some(order_type) = request.type_ {
            query_params.push(("type".to_string(), order_type.as_str().to_string()));
        }

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(label) = request.label {
            query_params.push(("label".to_string(), label));
        }

        if let Some(time_in_force) = request.time_in_force {
            query_params.push((
                "time_in_force".to_string(),
                time_in_force.as_str().to_string(),
            ));
        }

        if let Some(post_only) = request.post_only
            && post_only
        {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if let Some(reduce_only) = request.reduce_only
            && reduce_only
        {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        if let Some(trigger_price) = request.trigger_price {
            query_params.push(("trigger_price".to_string(), trigger_price.to_string()));
        }

        if let Some(trigger) = request.trigger {
            let trigger_str = match trigger {
                Trigger::IndexPrice => "index_price",
                Trigger::MarkPrice => "mark_price",
                Trigger::LastPrice => "last_price",
            };
            query_params.push(("trigger".to_string(), trigger_str.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), SELL, query_string);

        let response = self.make_authenticated_request(&url).await?;

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
    /// Cancels an order by its ID.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order ID to cancel
    ///
    pub async fn cancel_order(&self, order_id: &str) -> Result<OrderInfoResponse, HttpError> {
        let url = format!(
            "{}{}?order_id={}",
            self.base_url(),
            CANCEL,
            urlencoding::encode(order_id)
        );

        let response = self.make_authenticated_request(&url).await?;

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

        let api_response: ApiResponse<OrderInfoResponse> = response
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
    /// Cancels all orders for the account.
    ///
    /// # Returns
    ///
    /// Returns the number of cancelled orders.
    pub async fn cancel_all(&self) -> Result<u32, HttpError> {
        let url = format!("{}{}", self.base_url(), CANCEL_ALL);

        let response = self.make_authenticated_request(&url).await?;

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
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
    }

    /// Cancel all orders by currency
    ///
    /// Cancels all orders for the specified currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency to cancel orders for (BTC, ETH, USDC, etc.)
    ///
    /// # Returns
    ///
    /// Returns the number of cancelled orders.
    pub async fn cancel_all_by_currency(&self, currency: &str) -> Result<u32, HttpError> {
        let url = format!(
            "{}{}?currency={}",
            self.base_url(),
            CANCEL_ALL_BY_CURRENCY,
            urlencoding::encode(currency)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel all orders by currency failed: {}",
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
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
    }

    /// Cancel all orders by currency pair
    ///
    /// Cancels all orders for the specified currency pair.
    ///
    /// # Arguments
    ///
    /// * `currency_pair` - Currency pair to cancel orders for (e.g., "BTC_USD")
    ///
    /// # Returns
    ///
    /// Returns the number of cancelled orders.
    pub async fn cancel_all_by_currency_pair(&self, currency_pair: &str) -> Result<u32, HttpError> {
        let url = format!(
            "{}{}?currency_pair={}",
            self.base_url(),
            CANCEL_ALL_BY_CURRENCY_PAIR,
            urlencoding::encode(currency_pair)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel all orders by currency pair failed: {}",
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
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
    }

    /// Cancel all orders by instrument
    ///
    /// Cancels all orders for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name to cancel orders for (e.g., "BTC-PERPETUAL")
    ///
    /// # Returns
    ///
    /// Returns the number of cancelled orders.
    pub async fn cancel_all_by_instrument(&self, instrument_name: &str) -> Result<u32, HttpError> {
        let url = format!(
            "{}{}?instrument_name={}",
            self.base_url(),
            CANCEL_ALL_BY_INSTRUMENT,
            urlencoding::encode(instrument_name)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel all orders by instrument failed: {}",
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
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
    }

    /// Cancel all orders by kind or type
    ///
    /// Cancels all orders for the specified kind or type.
    ///
    /// # Arguments
    ///
    /// * `kind` - Kind of orders to cancel (future, option, spot, etc.) - optional
    /// * `order_type` - Type of orders to cancel (limit, market, etc.) - optional
    ///
    /// # Returns
    ///
    /// Returns the number of cancelled orders.
    pub async fn cancel_all_by_kind_or_type(
        &self,
        kind: Option<&str>,
        order_type: Option<&str>,
    ) -> Result<u32, HttpError> {
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
            "{}{}{}",
            self.base_url(),
            CANCEL_ALL_BY_KIND_OR_TYPE,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel all orders by kind or type failed: {}",
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
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
    }

    /// Cancel orders by label
    ///
    /// Cancels all orders with the specified label.
    ///
    /// # Arguments
    ///
    /// * `label` - Label of orders to cancel
    ///
    /// # Returns
    ///
    /// Returns the number of cancelled orders.
    pub async fn cancel_by_label(&self, label: &str) -> Result<u32, HttpError> {
        let url = format!(
            "{}{}?label={}",
            self.base_url(),
            CANCEL_BY_LABEL,
            urlencoding::encode(label)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel orders by label failed: {}",
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
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
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
    pub async fn get_account_summary(
        &self,
        currency: &str,
        extended: Option<bool>,
    ) -> Result<AccountSummaryResponse, HttpError> {
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
            "{}{}?{}",
            self.base_url(),
            GET_ACCOUNT_SUMMARY,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

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

        let api_response: ApiResponse<AccountSummaryResponse> = response
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
    /// let client = DeribitHttpClient::new();
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

        let url = format!("{}{}{}", self.base_url(), GET_POSITIONS, query_string);

        let response = self.make_authenticated_request(&url).await?;

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

    /// Get position for a specific instrument
    ///
    /// Retrieves the current position for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The name of the instrument to get position for
    ///
    /// # Returns
    ///
    /// Returns a vector of positions for the specified instrument
    ///
    pub async fn get_position(&self, instrument_name: &str) -> Result<Vec<Position>, HttpError> {
        let query_string = format!("instrument_name={}", instrument_name);
        let url = format!("{}{}{}", self.base_url(), GET_POSITION, query_string);
        let response = self.make_authenticated_request(&url).await?;

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

    /// Edit an order
    ///
    /// Edits an existing order.
    ///
    /// # Arguments
    ///
    /// * `request` - The edit order request parameters
    ///
    pub async fn edit_order(&self, request: OrderRequest) -> Result<OrderResponse, HttpError> {
        let order_id = request.order_id.ok_or_else(|| {
            HttpError::RequestFailed("order_id is required for edit_order".to_string())
        })?;
        let mut query_params = vec![("order_id", order_id.as_str())];

        let amount_str;
        if let Some(amount) = request.amount {
            amount_str = amount.to_string();
            query_params.push(("amount", amount_str.as_str()));
        }

        let price_str;
        if let Some(price) = request.price {
            price_str = price.to_string();
            query_params.push(("price", price_str.as_str()));
        }

        if let Some(post_only) = request.post_only
            && post_only
        {
            query_params.push(("post_only", "true"));
        }

        if let Some(reduce_only) = request.reduce_only
            && reduce_only
        {
            query_params.push(("reduce_only", "true"));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), EDIT, query_string);

        let response = self.make_authenticated_request(&url).await?;

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

    /// Edit an order by label
    ///
    /// Modifies an order identified by its label. This method works only when there
    /// is exactly one open order with the specified label.
    ///
    /// # Arguments
    ///
    /// * `request` - The edit order request parameters (must include label and instrument_name)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    /// use deribit_http::model::request::order::OrderRequest;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let request = OrderRequest {
    /// //     label: Some("my_order_label".to_string()),
    /// //     instrument_name: "BTC-PERPETUAL".to_string(),
    /// //     amount: Some(150.0),
    /// //     price: Some(50111.0),
    /// //     ..Default::default()
    /// // };
    /// // let result = client.edit_order_by_label(request).await?;
    /// ```
    pub async fn edit_order_by_label(
        &self,
        request: OrderRequest,
    ) -> Result<OrderResponse, HttpError> {
        let label = request.label.ok_or_else(|| {
            HttpError::RequestFailed("label is required for edit_order_by_label".to_string())
        })?;

        let mut query_params = vec![
            ("label".to_string(), label),
            ("instrument_name".to_string(), request.instrument_name),
        ];

        if let Some(amount) = request.amount {
            query_params.push(("amount".to_string(), amount.to_string()));
        }

        if let Some(contracts) = request.contracts {
            query_params.push(("contracts".to_string(), contracts.to_string()));
        }

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(post_only) = request.post_only
            && post_only
        {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if let Some(reduce_only) = request.reduce_only
            && reduce_only
        {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        if let Some(reject_post_only) = request.reject_post_only
            && reject_post_only
        {
            query_params.push(("reject_post_only".to_string(), "true".to_string()));
        }

        if let Some(advanced) = request.advanced {
            let advanced_str = match advanced {
                crate::model::request::order::AdvancedOrderType::Usd => "usd",
                crate::model::request::order::AdvancedOrderType::Implv => "implv",
            };
            query_params.push(("advanced".to_string(), advanced_str.to_string()));
        }

        if let Some(trigger_price) = request.trigger_price {
            query_params.push(("trigger_price".to_string(), trigger_price.to_string()));
        }

        if let Some(mmp) = request.mmp
            && mmp
        {
            query_params.push(("mmp".to_string(), "true".to_string()));
        }

        if let Some(valid_until) = request.valid_until {
            query_params.push(("valid_until".to_string(), valid_until.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), EDIT_BY_LABEL, query_string);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Edit order by label failed: {}",
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

    /// Close an existing position
    ///
    /// Places a reduce-only order to close an existing position. The order will
    /// automatically be set to reduce-only to ensure it only closes the position.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument identifier (e.g., "BTC-PERPETUAL")
    /// * `order_type` - Order type: "market" or "limit"
    /// * `price` - Optional price for limit orders (required if order_type is "limit")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // Close position with market order
    /// // let result = client.close_position("BTC-PERPETUAL", "market", None).await?;
    /// // Close position with limit order
    /// // let result = client.close_position("ETH-PERPETUAL", "limit", Some(2500.0)).await?;
    /// ```
    pub async fn close_position(
        &self,
        instrument_name: &str,
        order_type: &str,
        price: Option<f64>,
    ) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![
            ("instrument_name".to_string(), instrument_name.to_string()),
            ("type".to_string(), order_type.to_string()),
        ];

        if let Some(price) = price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), CLOSE_POSITION, query_string);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Close position failed: {}",
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

    /// Get margin requirements
    ///
    /// Calculates margin requirements for a hypothetical order on a given instrument.
    /// Returns initial margin and maintenance margin for the specified instrument,
    /// quantity, and price.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument identifier (e.g., "BTC-PERPETUAL")
    /// * `amount` - Order size (USD for perpetual/inverse, base currency for options/linear)
    /// * `price` - Order price
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let margins = client.get_margins("BTC-PERPETUAL", 10000.0, 50000.0).await?;
    /// // println!("Buy margin: {}, Sell margin: {}", margins.buy, margins.sell);
    /// ```
    pub async fn get_margins(
        &self,
        instrument_name: &str,
        amount: f64,
        price: f64,
    ) -> Result<MarginsResponse, HttpError> {
        let query_params = [
            ("instrument_name".to_string(), instrument_name.to_string()),
            ("amount".to_string(), amount.to_string()),
            ("price".to_string(), price.to_string()),
        ];

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), GET_MARGINS, query_string);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get margins failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<MarginsResponse> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No margin data in response".to_string()))
    }

    /// Get order margin by IDs
    ///
    /// Retrieves the initial margin requirements for one or more orders identified
    /// by their order IDs. Initial margin is the amount of funds required to open
    /// a position with these orders.
    ///
    /// # Arguments
    ///
    /// * `ids` - Array of order IDs (e.g., ["ETH-349280", "ETH-349279"])
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let margins = client.get_order_margin_by_ids(&["ETH-349280", "ETH-349279"]).await?;
    /// ```
    pub async fn get_order_margin_by_ids(
        &self,
        ids: &[&str],
    ) -> Result<Vec<OrderMargin>, HttpError> {
        if ids.is_empty() {
            return Err(HttpError::RequestFailed(
                "ids array cannot be empty".to_string(),
            ));
        }

        // Format IDs as JSON array for the query parameter
        let ids_json = serde_json::to_string(ids)
            .map_err(|e| HttpError::InvalidResponse(format!("Failed to serialize ids: {}", e)))?;

        let query_string = format!("ids={}", urlencoding::encode(&ids_json));
        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_ORDER_MARGIN_BY_IDS,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get order margin by IDs failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderMargin>> = response
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
            HttpError::InvalidResponse("No order margin data in response".to_string())
        })
    }

    /// Get order state by label
    ///
    /// Retrieves the state of recent orders that have a specific label.
    /// Results are filtered by currency and label. The response includes
    /// order details such as status, filled amount, remaining amount, and
    /// other order properties for all orders with the specified label.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (e.g., "BTC", "ETH", "USDC")
    /// * `label` - User-defined label (max 64 characters)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let orders = client.get_order_state_by_label("ETH", "myLabel").await?;
    /// ```
    pub async fn get_order_state_by_label(
        &self,
        currency: &str,
        label: &str,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
        let query_params = [
            ("currency".to_string(), currency.to_string()),
            ("label".to_string(), label.to_string()),
        ];

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_ORDER_STATE_BY_LABEL,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get order state by label failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderInfoResponse>> = response
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

    /// Get settlement history by currency
    ///
    /// Retrieves settlement, delivery, and bankruptcy events that have affected
    /// your account for a specific currency. Settlements occur when futures or
    /// options contracts expire and are settled at the delivery price.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (e.g., "BTC", "ETH", "USDC")
    /// * `settlement_type` - Settlement type: "settlement", "delivery", or "bankruptcy" (optional)
    /// * `count` - Number of items (default 20, max 1000) (optional)
    /// * `continuation` - Pagination token (optional)
    /// * `search_start_timestamp` - Latest timestamp to return results from in ms (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let history = client.get_settlement_history_by_currency("BTC", None, None, None, None).await?;
    /// ```
    pub async fn get_settlement_history_by_currency(
        &self,
        currency: &str,
        settlement_type: Option<&str>,
        count: Option<u32>,
        continuation: Option<&str>,
        search_start_timestamp: Option<u64>,
    ) -> Result<SettlementsResponse, HttpError> {
        let mut url = format!(
            "{}{}?currency={}",
            self.base_url(),
            GET_SETTLEMENT_HISTORY_BY_CURRENCY,
            urlencoding::encode(currency)
        );

        if let Some(settlement_type) = settlement_type {
            url.push_str(&format!("&type={}", urlencoding::encode(settlement_type)));
        }

        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        if let Some(continuation) = continuation {
            url.push_str(&format!(
                "&continuation={}",
                urlencoding::encode(continuation)
            ));
        }

        if let Some(search_start_timestamp) = search_start_timestamp {
            url.push_str(&format!(
                "&search_start_timestamp={}",
                search_start_timestamp
            ));
        }

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get settlement history by currency failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<SettlementsResponse> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No settlement data in response".to_string()))
    }

    /// Get settlement history by instrument
    ///
    /// Retrieves settlement, delivery, and bankruptcy events for a specific
    /// instrument that have affected your account. Settlements occur when futures
    /// or options contracts expire and are settled at the delivery price.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument identifier (e.g., "BTC-PERPETUAL")
    /// * `settlement_type` - Settlement type: "settlement", "delivery", or "bankruptcy" (optional)
    /// * `count` - Number of items (default 20, max 1000) (optional)
    /// * `continuation` - Pagination token (optional)
    /// * `search_start_timestamp` - Latest timestamp to return results from in ms (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let history = client.get_settlement_history_by_instrument("BTC-PERPETUAL", None, None, None, None).await?;
    /// ```
    pub async fn get_settlement_history_by_instrument(
        &self,
        instrument_name: &str,
        settlement_type: Option<&str>,
        count: Option<u32>,
        continuation: Option<&str>,
        search_start_timestamp: Option<u64>,
    ) -> Result<SettlementsResponse, HttpError> {
        let mut url = format!(
            "{}{}?instrument_name={}",
            self.base_url(),
            GET_SETTLEMENT_HISTORY_BY_INSTRUMENT,
            urlencoding::encode(instrument_name)
        );

        if let Some(settlement_type) = settlement_type {
            url.push_str(&format!("&type={}", urlencoding::encode(settlement_type)));
        }

        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        if let Some(continuation) = continuation {
            url.push_str(&format!(
                "&continuation={}",
                urlencoding::encode(continuation)
            ));
        }

        if let Some(search_start_timestamp) = search_start_timestamp {
            url.push_str(&format!(
                "&search_start_timestamp={}",
                search_start_timestamp
            ));
        }

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get settlement history by instrument failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<SettlementsResponse> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No settlement data in response".to_string()))
    }

    /// Get MMP configuration
    ///
    /// Retrieves Market Maker Protection (MMP) configuration for an index.
    /// If index_name is not provided, returns all MMP configurations.
    ///
    /// # Arguments
    ///
    /// * `index_name` - Index identifier (e.g., "btc_usd", "eth_usd"), optional
    /// * `mmp_group` - MMP group name for Mass Quotes, optional
    /// * `block_rfq` - If true, retrieve MMP config for Block RFQ, optional
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let configs = client.get_mmp_config(Some("btc_usd"), None, None).await?;
    /// ```
    pub async fn get_mmp_config(
        &self,
        index_name: Option<&str>,
        mmp_group: Option<&str>,
        block_rfq: Option<bool>,
    ) -> Result<Vec<MmpConfig>, HttpError> {
        let mut query_params: Vec<(String, String)> = Vec::new();

        if let Some(index) = index_name {
            query_params.push(("index_name".to_string(), index.to_string()));
        }

        if let Some(group) = mmp_group {
            query_params.push(("mmp_group".to_string(), group.to_string()));
        }

        if let Some(rfq) = block_rfq
            && rfq
        {
            query_params.push(("block_rfq".to_string(), "true".to_string()));
        }

        let url = if query_params.is_empty() {
            format!("{}{}", self.base_url(), GET_MMP_CONFIG)
        } else {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}{}?{}", self.base_url(), GET_MMP_CONFIG, query_string)
        };

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get MMP config failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<MmpConfig>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No MMP config data in response".to_string()))
    }

    /// Get MMP status
    ///
    /// Retrieves Market Maker Protection (MMP) status for a triggered index or MMP group.
    /// If index_name is not provided, returns all triggered MMP statuses.
    ///
    /// # Arguments
    ///
    /// * `index_name` - Index identifier (e.g., "btc_usd", "eth_usd"), optional
    /// * `mmp_group` - MMP group name for Mass Quotes, optional
    /// * `block_rfq` - If true, retrieve MMP status for Block RFQ, optional
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let statuses = client.get_mmp_status(Some("btc_usd"), None, None).await?;
    /// ```
    pub async fn get_mmp_status(
        &self,
        index_name: Option<&str>,
        mmp_group: Option<&str>,
        block_rfq: Option<bool>,
    ) -> Result<Vec<MmpStatus>, HttpError> {
        let mut query_params: Vec<(String, String)> = Vec::new();

        if let Some(index) = index_name {
            query_params.push(("index_name".to_string(), index.to_string()));
        }

        if let Some(group) = mmp_group {
            query_params.push(("mmp_group".to_string(), group.to_string()));
        }

        if let Some(rfq) = block_rfq
            && rfq
        {
            query_params.push(("block_rfq".to_string(), "true".to_string()));
        }

        let url = if query_params.is_empty() {
            format!("{}{}", self.base_url(), GET_MMP_STATUS)
        } else {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}{}?{}", self.base_url(), GET_MMP_STATUS, query_string)
        };

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get MMP status failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<MmpStatus>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No MMP status data in response".to_string()))
    }

    /// Set MMP configuration
    ///
    /// Configures Market Maker Protection (MMP) for a specific index.
    /// Set interval to 0 to remove MMP configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - The MMP configuration request
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    /// use deribit_http::model::response::mmp::SetMmpConfigRequest;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let request = SetMmpConfigRequest {
    /// //     index_name: "btc_usd".to_string(),
    /// //     interval: 60,
    /// //     frozen_time: 0,
    /// //     quantity_limit: Some(3.0),
    /// //     max_quote_quantity: Some(2.5),
    /// //     ..Default::default()
    /// // };
    /// // let config = client.set_mmp_config(request).await?;
    /// ```
    pub async fn set_mmp_config(
        &self,
        request: SetMmpConfigRequest,
    ) -> Result<MmpConfig, HttpError> {
        let mut query_params = vec![
            ("index_name".to_string(), request.index_name),
            ("interval".to_string(), request.interval.to_string()),
            ("frozen_time".to_string(), request.frozen_time.to_string()),
        ];

        if let Some(quantity_limit) = request.quantity_limit {
            query_params.push(("quantity_limit".to_string(), quantity_limit.to_string()));
        }

        if let Some(delta_limit) = request.delta_limit {
            query_params.push(("delta_limit".to_string(), delta_limit.to_string()));
        }

        if let Some(vega_limit) = request.vega_limit {
            query_params.push(("vega_limit".to_string(), vega_limit.to_string()));
        }

        if let Some(max_quote_quantity) = request.max_quote_quantity {
            query_params.push((
                "max_quote_quantity".to_string(),
                max_quote_quantity.to_string(),
            ));
        }

        if let Some(mmp_group) = request.mmp_group {
            query_params.push(("mmp_group".to_string(), mmp_group));
        }

        if let Some(block_rfq) = request.block_rfq
            && block_rfq
        {
            query_params.push(("block_rfq".to_string(), "true".to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), SET_MMP_CONFIG, query_string);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Set MMP config failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<MmpConfig> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No MMP config data in response".to_string()))
    }

    /// Reset MMP limits
    ///
    /// Resets Market Maker Protection (MMP) limits for the specified currency pair or MMP group.
    /// If MMP protection has been triggered and quoting is frozen, this allows manual resume.
    ///
    /// # Arguments
    ///
    /// * `index_name` - Currency pair (e.g., "btc_usd", "eth_usd")
    /// * `mmp_group` - MMP group name for Mass Quotes, optional
    /// * `block_rfq` - If true, reset MMP for Block RFQ, optional
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let result = client.reset_mmp("btc_usd", None, None).await?;
    /// ```
    pub async fn reset_mmp(
        &self,
        index_name: &str,
        mmp_group: Option<&str>,
        block_rfq: Option<bool>,
    ) -> Result<String, HttpError> {
        let mut query_params = vec![("index_name".to_string(), index_name.to_string())];

        if let Some(group) = mmp_group {
            query_params.push(("mmp_group".to_string(), group.to_string()));
        }

        if let Some(rfq) = block_rfq
            && rfq
        {
            query_params.push(("block_rfq".to_string(), "true".to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), RESET_MMP, query_string);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Reset MMP failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<String> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No result in response".to_string()))
    }

    /// Mass quote
    ///
    /// Places multiple quotes at once.
    ///
    /// # Arguments
    ///
    /// * `quotes` - Vector of mass quote requests
    ///
    pub async fn mass_quote(
        &self,
        _quotes: MassQuoteRequest,
    ) -> Result<MassQuoteResponse, HttpError> {
        Err(HttpError::ConfigError(
            "Mass quote endpoint is only available via WebSocket connections. \
             According to Deribit's technical specifications, private/mass_quote requires \
             WebSocket for real-time quote management, MMP group integration, and \
             Cancel-on-Disconnect functionality. Please use the deribit-websocket client \
             for mass quote operations."
                .to_string(),
        ))
    }

    /// Get user trades by instrument
    ///
    /// Retrieves user trades for a specific instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `start_seq` - Start sequence number (optional)
    /// * `end_seq` - End sequence number (optional)
    /// * `count` - Number of requested items (optional)
    /// * `include_old` - Include old trades (optional)
    /// * `sorting` - Direction of results sorting (optional)
    ///
    pub async fn get_user_trades_by_instrument(
        &self,
        instrument_name: &str,
        start_seq: Option<u64>,
        end_seq: Option<u64>,
        count: Option<u32>,
        include_old: Option<bool>,
        sorting: Option<&str>,
    ) -> Result<UserTradeWithPaginationResponse, HttpError> {
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
            "{}{}?{}",
            self.base_url(),
            GET_USER_TRADES_BY_INSTRUMENT,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get user trades by instrument failed: {}",
                error_text
            )));
        }

        // Debug: Log the raw response text before trying to parse it
        let response_text = response.text().await.map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to read response text: {}", e))
        })?;

        tracing::debug!(
            "Raw API response for get_user_trades_by_instrument: {}",
            response_text
        );

        // Try to parse as JSON
        let api_response: ApiResponse<UserTradeWithPaginationResponse> =
            serde_json::from_str(&response_text).map_err(|e| {
                HttpError::InvalidResponse(format!(
                    "error decoding response body: {} - Raw response: {}",
                    e, response_text
                ))
            })?;

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

    /// Cancel quotes
    ///
    /// Cancels all mass quotes.
    ///
    /// # Arguments
    ///
    /// * `cancel_type` - Type of cancellation ("all", "by_currency", "by_instrument", etc.)
    ///
    pub async fn cancel_quotes(&self, cancel_type: Option<&str>) -> Result<u32, HttpError> {
        let mut url = format!("{}{}", self.base_url(), CANCEL_QUOTES);

        if let Some(cancel_type) = cancel_type {
            url.push_str(&format!(
                "?cancel_type={}",
                urlencoding::encode(cancel_type)
            ));
        } else {
            url.push_str("?cancel_type=all");
        }

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel quotes failed: {}",
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
            .ok_or_else(|| HttpError::InvalidResponse("No cancel result in response".to_string()))
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
    pub async fn get_open_orders(
        &self,
        kind: Option<&str>,
        order_type: Option<&str>,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
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

        let url = format!("{}{}{}", self.base_url(), GET_OPEN_ORDERS, query_string);

        let response = self.make_authenticated_request(&url).await?;

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

        let api_response: ApiResponse<Vec<OrderInfoResponse>> = response
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

    /// Get open orders by label
    ///
    /// Retrieves open orders filtered by a specific label.
    ///
    /// # Arguments
    ///
    /// * `label` - The label to filter orders by
    /// * `currency` - The currency symbol (BTC, ETH, etc.)
    ///
    pub async fn get_open_orders_by_label(
        &self,
        label: &str,
        currency: &str,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
        let url = format!(
            "{}{}?label={}&currency={}",
            self.base_url(),
            GET_OPEN_ORDERS_BY_LABEL,
            urlencoding::encode(label),
            urlencoding::encode(currency)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get open orders by label failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderInfoResponse>> = response
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

    /// Get order state
    ///
    /// Retrieves the state of a specific order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order ID
    ///
    pub async fn get_order_state(&self, order_id: &str) -> Result<OrderInfoResponse, HttpError> {
        let url = format!(
            "{}{}?order_id={}",
            self.base_url(),
            GET_ORDER_STATE,
            urlencoding::encode(order_id)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get order state failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderInfoResponse> = response
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

    /// Get open orders by currency
    ///
    /// Retrieves open orders for a specific currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (BTC, ETH, etc.)
    /// * `kind` - Instrument kind filter (optional)
    /// * `order_type` - Order type filter (optional)
    ///
    pub async fn get_open_orders_by_currency(
        &self,
        currency: &str,
        kind: Option<&str>,
        order_type: Option<&str>,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
        let mut query_params = vec![("currency".to_string(), currency.to_string())];

        if let Some(kind) = kind {
            query_params.push(("kind".to_string(), kind.to_string()));
        }

        if let Some(order_type) = order_type {
            query_params.push(("type".to_string(), order_type.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_OPEN_ORDERS_BY_CURRENCY,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get open orders by currency failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderInfoResponse>> = response
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

    /// Get open orders by instrument
    ///
    /// Retrieves open orders for a specific instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument name
    /// * `order_type` - Order type filter (optional)
    ///
    pub async fn get_open_orders_by_instrument(
        &self,
        instrument_name: &str,
        order_type: Option<&str>,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
        let mut query_params = vec![("instrument_name".to_string(), instrument_name.to_string())];

        if let Some(order_type) = order_type {
            query_params.push(("type".to_string(), order_type.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_OPEN_ORDERS_BY_INSTRUMENT,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get open orders by instrument failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderInfoResponse>> = response
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
    pub async fn get_order_history(
        &self,
        currency: &str,
        kind: Option<&str>,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
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
            "{}{}?{}",
            self.base_url(),
            GET_ORDER_HISTORY_BY_CURRENCY,
            query_string
        );
        let response = self.make_authenticated_request(&url).await?;

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

        let api_response: ApiResponse<Vec<OrderInfoResponse>> = response
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

    /// Get order history by currency
    ///
    /// Retrieves order history for a specific currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    /// * `kind` - Instrument kind filter (optional)
    /// * `count` - Number of requested items (optional)
    /// * `offset` - Offset for pagination (optional)
    ///
    pub async fn get_order_history_by_currency(
        &self,
        currency: &str,
        kind: Option<&str>,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
        // This is an alias to the existing get_order_history method
        self.get_order_history(currency, kind, count, offset).await
    }

    /// Get order history by instrument
    ///
    /// Retrieves order history for a specific instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument name
    /// * `count` - Number of requested items (optional)
    /// * `offset` - Offset for pagination (optional)
    ///
    pub async fn get_order_history_by_instrument(
        &self,
        instrument_name: &str,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<OrderInfoResponse>, HttpError> {
        let mut query_params = vec![("instrument_name".to_string(), instrument_name.to_string())];

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
            "{}{}?{}",
            self.base_url(),
            GET_ORDER_HISTORY_BY_INSTRUMENT,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get order history by instrument failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<OrderInfoResponse>> = response
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

    /// Get user trades by currency
    ///
    /// Retrieves user trades filtered by currency.
    ///
    /// # Arguments
    ///
    /// * `request` - A `TradesRequest` struct containing:
    ///   * `currency` - Currency symbol (BTC, ETH, etc.)
    ///   * `kind` - Instrument kind filter (optional)
    ///   * `start_id` - The ID of the first trade to be returned (optional)
    ///   * `end_id` - The ID of the last trade to be returned (optional)
    ///   * `count` - Number of requested items (optional, default 10, max 1000)
    ///   * `start_timestamp` - The earliest timestamp to return results from (optional)
    ///   * `end_timestamp` - The most recent timestamp to return results from (optional)
    ///   * `sorting` - Direction of results sorting (optional)
    ///   * `historical` - If true, retrieves historical records that persist indefinitely.
    ///     If false (default), retrieves recent records available for 24 hours.
    ///   * `subaccount_id` - The user id for the subaccount (optional)
    ///
    #[allow(clippy::too_many_arguments)]
    pub async fn get_user_trades_by_currency(
        &self,
        request: TradesRequest,
    ) -> Result<UserTradeWithPaginationResponse, HttpError> {
        let mut query_params = vec![("currency".to_string(), request.currency.to_string())];

        if let Some(kind) = request.kind {
            query_params.push(("kind".to_string(), kind.to_string()));
        }

        if let Some(start_id) = request.start_id {
            query_params.push(("start_id".to_string(), start_id));
        }

        if let Some(end_id) = request.end_id {
            query_params.push(("end_id".to_string(), end_id));
        }

        if let Some(count) = request.count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(start_timestamp) = request.start_timestamp {
            query_params.push(("start_timestamp".to_string(), start_timestamp.to_string()));
        }

        if let Some(end_timestamp) = request.end_timestamp {
            query_params.push(("end_timestamp".to_string(), end_timestamp.to_string()));
        }

        if let Some(sorting) = request.sorting {
            query_params.push(("sorting".to_string(), sorting.to_string()));
        }

        if let Some(historical) = request.historical {
            query_params.push(("historical".to_string(), historical.to_string()));
        }

        if let Some(subaccount_id) = request.subaccount_id {
            query_params.push(("subaccount_id".to_string(), subaccount_id.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_USER_TRADES_BY_CURRENCY,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get user trades by currency failed: {}",
                error_text
            )));
        }

        // Debug: Log the raw response text before trying to parse it
        let response_text = response.text().await.map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to read response text: {}", e))
        })?;

        tracing::debug!(
            "Raw API response for get_user_trades_by_order: {}",
            response_text
        );

        // Try to parse as JSON
        let api_response: ApiResponse<UserTradeWithPaginationResponse> =
            serde_json::from_str(&response_text).map_err(|e| {
                HttpError::InvalidResponse(format!(
                    "error decoding response body: {} - Raw response: {}",
                    e, response_text
                ))
            })?;

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

    /// Get user trades by currency and time
    ///
    /// Retrieves user trades filtered by currency within a time range.
    ///
    /// # Arguments
    ///
    /// * `request` - A `TradesRequest` struct containing:
    ///   * `currency` - Currency symbol (BTC, ETH, etc.)
    ///   * `kind` - Instrument kind filter (optional)
    ///   * `start_id` - The ID of the first trade to be returned (optional)
    ///   * `end_id` - The ID of the last trade to be returned (optional)
    ///   * `count` - Number of requested items (optional, default 10, max 1000)
    ///   * `start_timestamp` - The earliest timestamp to return results from (optional)
    ///   * `end_timestamp` - The most recent timestamp to return results from (optional)
    ///   * `sorting` - Direction of results sorting (optional)
    ///   * `historical` - If true, retrieves historical records that persist indefinitely.
    ///     If false (default), retrieves recent records available for 24 hours.
    ///   * `subaccount_id` - The user id for the subaccount (optional)
    ///
    #[allow(clippy::too_many_arguments)]
    pub async fn get_user_trades_by_currency_and_time(
        &self,
        request: TradesRequest,
    ) -> Result<UserTradeWithPaginationResponse, HttpError> {
        let mut query_params = vec![("currency".to_string(), request.currency.to_string())];

        if let Some(kind) = request.kind {
            query_params.push(("kind".to_string(), kind.to_string()));
        }

        if let Some(start_id) = request.start_id {
            query_params.push(("start_id".to_string(), start_id));
        }

        if let Some(end_id) = request.end_id {
            query_params.push(("end_id".to_string(), end_id));
        }

        if let Some(count) = request.count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        if let Some(start_timestamp) = request.start_timestamp {
            query_params.push(("start_timestamp".to_string(), start_timestamp.to_string()));
        }

        if let Some(end_timestamp) = request.end_timestamp {
            query_params.push(("end_timestamp".to_string(), end_timestamp.to_string()));
        }

        if let Some(sorting) = request.sorting {
            query_params.push(("sorting".to_string(), sorting.to_string()));
        }

        if let Some(historical) = request.historical {
            query_params.push(("historical".to_string(), historical.to_string()));
        }

        if let Some(subaccount_id) = request.subaccount_id {
            query_params.push(("subaccount_id".to_string(), subaccount_id.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_USER_TRADES_BY_CURRENCY_AND_TIME,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get user trades by currency and time failed: {}",
                error_text
            )));
        }

        // Debug: Log the raw response text before trying to parse it
        let response_text = response.text().await.map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to read response text: {}", e))
        })?;

        tracing::debug!(
            "Raw API response for get_user_trades_by_order: {}",
            response_text
        );

        // Try to parse as JSON
        let api_response: ApiResponse<UserTradeWithPaginationResponse> =
            serde_json::from_str(&response_text).map_err(|e| {
                HttpError::InvalidResponse(format!(
                    "error decoding response body: {} - Raw response: {}",
                    e, response_text
                ))
            })?;

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

    /// Get user trades by instrument and time
    ///
    /// Retrieves user trades for a specific instrument within a time range.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `start_timestamp` - Start timestamp in milliseconds
    /// * `end_timestamp` - End timestamp in milliseconds
    /// * `count` - Number of requested items (optional, default 10)
    /// * `include_old` - Include trades older than 7 days (optional)
    /// * `sorting` - Direction of results sorting (optional)
    ///
    pub async fn get_user_trades_by_instrument_and_time(
        &self,
        instrument_name: &str,
        start_timestamp: u64,
        end_timestamp: u64,
        count: Option<u32>,
        include_old: Option<bool>,
        sorting: Option<&str>,
    ) -> Result<UserTradeWithPaginationResponse, HttpError> {
        let mut query_params = vec![
            ("instrument_name".to_string(), instrument_name.to_string()),
            ("start_timestamp".to_string(), start_timestamp.to_string()),
            ("end_timestamp".to_string(), end_timestamp.to_string()),
        ];

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
            "{}{}?{}",
            self.base_url(),
            GET_USER_TRADES_BY_INSTRUMENT_AND_TIME,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get user trades by instrument and time failed: {}",
                error_text
            )));
        }

        // Debug: Log the raw response text before trying to parse it
        let response_text = response.text().await.map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to read response text: {}", e))
        })?;

        tracing::debug!(
            "Raw API response for get_user_trades_by_instrument_and_time: {}",
            response_text
        );

        // Try to parse as JSON
        let api_response: ApiResponse<UserTradeWithPaginationResponse> =
            serde_json::from_str(&response_text).map_err(|e| {
                HttpError::InvalidResponse(format!(
                    "error decoding response body: {} - Raw response: {}",
                    e, response_text
                ))
            })?;

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

    /// Get user trades by order
    ///
    /// Retrieves user trades for a specific order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - Order ID
    /// * `sorting` - Direction of results sorting (optional)
    ///
    pub async fn get_user_trades_by_order(
        &self,
        order_id: &str,
        sorting: Option<&str>,
        historical: bool,
    ) -> Result<Vec<UserTradeResponseByOrder>, HttpError> {
        let mut query_params = vec![("order_id".to_string(), order_id.to_string())];

        if let Some(sorting) = sorting {
            query_params.push(("sorting".to_string(), sorting.to_string()));
        }
        if historical {
            query_params.push(("historical".to_string(), historical.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            GET_USER_TRADES_BY_ORDER,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get user trades by order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<UserTradeResponseByOrder>> = response
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
