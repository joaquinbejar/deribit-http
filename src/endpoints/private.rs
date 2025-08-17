//! Private endpoints for authenticated API calls

use crate::DeribitHttpClient;
use crate::endpoints::types::*;
use crate::error::HttpError;
use crate::model::http_types::ApiResponse;

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
