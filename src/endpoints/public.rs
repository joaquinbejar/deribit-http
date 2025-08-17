//! REST API endpoints implementation
//!
//! This module implements all Deribit REST API endpoints including
//! market data, trading, account management, and system endpoints.

use crate::DeribitHttpClient;
use crate::error::HttpError;
use crate::model::http_types::ApiResponse;
use serde::{Deserialize, Serialize};

/// Market data endpoints
impl DeribitHttpClient {
    /// Get all supported currencies
    ///
    /// Retrieves all cryptocurrencies supported by the API.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let currencies = client.get_currencies().await?;
    /// for currency in currencies {
    ///     println!("Currency: {} ({})", currency.currency, currency.currency_long);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_currencies(&self) -> Result<Vec<Currency>, HttpError> {
        let url = format!("{}/public/get_currencies", self.base_url());

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
                "Get currencies failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<Currency>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No currencies in response".to_string()))
    }

    /// Get current index price for a currency
    ///
    /// Retrieves the current index price for the instruments, for the selected currency.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let index_data = client.get_index("BTC").await?;
    /// println!("Estimated delivery price: {}", index_data.edp);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index(&self, currency: &str) -> Result<IndexData, HttpError> {
        let url = format!(
            "{}/public/get_index?currency={}",
            self.base_url(),
            currency
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
                "Get index failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<IndexData> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No index data in response".to_string()))
    }

    /// Get index price by name
    ///
    /// Retrieves the current index price value for given index name.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `index_name` - The index identifier (e.g., "btc_usd", "eth_usd")
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let index_price = client.get_index_price("btc_usd").await?;
    /// println!("Index price: {}", index_price.index_price);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_price(&self, index_name: &str) -> Result<IndexPriceData, HttpError> {
        let url = format!(
            "{}/public/get_index_price?index_name={}",
            self.base_url(),
            index_name
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
                "Get index price failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<IndexPriceData> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No index price data in response".to_string()))
    }

    /// Get all supported index price names
    ///
    /// Retrieves the identifiers of all supported Price Indexes.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let index_names = client.get_index_price_names().await?;
    /// for name in index_names {
    ///     println!("Index: {}", name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_price_names(&self) -> Result<Vec<String>, HttpError> {
        let url = format!("{}/public/get_index_price_names", self.base_url());

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
                "Get index price names failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<String>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No index price names in response".to_string()))
    }

    /// Get book summary by currency
    ///
    /// Retrieves the summary information such as open interest, 24h volume, etc. 
    /// for all instruments for the currency (optionally filtered by kind).
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `kind` - Optional instrument kind filter (future, option, spot, future_combo, option_combo)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let summaries = client.get_book_summary_by_currency("BTC", Some("future")).await?;
    /// for summary in summaries {
    ///     println!("Instrument: {} - Volume: {}", summary.instrument_name, summary.volume);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_book_summary_by_currency(
        &self,
        currency: &str,
        kind: Option<&str>,
    ) -> Result<Vec<BookSummary>, HttpError> {
        let mut url = format!(
            "{}/public/get_book_summary_by_currency?currency={}",
            self.base_url(),
            currency
        );

        if let Some(kind) = kind {
            url.push_str(&format!("&kind={}", kind));
        }

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
                "Get book summary by currency failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<BookSummary>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No book summary data in response".to_string()))
    }

    /// Get single instrument information
    ///
    /// Retrieves detailed information about a specific instrument.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier (e.g., "BTC-PERPETUAL")
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let instrument = client.get_instrument("BTC-PERPETUAL").await?;
    /// println!("Contract size: {}", instrument.contract_size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_instrument(&self, instrument_name: &str) -> Result<Instrument, HttpError> {
        let url = format!(
            "{}/public/get_instrument?instrument_name={}",
            self.base_url(),
            instrument_name
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
                "Get instrument failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Instrument> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No instrument data in response".to_string()))
    }

    /// Get book summary by instrument
    ///
    /// Retrieves the summary information such as open interest, 24h volume, etc. 
    /// for a specific instrument.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier (e.g., "BTC-PERPETUAL")
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let summary = client.get_book_summary_by_instrument("BTC-PERPETUAL").await?;
    /// println!("Volume: {} USD", summary.volume_usd.unwrap_or(0.0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_book_summary_by_instrument(&self, instrument_name: &str) -> Result<BookSummary, HttpError> {
        let url = format!(
            "{}/public/get_book_summary_by_instrument?instrument_name={}",
            self.base_url(),
            instrument_name
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
                "Get book summary by instrument failed: {}",
                error_text
            )));
        }

        // The API returns an array with one element, so we need to parse it as Vec<BookSummary>
        let api_response: ApiResponse<Vec<BookSummary>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        let book_summaries = api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No book summary data in response".to_string()))?;

        // Return the first (and typically only) element
        book_summaries
            .into_iter()
            .next()
            .ok_or_else(|| HttpError::InvalidResponse("Empty book summary array in response".to_string()))
    }

    /// Get contract size for an instrument
    ///
    /// Retrieves contract size for specified instrument.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier (e.g., "BTC-PERPETUAL")
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let contract_size = client.get_contract_size("BTC-PERPETUAL").await?;
    /// println!("Contract size: {}", contract_size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_contract_size(&self, instrument_name: &str) -> Result<f64, HttpError> {
        let url = format!(
            "{}/public/get_contract_size?instrument_name={}",
            self.base_url(),
            instrument_name
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
                "Get contract size failed: {}",
                error_text
            )));
        }

        // The API returns an object with contract_size field
        let api_response: ApiResponse<ContractSizeResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        let contract_size_response = api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No contract size in response".to_string()))?;

        Ok(contract_size_response.contract_size)
    }

    /// Get server time
    ///
    /// Returns the current server time in milliseconds since Unix epoch.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let server_time = client.get_server_time().await?;
    /// println!("Server time: {}", server_time);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_server_time(&self) -> Result<u64, HttpError> {
        let url = format!("{}/public/get_time", self.base_url());

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
                "Get server time failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<u64> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No server time in response".to_string()))
    }

    /// Test connectivity to the API
    ///
    /// Returns the API version to test connectivity.
    /// This is a public endpoint that doesn't require authentication.
    pub async fn test_connection(&self) -> Result<String, HttpError> {
        let url = format!("{}/public/test", self.base_url());

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
                "Test connection failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TestResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        let test_result = api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No test result in response".to_string()))?;

        Ok(test_result.version)
    }

    /// Hello endpoint for WebSocket client introduction
    ///
    /// **Note**: This endpoint is only available via WebSocket connections according 
    /// to the Deribit API documentation. This HTTP client cannot access this endpoint.
    /// The method is provided for completeness but will always return an error.
    ///
    /// # Arguments
    ///
    /// * `client_name` - Client software name
    /// * `client_version` - Client software version
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// // This will return an error since hello is WebSocket-only
    /// match client.hello("my-client", "1.0.0").await {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => println!("Expected error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn hello(&self, _client_name: &str, _client_version: &str) -> Result<HelloResponse, HttpError> {
        Err(HttpError::ConfigError(
            "The /public/hello endpoint is only available via WebSocket connections. \
             This endpoint cannot be accessed via HTTP. Please use the Deribit WebSocket API \
             for client introduction functionality.".to_string()
        ))
    }

    /// Get platform status and locked currency indices
    ///
    /// Returns information about the platform status and any locked currency indices.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let status = client.get_status().await?;
    /// println!("Platform locked: {}", status.locked);
    /// for index in &status.locked_indices {
    ///     println!("Locked currency index: {}", index);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_status(&self) -> Result<StatusResponse, HttpError> {
        let url = format!("{}/public/status", self.base_url());

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
                "Get status failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<StatusResponse> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No status data in response".to_string()))
    }

    /// Get APR history for yield tokens
    ///
    /// Retrieves historical APR data for specified currency. Only applicable to yield-generating tokens (USDE, STETH).
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency for which to retrieve APR history (usde or steth)
    /// * `limit` - Optional number of days to retrieve (default 365, maximum 365)
    /// * `before` - Optional parameter to receive APR history before given epoch day
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let apr_history = client.get_apr_history("steth", Some(10), None).await?;
    /// for data_point in &apr_history.data {
    ///     println!("Day {}: APR {}%", data_point.day, data_point.apr);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_apr_history(
        &self,
        currency: &str,
        limit: Option<u32>,
        before: Option<i32>,
    ) -> Result<AprHistoryResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_apr_history?currency={}",
            self.base_url(),
            currency
        );

        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }

        if let Some(before) = before {
            url.push_str(&format!("&before={}", before));
        }

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
                "Get APR history failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<AprHistoryResponse> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No APR history data in response".to_string()))
    }

    /// Get ticker information for an instrument
    ///
    /// Returns ticker data including last price, bid/ask, volume, etc.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier (e.g., "BTC-PERPETUAL")
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true);
    /// let ticker = client.get_ticker("BTC-PERPETUAL").await?;
    /// println!("Last price: {}", ticker.last_price);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ticker(&self, instrument_name: &str) -> Result<TickerData, HttpError> {
        let url = format!(
            "{}/public/ticker?instrument_name={}",
            self.base_url(),
            instrument_name
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
                "Get ticker failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TickerData> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No ticker data in response".to_string()))
    }

    /// Get order book for an instrument
    ///
    /// Returns the current order book with bids and asks.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier
    /// * `depth` - Optional depth of the order book (default: 5)
    pub async fn get_order_book(
        &self,
        instrument_name: &str,
        depth: Option<u32>,
    ) -> Result<OrderBook, HttpError> {
        let mut url = format!(
            "{}/public/get_order_book?instrument_name={}",
            self.base_url(),
            instrument_name
        );

        if let Some(d) = depth {
            url.push_str(&format!("&depth={}", d));
        }

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
                "Get order book failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderBook> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No order book data in response".to_string()))
    }

    /// Get available instruments
    ///
    /// Returns a list of available trading instruments.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC", "ETH")
    /// * `kind` - Optional instrument kind ("future", "option", "spot")
    /// * `expired` - Whether to include expired instruments
    pub async fn get_instruments(
        &self,
        currency: &str,
        kind: Option<&str>,
        expired: Option<bool>,
    ) -> Result<Vec<Instrument>, HttpError> {
        let mut url = format!(
            "{}/public/get_instruments?currency={}",
            self.base_url(),
            currency
        );

        if let Some(k) = kind {
            url.push_str(&format!("&kind={}", k));
        }

        if let Some(exp) = expired {
            url.push_str(&format!("&expired={}", exp));
        }

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
                "Get instruments failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<Instrument>> = response
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
            HttpError::InvalidResponse("No instruments data in response".to_string())
        })
    }

    /// Get recent trades for an instrument
    ///
    /// Returns recent trade history for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier
    /// * `count` - Optional number of trades to return (default: 10, max: 1000)
    /// * `include_old` - Whether to include old trades
    pub async fn get_last_trades(
        &self,
        instrument_name: &str,
        count: Option<u32>,
        include_old: Option<bool>,
    ) -> Result<Vec<Trade>, HttpError> {
        let mut url = format!(
            "{}/public/get_last_trades_by_instrument?instrument_name={}",
            self.base_url(),
            instrument_name
        );

        if let Some(c) = count {
            url.push_str(&format!("&count={}", c));
        }

        if let Some(old) = include_old {
            url.push_str(&format!("&include_old={}", old));
        }

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
                "Get last trades failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<Trade>> = response
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
            .ok_or_else(|| HttpError::InvalidResponse("No trades data in response".to_string()))
    }

    /// Get historical volatility
    ///
    /// Provides information about historical volatility for given cryptocurrency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (BTC, ETH, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let volatility = client.get_historical_volatility("BTC").await?;
    /// // tracing::info!("Found {} volatility data points", volatility.len());
    /// ```
    pub async fn get_historical_volatility(
        &self,
        currency: &str,
    ) -> Result<Vec<[f64; 2]>, HttpError> {
        let url = format!(
            "{}/public/get_historical_volatility?currency={}",
            self.base_url(),
            urlencoding::encode(currency)
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
                "Get historical volatility failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<[f64; 2]>> = response
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
            HttpError::InvalidResponse("No historical volatility data in response".to_string())
        })
    }

    /// Get funding chart data
    ///
    /// Retrieves the list of the latest PERPETUAL funding chart points within a given time period.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `length` - Time period (8h, 24h, 1m)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let funding_data = client.get_funding_chart_data("BTC-PERPETUAL", "8h").await?;
    /// // tracing::info!("Current interest: {}", funding_data.current_interest);
    /// ```
    pub async fn get_funding_chart_data(
        &self,
        instrument_name: &str,
        length: &str,
    ) -> Result<FundingChartData, HttpError> {
        let url = format!(
            "{}/public/get_funding_chart_data?instrument_name={}&length={}",
            self.base_url(),
            urlencoding::encode(instrument_name),
            urlencoding::encode(length)
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
                "Get funding chart data failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<FundingChartData> = response
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
            HttpError::InvalidResponse("No funding chart data in response".to_string())
        })
    }

    /// Get TradingView chart data
    ///
    /// Publicly available market data used to generate a TradingView candle chart.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `start_timestamp` - Start timestamp in milliseconds
    /// * `end_timestamp` - End timestamp in milliseconds
    /// * `resolution` - Chart resolution (1, 3, 5, 10, 15, 30, 60, 120, 180, 360)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    ///
    /// let client = DeribitHttpClient::new(true);
    /// // let chart_data = client.get_tradingview_chart_data("BTC-PERPETUAL", 1554373800000, 1554376800000, "30").await?;
    /// // tracing::info!("Chart status: {}", chart_data.status);
    /// ```
    pub async fn get_tradingview_chart_data(
        &self,
        instrument_name: &str,
        start_timestamp: u64,
        end_timestamp: u64,
        resolution: &str,
    ) -> Result<TradingViewChartData, HttpError> {
        let url = format!(
            "{}/public/get_tradingview_chart_data?instrument_name={}&start_timestamp={}&end_timestamp={}&resolution={}",
            self.base_url(),
            urlencoding::encode(instrument_name),
            start_timestamp,
            end_timestamp,
            urlencoding::encode(resolution)
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
                "Get TradingView chart data failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TradingViewChartData> = response
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
            HttpError::InvalidResponse("No TradingView chart data in response".to_string())
        })
    }
}

/// Public endpoint data structures

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

/// Index price data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct IndexPriceData {
    /// Estimated delivery price for the market
    pub estimated_delivery_price: f64,
    /// Value of requested index
    pub index_price: f64,
}

/// Book summary structure
#[derive(Clone, Serialize, Deserialize)]
pub struct BookSummary {
    /// Volume in USD
    pub volume_usd: Option<f64>,
    /// Volume in base currency
    pub volume: f64,
    /// Quote currency
    pub quote_currency: String,
    /// Price change percentage
    pub price_change: Option<f64>,
    /// Open interest
    pub open_interest: f64,
    /// Mid price
    pub mid_price: Option<f64>,
    /// Mark price
    pub mark_price: f64,
    /// Mark implied volatility (for options)
    pub mark_iv: Option<f64>,
    /// 24h low price
    pub low: Option<f64>,
    /// Last trade price
    pub last: Option<f64>,
    /// Instrument name
    pub instrument_name: String,
    /// 24h high price
    pub high: Option<f64>,
    /// Funding rate for 8h (for perpetuals)
    pub funding_8h: Option<f64>,
    /// Estimated delivery price
    pub estimated_delivery_price: f64,
    /// Current funding rate (for perpetuals)
    pub current_funding: Option<f64>,
    /// Creation timestamp
    pub creation_timestamp: u64,
    /// Best bid price
    pub bid_price: Option<f64>,
    /// Base currency
    pub base_currency: String,
    /// Best ask price
    pub ask_price: Option<f64>,
}

/// Contract size response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct ContractSizeResponse {
    /// Contract size value
    pub contract_size: f64,
}

/// Test response structure for connectivity test
#[derive(Clone, Serialize, Deserialize)]
pub struct TestResponse {
    /// API version
    pub version: String,
}

/// Hello response structure for WebSocket client introduction
#[derive(Clone, Serialize, Deserialize)]
pub struct HelloResponse {
    /// API version
    pub version: String,
}

/// Status response structure for platform status
#[derive(Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// List of locked currency indices
    pub locked_indices: Vec<String>,
    /// Platform lock status: "true", "false", or "partial"
    pub locked: String,
}

/// APR history response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct AprHistoryResponse {
    /// Array of APR data points
    pub data: Vec<AprDataPoint>,
    /// Continuation token for pagination
    pub continuation: Option<i32>,
}

/// APR data point structure
#[derive(Clone, Serialize, Deserialize)]
pub struct AprDataPoint {
    /// The full epoch day
    pub day: i32,
    /// The APR of the day
    pub apr: f64,
}

/// Ticker data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Best ask price
    pub best_ask_price: Option<f64>,
    /// Best ask amount
    pub best_ask_amount: Option<f64>,
    /// Best bid price
    pub best_bid_price: Option<f64>,
    /// Best bid amount
    pub best_bid_amount: Option<f64>,
    /// Last trade price
    pub last_price: Option<f64>,
    /// Mark price
    pub mark_price: f64,
    /// Open interest
    pub open_interest: f64,
    /// 24h statistics
    pub stats: TickerStats,
}

/// Ticker statistics structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerStats {
    /// Volume in base currency
    pub volume: f64,
    /// Price change percentage
    pub price_change: Option<f64>,
    /// 24h low price
    pub low: Option<f64>,
    /// 24h high price
    pub high: Option<f64>,
}

/// Order book structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Array of asks [price, amount]
    pub asks: Vec<[f64; 2]>,
    /// Array of bids [price, amount]
    pub bids: Vec<[f64; 2]>,
    /// Instrument name
    pub instrument_name: String,
}

/// Instrument structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument name
    pub instrument_name: String,
    /// Contract size
    pub contract_size: i32,
    /// Creation timestamp
    pub creation_timestamp: u64,
    /// Expiration timestamp
    pub expiration_timestamp: Option<u64>,
}

/// Trade structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade amount
    pub amount: f64,
    /// Trade direction
    pub direction: String,
    /// Trade price
    pub price: f64,
    /// Trade timestamp
    pub timestamp: u64,
}

/// Funding chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingChartData {
    /// Current interest rate
    pub current_interest: f64,
    /// Data points
    pub data: Vec<FundingDataPoint>,
    /// Interest 8h
    pub interest_8h: f64,
}

/// Funding data point structure
#[derive(Clone, Serialize, Deserialize)]
pub struct FundingDataPoint {
    /// Index price
    pub index_price: f64,
    /// Interest rate
    pub interest_1h: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// TradingView chart data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TradingViewChartData {
    /// Chart status
    pub status: String,
    /// Array of close prices
    pub close: Vec<f64>,
    /// Array of high prices
    pub high: Vec<f64>,
    /// Array of low prices
    pub low: Vec<f64>,
    /// Array of open prices
    pub open: Vec<f64>,
    /// Array of volumes
    pub volume: Vec<f64>,
    /// Array of timestamps
    pub ticks: Vec<u64>,
}

// Implement Display and Debug traits using macros from deribit-base
deribit_base::impl_json_display!(
    Currency, IndexData, WithdrawalPriority, IndexPriceData, BookSummary, 
    ContractSizeResponse, TestResponse, HelloResponse, StatusResponse,
    AprHistoryResponse, AprDataPoint, TickerData, TickerStats, OrderBook,
    Instrument, Trade, FundingChartData, FundingDataPoint, TradingViewChartData
);

deribit_base::impl_json_debug_pretty!(
    Currency, IndexData, WithdrawalPriority, IndexPriceData, BookSummary, 
    ContractSizeResponse, TestResponse, HelloResponse, StatusResponse,
    AprHistoryResponse, AprDataPoint, TickerData, TickerStats, OrderBook,
    Instrument, Trade, FundingChartData, FundingDataPoint, TradingViewChartData
);
