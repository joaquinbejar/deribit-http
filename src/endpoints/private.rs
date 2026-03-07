//! Private endpoints for authenticated API calls

use crate::DeribitHttpClient;
use crate::constants::endpoints::*;
use crate::error::HttpError;
use crate::model::account::Subaccount;
use crate::model::api_key::{ApiKeyInfo, CreateApiKeyRequest, EditApiKeyRequest};
use crate::model::position::Position;
use crate::model::request::mass_quote::MassQuoteRequest;
use crate::model::request::order::OrderRequest;
use crate::model::request::position::MovePositionTrade;
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
use crate::model::response::position::MovePositionResult;
use crate::model::response::subaccount::SubaccountDetails;
use crate::model::response::trigger::TriggerOrderHistoryResponse;
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

    /// Get subaccounts details with positions
    ///
    /// Retrieves position details for all subaccounts for a specific currency.
    /// Returns positions aggregated across all subaccounts, including size,
    /// average entry price, mark price, and P&L information.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, USDC, etc.)
    /// * `with_open_orders` - Include open orders for each subaccount (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let details = client.get_subaccounts_details("BTC", Some(true)).await?;
    /// ```
    pub async fn get_subaccounts_details(
        &self,
        currency: &str,
        with_open_orders: Option<bool>,
    ) -> Result<Vec<SubaccountDetails>, HttpError> {
        let mut url = format!(
            "{}{}?currency={}",
            self.base_url(),
            GET_SUBACCOUNTS_DETAILS,
            urlencoding::encode(currency)
        );

        if let Some(with_open_orders) = with_open_orders {
            url.push_str(&format!("&with_open_orders={}", with_open_orders));
        }

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get subaccounts details failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<SubaccountDetails>> = response
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
            HttpError::InvalidResponse("No subaccounts details data in response".to_string())
        })
    }

    /// Create a new subaccount
    ///
    /// Creates a new subaccount under the main account.
    ///
    /// # Returns
    ///
    /// Returns the newly created `Subaccount` with its details.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or if the user is not a main account.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let subaccount = client.create_subaccount().await?;
    /// // tracing::info!("Created subaccount with ID: {}", subaccount.id);
    /// ```
    pub async fn create_subaccount(&self) -> Result<Subaccount, HttpError> {
        let url = format!("{}{}", self.base_url(), CREATE_SUBACCOUNT);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Create subaccount failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Subaccount> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No subaccount data in response".to_string()))
    }

    /// Remove an empty subaccount
    ///
    /// Removes a subaccount that has no open positions or pending orders.
    ///
    /// # Arguments
    ///
    /// * `subaccount_id` - The ID of the subaccount to remove
    ///
    /// # Returns
    ///
    /// Returns `"ok"` on success.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or if the subaccount is not empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let result = client.remove_subaccount(123).await?;
    /// // assert_eq!(result, "ok");
    /// ```
    pub async fn remove_subaccount(&self, subaccount_id: u64) -> Result<String, HttpError> {
        let url = format!(
            "{}{}?subaccount_id={}",
            self.base_url(),
            REMOVE_SUBACCOUNT,
            subaccount_id
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Remove subaccount failed: {}",
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

    /// Change the name of a subaccount
    ///
    /// Updates the username for a subaccount.
    ///
    /// # Arguments
    ///
    /// * `sid` - The subaccount ID
    /// * `name` - The new username for the subaccount
    ///
    /// # Returns
    ///
    /// Returns `"ok"` on success.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let result = client.change_subaccount_name(123, "new_name").await?;
    /// // assert_eq!(result, "ok");
    /// ```
    pub async fn change_subaccount_name(&self, sid: u64, name: &str) -> Result<String, HttpError> {
        let url = format!(
            "{}{}?sid={}&name={}",
            self.base_url(),
            CHANGE_SUBACCOUNT_NAME,
            sid,
            urlencoding::encode(name)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Change subaccount name failed: {}",
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

    /// Enable or disable login for a subaccount
    ///
    /// Toggles whether a subaccount can log in. If login is disabled and a session
    /// for the subaccount exists, that session will be terminated.
    ///
    /// # Arguments
    ///
    /// * `sid` - The subaccount ID
    /// * `state` - Either `"enable"` or `"disable"`
    ///
    /// # Returns
    ///
    /// Returns `"ok"` on success.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let result = client.toggle_subaccount_login(123, "enable").await?;
    /// // assert_eq!(result, "ok");
    /// ```
    pub async fn toggle_subaccount_login(
        &self,
        sid: u64,
        state: &str,
    ) -> Result<String, HttpError> {
        let url = format!(
            "{}{}?sid={}&state={}",
            self.base_url(),
            TOGGLE_SUBACCOUNT_LOGIN,
            sid,
            urlencoding::encode(state)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Toggle subaccount login failed: {}",
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

    /// Set email address for a subaccount
    ///
    /// Assigns an email address to a subaccount. The user will receive an email
    /// with a confirmation link.
    ///
    /// # Arguments
    ///
    /// * `sid` - The subaccount ID
    /// * `email` - The email address to assign
    ///
    /// # Returns
    ///
    /// Returns `"ok"` on success.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let result = client.set_email_for_subaccount(123, "user@example.com").await?;
    /// // assert_eq!(result, "ok");
    /// ```
    pub async fn set_email_for_subaccount(
        &self,
        sid: u64,
        email: &str,
    ) -> Result<String, HttpError> {
        let url = format!(
            "{}{}?sid={}&email={}",
            self.base_url(),
            SET_EMAIL_FOR_SUBACCOUNT,
            sid,
            urlencoding::encode(email)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Set email for subaccount failed: {}",
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

    /// Enable or disable notifications for a subaccount
    ///
    /// Toggles whether the main account receives notifications from a subaccount.
    ///
    /// # Arguments
    ///
    /// * `sid` - The subaccount ID
    /// * `state` - `true` to enable notifications, `false` to disable
    ///
    /// # Returns
    ///
    /// Returns `"ok"` on success.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let result = client.toggle_notifications_from_subaccount(123, true).await?;
    /// // assert_eq!(result, "ok");
    /// ```
    pub async fn toggle_notifications_from_subaccount(
        &self,
        sid: u64,
        state: bool,
    ) -> Result<String, HttpError> {
        let url = format!(
            "{}{}?sid={}&state={}",
            self.base_url(),
            TOGGLE_NOTIFICATIONS_FROM_SUBACCOUNT,
            sid,
            state
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Toggle notifications from subaccount failed: {}",
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

    /// Get account summaries for all currencies
    ///
    /// Retrieves a per-currency list of account summaries for the authenticated user.
    /// Each summary includes balance, equity, available funds, and margin information
    /// for each currency. Unlike `get_account_summary`, this returns data for all
    /// currencies at once.
    ///
    /// # Arguments
    ///
    /// * `subaccount_id` - Retrieve summaries for a specific subaccount (optional)
    /// * `extended` - Include additional account details (id, username, email, type) (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let summaries = client.get_account_summaries(None, Some(true)).await?;
    /// ```
    pub async fn get_account_summaries(
        &self,
        subaccount_id: Option<i64>,
        extended: Option<bool>,
    ) -> Result<AccountSummaryResponse, HttpError> {
        let mut url = format!("{}{}", self.base_url(), GET_ACCOUNT_SUMMARIES);

        let mut params = Vec::new();

        if let Some(subaccount_id) = subaccount_id {
            params.push(format!("subaccount_id={}", subaccount_id));
        }

        if let Some(extended) = extended {
            params.push(format!("extended={}", extended));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get account summaries failed: {}",
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
            HttpError::InvalidResponse("No account summaries data in response".to_string())
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

    /// Get trigger order history
    ///
    /// Retrieves a detailed log of all trigger orders (stop orders, take-profit orders, etc.)
    /// for the authenticated account. The log includes trigger order creation, activation,
    /// execution, and cancellation events.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (e.g., "BTC", "ETH", "USDC")
    /// * `instrument_name` - Filter by specific instrument (optional)
    /// * `count` - Number of items (default 20, max 1000) (optional)
    /// * `continuation` - Pagination token (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new();
    /// // let history = client.get_trigger_order_history("BTC", None, None, None).await?;
    /// ```
    pub async fn get_trigger_order_history(
        &self,
        currency: &str,
        instrument_name: Option<&str>,
        count: Option<u32>,
        continuation: Option<&str>,
    ) -> Result<TriggerOrderHistoryResponse, HttpError> {
        let mut url = format!(
            "{}{}?currency={}",
            self.base_url(),
            GET_TRIGGER_ORDER_HISTORY,
            urlencoding::encode(currency)
        );

        if let Some(instrument_name) = instrument_name {
            url.push_str(&format!(
                "&instrument_name={}",
                urlencoding::encode(instrument_name)
            ));
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

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get trigger order history failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TriggerOrderHistoryResponse> = response
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
            HttpError::InvalidResponse("No trigger order history data in response".to_string())
        })
    }

    /// Move positions between subaccounts
    ///
    /// Moves positions from a source subaccount to a target subaccount. This operation
    /// transfers open positions between subaccounts, which is useful for rebalancing
    /// or reorganizing trading activities.
    ///
    /// **Rate Limits**: 6 requests/minute, 100 move_position uses per week (168 hours)
    ///
    /// **Important**: In rare cases, the request may return an internal_server_error.
    /// This does not necessarily mean the operation failed entirely. Part or all of
    /// the position transfer might have still been processed successfully.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (e.g., "BTC", "ETH", "USDC")
    /// * `source_uid` - Source subaccount ID
    /// * `target_uid` - Target subaccount ID
    /// * `trades` - List of position trades to move
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    /// use deribit_http::model::request::position::MovePositionTrade;
    ///
    /// let client = DeribitHttpClient::new();
    /// let trades = vec![
    ///     MovePositionTrade::with_price("BTC-PERPETUAL", 110.0, 35800.0),
    /// ];
    /// // let results = client.move_positions("BTC", 3, 23, &trades).await?;
    /// ```
    pub async fn move_positions(
        &self,
        currency: &str,
        source_uid: i64,
        target_uid: i64,
        trades: &[MovePositionTrade],
    ) -> Result<Vec<MovePositionResult>, HttpError> {
        let mut url = format!(
            "{}{}?currency={}&source_uid={}&target_uid={}",
            self.base_url(),
            MOVE_POSITIONS,
            urlencoding::encode(currency),
            source_uid,
            target_uid
        );

        // Build trades array as JSON
        let trades_json = serde_json::to_string(trades).map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to serialize trades: {}", e))
        })?;
        url.push_str(&format!("&trades={}", urlencoding::encode(&trades_json)));

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Move positions failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<MovePositionResult>> = response
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
            HttpError::InvalidResponse("No move positions data in response".to_string())
        })
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

    // ==================== API Key Management ====================

    /// Create a new API key
    ///
    /// Creates a new API key with the specified scope and optional settings.
    ///
    /// # Arguments
    ///
    /// * `request` - The create API key request parameters
    ///
    /// # Returns
    ///
    /// Returns the newly created API key information including client_id and client_secret.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or authentication is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use deribit_http::{DeribitHttpClient, model::CreateApiKeyRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// let request = CreateApiKeyRequest {
    ///     max_scope: "account:read trade:read_write".to_string(),
    ///     name: Some("my_trading_key".to_string()),
    ///     ..Default::default()
    /// };
    /// let api_key = client.create_api_key(request).await?;
    /// println!("Created API key: {}", api_key.client_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_api_key(
        &self,
        request: CreateApiKeyRequest,
    ) -> Result<ApiKeyInfo, HttpError> {
        let mut query_params = vec![("max_scope".to_string(), request.max_scope)];

        if let Some(name) = request.name {
            query_params.push(("name".to_string(), name));
        }

        if let Some(public_key) = request.public_key {
            query_params.push(("public_key".to_string(), public_key));
        }

        if let Some(enabled_features) = request.enabled_features {
            for feature in enabled_features {
                query_params.push(("enabled_features".to_string(), feature));
            }
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), CREATE_API_KEY, query_string);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Create API key failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ApiKeyInfo> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API key data in response".to_string()))
    }

    /// Edit an existing API key
    ///
    /// Modifies an existing API key's scope, name, or other settings.
    ///
    /// # Arguments
    ///
    /// * `request` - The edit API key request parameters
    ///
    /// # Returns
    ///
    /// Returns the updated API key information.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the API key is not found.
    pub async fn edit_api_key(&self, request: EditApiKeyRequest) -> Result<ApiKeyInfo, HttpError> {
        let mut query_params = vec![
            ("id".to_string(), request.id.to_string()),
            ("max_scope".to_string(), request.max_scope),
        ];

        if let Some(name) = request.name {
            query_params.push(("name".to_string(), name));
        }

        if let Some(enabled) = request.enabled {
            query_params.push(("enabled".to_string(), enabled.to_string()));
        }

        if let Some(enabled_features) = request.enabled_features {
            for feature in enabled_features {
                query_params.push(("enabled_features".to_string(), feature));
            }
        }

        if let Some(ip_whitelist) = request.ip_whitelist {
            for ip in ip_whitelist {
                query_params.push(("ip_whitelist".to_string(), ip));
            }
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}{}?{}", self.base_url(), EDIT_API_KEY, query_string);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Edit API key failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ApiKeyInfo> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API key data in response".to_string()))
    }

    /// Disable an API key
    ///
    /// Disables the API key with the specified ID. The key cannot be used
    /// for authentication until it is re-enabled.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key to disable
    ///
    /// # Returns
    ///
    /// Returns the updated API key information with `enabled` set to `false`.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the API key is not found.
    pub async fn disable_api_key(&self, id: u64) -> Result<ApiKeyInfo, HttpError> {
        let url = format!("{}{}?id={}", self.base_url(), DISABLE_API_KEY, id);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Disable API key failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ApiKeyInfo> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API key data in response".to_string()))
    }

    /// Enable an API key
    ///
    /// Enables a previously disabled API key with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key to enable
    ///
    /// # Returns
    ///
    /// Returns the updated API key information with `enabled` set to `true`.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the API key is not found.
    pub async fn enable_api_key(&self, id: u64) -> Result<ApiKeyInfo, HttpError> {
        let url = format!("{}{}?id={}", self.base_url(), ENABLE_API_KEY, id);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Enable API key failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ApiKeyInfo> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API key data in response".to_string()))
    }

    /// List all API keys
    ///
    /// Retrieves a list of all API keys associated with the account.
    ///
    /// # Returns
    ///
    /// Returns a vector of API key information.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or authentication is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use deribit_http::DeribitHttpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// let api_keys = client.list_api_keys().await?;
    /// for key in api_keys {
    ///     println!("Key ID: {}, Name: {}, Enabled: {}", key.id, key.name, key.enabled);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_api_keys(&self) -> Result<Vec<ApiKeyInfo>, HttpError> {
        let url = format!("{}{}", self.base_url(), LIST_API_KEYS);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "List API keys failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<ApiKeyInfo>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API keys data in response".to_string()))
    }

    /// Remove an API key
    ///
    /// Permanently removes the API key with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key to remove
    ///
    /// # Returns
    ///
    /// Returns `"ok"` on success.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the API key is not found.
    pub async fn remove_api_key(&self, id: u64) -> Result<String, HttpError> {
        let url = format!("{}{}?id={}", self.base_url(), REMOVE_API_KEY, id);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Remove API key failed: {}",
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

    /// Reset an API key secret
    ///
    /// Generates a new client_secret for the API key with the specified ID.
    /// The old secret will no longer be valid.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key to reset
    ///
    /// # Returns
    ///
    /// Returns the updated API key information with the new client_secret.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the API key is not found.
    pub async fn reset_api_key(&self, id: u64) -> Result<ApiKeyInfo, HttpError> {
        let url = format!("{}{}?id={}", self.base_url(), RESET_API_KEY, id);

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Reset API key failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ApiKeyInfo> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API key data in response".to_string()))
    }

    /// Change API key name
    ///
    /// Changes the name of the API key with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key
    /// * `name` - The new name (only letters, numbers and underscores; max 16 characters)
    ///
    /// # Returns
    ///
    /// Returns the updated API key information.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the API key is not found.
    pub async fn change_api_key_name(&self, id: u64, name: &str) -> Result<ApiKeyInfo, HttpError> {
        let url = format!(
            "{}{}?id={}&name={}",
            self.base_url(),
            CHANGE_API_KEY_NAME,
            id,
            urlencoding::encode(name)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Change API key name failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ApiKeyInfo> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API key data in response".to_string()))
    }

    /// Change API key scope
    ///
    /// Changes the maximum scope of the API key with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key
    /// * `max_scope` - The new maximum scope (e.g., "account:read trade:read_write")
    ///
    /// # Returns
    ///
    /// Returns the updated API key information.
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the API key is not found.
    pub async fn change_scope_in_api_key(
        &self,
        id: u64,
        max_scope: &str,
    ) -> Result<ApiKeyInfo, HttpError> {
        let url = format!(
            "{}{}?id={}&max_scope={}",
            self.base_url(),
            CHANGE_SCOPE_IN_API_KEY,
            id,
            urlencoding::encode(max_scope)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Change API key scope failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ApiKeyInfo> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No API key data in response".to_string()))
    }

    // ========================================================================
    // Address Beneficiary Endpoints
    // ========================================================================

    /// Save address beneficiary information.
    ///
    /// Saves beneficiary information for a cryptocurrency address,
    /// required for travel rule compliance.
    ///
    /// # Arguments
    ///
    /// * `request` - The beneficiary information to save
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the response is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use deribit_http::DeribitHttpClient;
    /// use deribit_http::model::SaveAddressBeneficiaryRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// let request = SaveAddressBeneficiaryRequest {
    ///     currency: "BTC".to_string(),
    ///     address: "bc1qtest".to_string(),
    ///     agreed: true,
    ///     personal: false,
    ///     unhosted: false,
    ///     beneficiary_vasp_name: "Test VASP".to_string(),
    ///     beneficiary_vasp_did: "did:test:123".to_string(),
    ///     beneficiary_address: "Test Address".to_string(),
    ///     ..Default::default()
    /// };
    /// // let beneficiary = client.save_address_beneficiary(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save_address_beneficiary(
        &self,
        request: &crate::model::SaveAddressBeneficiaryRequest,
    ) -> Result<crate::model::AddressBeneficiary, HttpError> {
        let mut params = vec![
            format!("currency={}", urlencoding::encode(&request.currency)),
            format!("address={}", urlencoding::encode(&request.address)),
            format!("agreed={}", request.agreed),
            format!("personal={}", request.personal),
            format!("unhosted={}", request.unhosted),
            format!(
                "beneficiary_vasp_name={}",
                urlencoding::encode(&request.beneficiary_vasp_name)
            ),
            format!(
                "beneficiary_vasp_did={}",
                urlencoding::encode(&request.beneficiary_vasp_did)
            ),
            format!(
                "beneficiary_address={}",
                urlencoding::encode(&request.beneficiary_address)
            ),
        ];

        if let Some(ref tag) = request.tag {
            params.push(format!("tag={}", urlencoding::encode(tag)));
        }
        if let Some(ref website) = request.beneficiary_vasp_website {
            params.push(format!(
                "beneficiary_vasp_website={}",
                urlencoding::encode(website)
            ));
        }
        if let Some(ref first_name) = request.beneficiary_first_name {
            params.push(format!(
                "beneficiary_first_name={}",
                urlencoding::encode(first_name)
            ));
        }
        if let Some(ref last_name) = request.beneficiary_last_name {
            params.push(format!(
                "beneficiary_last_name={}",
                urlencoding::encode(last_name)
            ));
        }
        if let Some(ref company_name) = request.beneficiary_company_name {
            params.push(format!(
                "beneficiary_company_name={}",
                urlencoding::encode(company_name)
            ));
        }

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            SAVE_ADDRESS_BENEFICIARY,
            params.join("&")
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Save address beneficiary failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::AddressBeneficiary> = response
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
            HttpError::InvalidResponse("No beneficiary data in response".to_string())
        })
    }

    /// Delete address beneficiary information.
    ///
    /// Removes beneficiary information for the specified address.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (e.g., "BTC", "ETH")
    /// * `address` - The cryptocurrency address
    /// * `tag` - Optional tag for XRP addresses
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the response is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use deribit_http::DeribitHttpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// // let result = client.delete_address_beneficiary("BTC", "bc1qtest", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_address_beneficiary(
        &self,
        currency: &str,
        address: &str,
        tag: Option<&str>,
    ) -> Result<String, HttpError> {
        let mut url = format!(
            "{}{}?currency={}&address={}",
            self.base_url(),
            DELETE_ADDRESS_BENEFICIARY,
            urlencoding::encode(currency),
            urlencoding::encode(address)
        );

        if let Some(t) = tag {
            url.push_str(&format!("&tag={}", urlencoding::encode(t)));
        }

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Delete address beneficiary failed: {}",
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

    /// Get address beneficiary information.
    ///
    /// Retrieves beneficiary information for the specified address.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (e.g., "BTC", "ETH")
    /// * `address` - The cryptocurrency address
    /// * `tag` - Optional tag for XRP addresses
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the response is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use deribit_http::DeribitHttpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// // let beneficiary = client.get_address_beneficiary("BTC", "bc1qtest", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_address_beneficiary(
        &self,
        currency: &str,
        address: &str,
        tag: Option<&str>,
    ) -> Result<crate::model::AddressBeneficiary, HttpError> {
        let mut url = format!(
            "{}{}?currency={}&address={}",
            self.base_url(),
            GET_ADDRESS_BENEFICIARY,
            urlencoding::encode(currency),
            urlencoding::encode(address)
        );

        if let Some(t) = tag {
            url.push_str(&format!("&tag={}", urlencoding::encode(t)));
        }

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get address beneficiary failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::AddressBeneficiary> = response
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
            HttpError::InvalidResponse("No beneficiary data in response".to_string())
        })
    }

    /// List address beneficiaries with filtering and pagination.
    ///
    /// Returns a paginated list of address beneficiaries with optional filters.
    ///
    /// # Arguments
    ///
    /// * `request` - Optional filtering and pagination parameters
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the response is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use deribit_http::DeribitHttpClient;
    /// use deribit_http::model::ListAddressBeneficiariesRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// let request = ListAddressBeneficiariesRequest {
    ///     currency: Some("BTC".to_string()),
    ///     limit: Some(10),
    ///     ..Default::default()
    /// };
    /// // let response = client.list_address_beneficiaries(Some(&request)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_address_beneficiaries(
        &self,
        request: Option<&crate::model::ListAddressBeneficiariesRequest>,
    ) -> Result<crate::model::ListAddressBeneficiariesResponse, HttpError> {
        let mut params: Vec<String> = Vec::new();

        if let Some(req) = request {
            if let Some(ref currency) = req.currency {
                params.push(format!("currency={}", urlencoding::encode(currency)));
            }
            if let Some(ref address) = req.address {
                params.push(format!("address={}", urlencoding::encode(address)));
            }
            if let Some(ref tag) = req.tag {
                params.push(format!("tag={}", urlencoding::encode(tag)));
            }
            if let Some(created_before) = req.created_before {
                params.push(format!("created_before={}", created_before));
            }
            if let Some(created_after) = req.created_after {
                params.push(format!("created_after={}", created_after));
            }
            if let Some(updated_before) = req.updated_before {
                params.push(format!("updated_before={}", updated_before));
            }
            if let Some(updated_after) = req.updated_after {
                params.push(format!("updated_after={}", updated_after));
            }
            if let Some(personal) = req.personal {
                params.push(format!("personal={}", personal));
            }
            if let Some(unhosted) = req.unhosted {
                params.push(format!("unhosted={}", unhosted));
            }
            if let Some(ref vasp_name) = req.beneficiary_vasp_name {
                params.push(format!(
                    "beneficiary_vasp_name={}",
                    urlencoding::encode(vasp_name)
                ));
            }
            if let Some(ref vasp_did) = req.beneficiary_vasp_did {
                params.push(format!(
                    "beneficiary_vasp_did={}",
                    urlencoding::encode(vasp_did)
                ));
            }
            if let Some(ref vasp_website) = req.beneficiary_vasp_website {
                params.push(format!(
                    "beneficiary_vasp_website={}",
                    urlencoding::encode(vasp_website)
                ));
            }
            if let Some(limit) = req.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(ref continuation) = req.continuation {
                params.push(format!(
                    "continuation={}",
                    urlencoding::encode(continuation)
                ));
            }
        }

        let url = if params.is_empty() {
            format!("{}{}", self.base_url(), LIST_ADDRESS_BENEFICIARIES)
        } else {
            format!(
                "{}{}?{}",
                self.base_url(),
                LIST_ADDRESS_BENEFICIARIES,
                params.join("&")
            )
        };

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "List address beneficiaries failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::ListAddressBeneficiariesResponse> = response
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
            HttpError::InvalidResponse("No beneficiaries data in response".to_string())
        })
    }

    /// Set clearance originator for a deposit.
    ///
    /// Sets the originator information for a deposit transaction,
    /// required for travel rule compliance.
    ///
    /// # Arguments
    ///
    /// * `deposit_id` - Identifier of the deposit
    /// * `originator` - Information about the originator
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if the request fails or the response is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use deribit_http::DeribitHttpClient;
    /// use deribit_http::model::{DepositId, Originator};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// let deposit_id = DepositId {
    ///     currency: "BTC".to_string(),
    ///     user_id: 123,
    ///     address: "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz".to_string(),
    ///     tx_hash: "230669110fdaf0a0dbcdc079b6b8b43d5af29cc73683835b9bc6b3406c065fda".to_string(),
    /// };
    /// let originator = Originator {
    ///     is_personal: false,
    ///     company_name: Some("Company Name".to_string()),
    ///     first_name: Some("First".to_string()),
    ///     last_name: Some("Last".to_string()),
    ///     address: "NL, Amsterdam, Street, 1".to_string(),
    /// };
    /// // let result = client.set_clearance_originator(&deposit_id, &originator).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_clearance_originator(
        &self,
        deposit_id: &crate::model::DepositId,
        originator: &crate::model::Originator,
    ) -> Result<crate::model::ClearanceDepositResult, HttpError> {
        let deposit_id_json = serde_json::to_string(deposit_id).map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to serialize deposit_id: {}", e))
        })?;
        let originator_json = serde_json::to_string(originator).map_err(|e| {
            HttpError::InvalidResponse(format!("Failed to serialize originator: {}", e))
        })?;

        let url = format!(
            "{}{}?deposit_id={}&originator={}",
            self.base_url(),
            SET_CLEARANCE_ORIGINATOR,
            urlencoding::encode(&deposit_id_json),
            urlencoding::encode(&originator_json)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Set clearance originator failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::ClearanceDepositResult> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No deposit result in response".to_string()))
    }

    /// Get account access log
    ///
    /// Retrieves the account access history showing login attempts and API access.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of entries to retrieve (optional, default 10)
    /// * `offset` - Offset for pagination (optional, default 0)
    ///
    pub async fn get_access_log(
        &self,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<crate::model::AccessLogResponse, HttpError> {
        let mut query_params = Vec::new();

        if let Some(count) = count {
            query_params.push(format!("count={}", count));
        }

        if let Some(offset) = offset {
            query_params.push(format!("offset={}", offset));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        let url = format!(
            "{}{}{}",
            self.base_url(),
            crate::constants::endpoints::GET_ACCESS_LOG,
            query_string
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get access log failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::AccessLogResponse> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No access log data in response".to_string()))
    }

    /// Get user account locks
    ///
    /// Retrieves information about any locks on the user's account.
    ///
    pub async fn get_user_locks(&self) -> Result<Vec<crate::model::UserLock>, HttpError> {
        let url = format!(
            "{}{}",
            self.base_url(),
            crate::constants::endpoints::GET_USER_LOCKS
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get user locks failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<crate::model::UserLock>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No user locks data in response".to_string()))
    }

    /// List custody accounts
    ///
    /// Retrieves the list of custody accounts for the specified currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    ///
    pub async fn list_custody_accounts(
        &self,
        currency: &str,
    ) -> Result<Vec<crate::model::CustodyAccount>, HttpError> {
        let url = format!(
            "{}{}?currency={}",
            self.base_url(),
            crate::constants::endpoints::LIST_CUSTODY_ACCOUNTS,
            urlencoding::encode(currency)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "List custody accounts failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<crate::model::CustodyAccount>> = response
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
            HttpError::InvalidResponse("No custody accounts data in response".to_string())
        })
    }

    /// Simulate portfolio margin
    ///
    /// Simulates portfolio margin for hypothetical positions.
    ///
    /// # Arguments
    ///
    /// * `request` - Simulation request parameters
    ///
    pub async fn simulate_portfolio(
        &self,
        request: crate::model::SimulatePortfolioRequest,
    ) -> Result<crate::model::SimulatePortfolioResponse, HttpError> {
        let mut query_params = vec![format!(
            "currency={}",
            urlencoding::encode(&request.currency)
        )];

        if let Some(add_positions) = request.add_positions {
            query_params.push(format!("add_positions={}", add_positions));
        }

        if let Some(ref positions) = request.simulated_positions {
            let positions_json = serde_json::to_string(positions)
                .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;
            query_params.push(format!(
                "simulated_positions={}",
                urlencoding::encode(&positions_json)
            ));
        }

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            crate::constants::endpoints::SIMULATE_PORTFOLIO,
            query_params.join("&")
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Simulate portfolio failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::SimulatePortfolioResponse> = response
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
            HttpError::InvalidResponse("No portfolio simulation data in response".to_string())
        })
    }

    /// PME margin simulation
    ///
    /// Simulates Portfolio Margin Engine (PME) margin for the specified currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    ///
    pub async fn pme_simulate(
        &self,
        currency: &str,
    ) -> Result<crate::model::PmeSimulateResponse, HttpError> {
        let url = format!(
            "{}{}?currency={}",
            self.base_url(),
            crate::constants::endpoints::PME_SIMULATE,
            urlencoding::encode(currency)
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "PME simulate failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::PmeSimulateResponse> = response
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
            HttpError::InvalidResponse("No PME simulation data in response".to_string())
        })
    }

    /// Change margin model
    ///
    /// Changes the margin model for the account or a specific user.
    ///
    /// # Arguments
    ///
    /// * `margin_model` - The new margin model to set
    /// * `user_id` - Optional user ID (for main account operating on subaccounts)
    /// * `dry_run` - Optional flag to simulate the change without applying it
    ///
    pub async fn change_margin_model(
        &self,
        margin_model: crate::model::MarginModel,
        user_id: Option<u64>,
        dry_run: Option<bool>,
    ) -> Result<crate::model::ChangeMarginModelResponse, HttpError> {
        let mut query_params = vec![format!("margin_model={}", margin_model.as_str())];

        if let Some(user_id) = user_id {
            query_params.push(format!("user_id={}", user_id));
        }

        if let Some(dry_run) = dry_run {
            query_params.push(format!("dry_run={}", dry_run));
        }

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            crate::constants::endpoints::CHANGE_MARGIN_MODEL,
            query_params.join("&")
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Change margin model failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::ChangeMarginModelResponse> = response
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
            HttpError::InvalidResponse("No margin model change data in response".to_string())
        })
    }

    /// Set self-trading configuration
    ///
    /// Configures self-trading prevention settings for the account.
    ///
    /// # Arguments
    ///
    /// * `mode` - Self-trading prevention mode
    /// * `extended_to_subaccounts` - Whether to extend the config to subaccounts
    /// * `block_rfq_self_match_prevention` - Optional RFQ self-match prevention setting
    ///
    pub async fn set_self_trading_config(
        &self,
        mode: crate::model::SelfTradingMode,
        extended_to_subaccounts: bool,
        block_rfq_self_match_prevention: Option<bool>,
    ) -> Result<bool, HttpError> {
        let mut query_params = vec![
            format!("mode={}", mode.as_str()),
            format!("extended_to_subaccounts={}", extended_to_subaccounts),
        ];

        if let Some(block_rfq) = block_rfq_self_match_prevention {
            query_params.push(format!("block_rfq_self_match_prevention={}", block_rfq));
        }

        let url = format!(
            "{}{}?{}",
            self.base_url(),
            crate::constants::endpoints::SET_SELF_TRADING_CONFIG,
            query_params.join("&")
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Set self trading config failed: {}",
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

        Ok(api_response.result.map(|s| s == "ok").unwrap_or(true))
    }

    /// Set disabled trading products
    ///
    /// Disables specific trading products for a user.
    ///
    /// # Arguments
    ///
    /// * `trading_products` - List of trading products to disable
    /// * `user_id` - User ID to apply the setting to
    ///
    pub async fn set_disabled_trading_products(
        &self,
        trading_products: &[crate::model::TradingProduct],
        user_id: u64,
    ) -> Result<bool, HttpError> {
        let products: Vec<&str> = trading_products.iter().map(|p| p.as_str()).collect();
        let products_json = serde_json::to_string(&products)
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        let url = format!(
            "{}{}?trading_products={}&user_id={}",
            self.base_url(),
            crate::constants::endpoints::SET_DISABLED_TRADING_PRODUCTS,
            urlencoding::encode(&products_json),
            user_id
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Set disabled trading products failed: {}",
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

        Ok(api_response.result.map(|s| s == "ok").unwrap_or(true))
    }

    /// Get new (unread) announcements
    ///
    /// Retrieves announcements that have not been marked as read.
    ///
    pub async fn get_new_announcements(
        &self,
    ) -> Result<Vec<crate::model::Announcement>, HttpError> {
        let url = format!(
            "{}{}",
            self.base_url(),
            crate::constants::endpoints::GET_NEW_ANNOUNCEMENTS
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get new announcements failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<crate::model::Announcement>> = response
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
            HttpError::InvalidResponse("No new announcements data in response".to_string())
        })
    }

    /// Mark announcement as read
    ///
    /// Marks a specific announcement as read so it won't appear in new announcements.
    ///
    /// # Arguments
    ///
    /// * `announcement_id` - ID of the announcement to mark as read
    ///
    pub async fn set_announcement_as_read(&self, announcement_id: u64) -> Result<bool, HttpError> {
        let url = format!(
            "{}{}?announcement_id={}",
            self.base_url(),
            crate::constants::endpoints::SET_ANNOUNCEMENT_AS_READ,
            announcement_id
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Set announcement as read failed: {}",
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

        Ok(api_response.result.map(|s| s == "ok").unwrap_or(true))
    }

    /// Enable affiliate program
    ///
    /// Enables the affiliate program for the user's account.
    ///
    pub async fn enable_affiliate_program(&self) -> Result<bool, HttpError> {
        let url = format!(
            "{}{}",
            self.base_url(),
            crate::constants::endpoints::ENABLE_AFFILIATE_PROGRAM
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Enable affiliate program failed: {}",
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

        Ok(api_response.result.map(|s| s == "ok").unwrap_or(true))
    }

    /// Get affiliate program information
    ///
    /// Retrieves information about the user's affiliate program status.
    ///
    pub async fn get_affiliate_program_info(
        &self,
    ) -> Result<crate::model::AffiliateProgramInfo, HttpError> {
        let url = format!(
            "{}{}",
            self.base_url(),
            crate::constants::endpoints::GET_AFFILIATE_PROGRAM_INFO
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get affiliate program info failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<crate::model::AffiliateProgramInfo> =
            response
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
            HttpError::InvalidResponse("No affiliate program info in response".to_string())
        })
    }

    /// Set email language preference
    ///
    /// Sets the preferred language for email communications.
    ///
    /// # Arguments
    ///
    /// * `language` - The language to set for emails
    ///
    pub async fn set_email_language(
        &self,
        language: crate::model::EmailLanguage,
    ) -> Result<bool, HttpError> {
        let url = format!(
            "{}{}?language={}",
            self.base_url(),
            crate::constants::endpoints::SET_EMAIL_LANGUAGE,
            language.as_str()
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Set email language failed: {}",
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

        Ok(api_response.result.map(|s| s == "ok").unwrap_or(true))
    }

    /// Get email language preference
    ///
    /// Retrieves the current email language preference.
    ///
    pub async fn get_email_language(&self) -> Result<String, HttpError> {
        let url = format!(
            "{}{}",
            self.base_url(),
            crate::constants::endpoints::GET_EMAIL_LANGUAGE
        );

        let response = self.make_authenticated_request(&url).await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get email language failed: {}",
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
            .ok_or_else(|| HttpError::InvalidResponse("No email language in response".to_string()))
    }
}
