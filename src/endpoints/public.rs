//! REST API endpoints implementation
//!
//! This module implements all Deribit REST API endpoints including
//! market data, trading, account management, and system endpoints.

use crate::DeribitHttpClient;
use crate::error::HttpError;
use crate::model::LastTradesResponse;
use crate::model::book::{BookSummary, OrderBook};
use crate::model::currency::Currency;
use crate::model::funding::{FundingChartData, FundingRateData};
use crate::model::index::{IndexData, IndexPriceData};
use crate::model::instrument::{Instrument, OptionType};
use crate::model::order::OrderSide;
use crate::model::other::{OptionInstrument, OptionInstrumentPair};
use crate::model::response::api_response::ApiResponse;
use crate::model::response::other::{
    AprHistoryResponse, ContractSizeResponse, DeliveryPricesResponse, ExpirationsResponse,
    SettlementsResponse, StatusResponse, TestResponse,
};
use crate::model::ticker::TickerData;
use crate::model::trade::{Liquidity, Trade};
use crate::model::tradingview::TradingViewChartData;
use std::collections::HashMap;

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
    /// let client = DeribitHttpClient::new(); // testnet
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
            .get(url)
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
    pub async fn get_index(&self, currency: &str) -> Result<IndexData, HttpError> {
        let url = format!("{}/public/get_index?currency={}", self.base_url(), currency);

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
    /// let client = DeribitHttpClient::new(); // testnet
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

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No index price data in response".to_string())
        })
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
    /// let client = DeribitHttpClient::new(); // testnet
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

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No index price names in response".to_string())
        })
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
    /// let client = DeribitHttpClient::new(); // testnet
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

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No book summary data in response".to_string())
        })
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
    /// let client = DeribitHttpClient::new(); // testnet
    /// let instrument = client.get_instrument("BTC-PERPETUAL").await?;
    /// println!("Contract size: {:?}", instrument.contract_size);
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
    pub async fn get_book_summary_by_instrument(
        &self,
        instrument_name: &str,
    ) -> Result<BookSummary, HttpError> {
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

        let book_summaries = api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No book summary data in response".to_string())
        })?;

        // Return the first (and typically only) element
        book_summaries.into_iter().next().ok_or_else(|| {
            HttpError::InvalidResponse("Empty book summary array in response".to_string())
        })
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
    /// let client = DeribitHttpClient::new(); // testnet
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

        let contract_size_response = api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No contract size in response".to_string())
        })?;

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
    /// let client = DeribitHttpClient::new(); // testnet
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

    /// Get platform status and locked currency indices
    ///
    /// Returns information about the platform status and any locked currency indices.
    /// This is a public endpoint that doesn't require authentication.
    ///
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

        // Try direct deserialization first (non-JSON-RPC response)
        match response.json::<StatusResponse>().await {
            Ok(status) => Ok(status),
            Err(_) => {
                // Fallback to JSON-RPC wrapper format
                let response = self
                    .http_client()
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| HttpError::NetworkError(e.to_string()))?;

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

                api_response.result.ok_or_else(|| {
                    HttpError::InvalidResponse("No status data in response".to_string())
                })
            }
        }
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

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No APR history data in response".to_string())
        })
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
    /// let client = DeribitHttpClient::new();
    /// let ticker = client.get_ticker("BTC-PERPETUAL").await?;
    /// println!("Last price: {:?}", ticker.last_price);
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

    /// Retrieves a list of option instruments for a given currency and expiry date.
    ///
    /// This asynchronous function fetches option instruments for the specified `currency`
    /// and `expiry` date and returns a filtered list of options along with their associated
    /// ticker information.
    ///
    /// # Arguments
    ///
    /// * `currency` - A string slice that represents the name of the currency (e.g., "BTC", "ETH").
    /// * `expiry` - A string slice representing the expiry date for the options (e.g., "20231027").
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `OptionInstrument` on success,
    /// or an `HttpError` on failure.
    ///
    /// - On success, it returns a `Vec<OptionInstrument>`, where each option contains
    ///   the instrument details and ticker information.
    /// - On failure, it returns an `HttpError`, such as in cases where the instrument
    ///   data could not be retrieved or tickers are inaccessible.
    ///
    /// # Errors
    ///
    /// This function may return an `HttpError` in the following scenarios:
    ///
    /// * If fetching the instrument data fails.
    /// * If retrieving ticker information for an instrument fails.
    ///
    /// # Implementation Details
    ///
    /// 1. Fetches instruments for the specified `currency` filtered by type `option`.
    /// 2. Filters the instruments to ensure they match the `currency`-`expiry` base name.
    /// 3. Constructs an `OptionInstrument` for each filtered instrument, including
    ///    the instrument details and ticker information.
    ///
    pub async fn get_options(
        &self,
        currency: &str,
        expiry: &str,
    ) -> Result<Vec<OptionInstrument>, HttpError> {
        let mut instruments = self
            .get_instruments(currency, Some("option"), Some(false))
            .await
            .map_err(|e| HttpError::RequestFailed(e.to_string()))?;

        let base_name = format!("{}-{}", currency, expiry).to_uppercase();
        // filter instruments by base name in instrument_name
        instruments.retain(|i| i.instrument_name.starts_with(&base_name));

        let mut options: Vec<OptionInstrument> = Vec::with_capacity(instruments.len());
        for instrument in instruments {
            let option = OptionInstrument {
                instrument: instrument.clone(),
                ticker: self.get_ticker(instrument.instrument_name.as_str()).await?,
            };
            options.push(option)
        }
        Ok(options)
    }

    /// Fetches option instruments for a given currency and expiry date, grouped by strike price.
    ///
    /// This method retrieves all option instruments for the specified currency and expiry,
    /// then groups them into pairs (call and put) by strike price. Each strike price
    /// maps to an `OptionInstrumentPair` containing the call and put options if available.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (e.g., "BTC", "ETH")
    /// * `expiry` - The expiry date in format "DDMMMYY" (e.g., "10SEP25")
    ///
    /// # Returns
    ///
    /// Returns a `HashMap` where:
    /// - Key: Strike price as `u64`
    /// - Value: `OptionInstrumentPair` containing call and put options for that strike
    ///
    /// # Errors
    ///
    /// Returns `HttpError` if:
    /// - The API request fails
    /// - An option instrument has no option type
    /// - Network or authentication errors occur
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new();
    /// let pairs = client.get_options_pair("BTC", "10SEP25").await?;
    ///
    /// for (strike, pair) in pairs {
    ///     println!("Strike {}: Call={:?}, Put={:?}",
    ///              strike, pair.call.is_some(), pair.put.is_some());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_options_pair(
        &self,
        currency: &str,
        expiry: &str,
    ) -> Result<HashMap<u64, OptionInstrumentPair>, HttpError> {
        let option_instruments = self.get_options(currency, expiry).await?;

        let mut strikes_map: HashMap<u64, OptionInstrumentPair> =
            HashMap::with_capacity(option_instruments.len() / 2);
        for instrument in option_instruments {
            let strike_price = instrument.instrument.strike.unwrap() as u64;
            strikes_map
                .entry(strike_price)
                .or_insert(OptionInstrumentPair {
                    call: None,
                    put: None,
                });
            match instrument.instrument.option_type.clone() {
                Some(option_type) => match option_type {
                    OptionType::Call => {
                        strikes_map.get_mut(&strike_price).unwrap().call = Some(instrument.clone());
                    }
                    OptionType::Put => {
                        strikes_map.get_mut(&strike_price).unwrap().put = Some(instrument.clone());
                    }
                },
                None => {
                    return Err(HttpError::RequestFailed(
                        "Option instrument has no option type".to_string(),
                    ));
                }
            }
        }

        Ok(strikes_map)
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
            urlencoding::encode(instrument_name)
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

        let api_response: ApiResponse<LastTradesResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        let trades_response = api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No trades data in response".to_string()))?;

        // Convert LastTrade to Trade
        let trades: Vec<Trade> = trades_response
            .trades
            .into_iter()
            .map(|last_trade| {
                Trade {
                    trade_id: last_trade.trade_id,
                    instrument_name: last_trade.instrument_name,
                    order_id: String::new(), // Not available in LastTrade
                    direction: match last_trade.direction.as_str() {
                        "buy" => OrderSide::Buy,
                        "sell" => OrderSide::Sell,
                        _ => OrderSide::Buy, // Default fallback
                    },
                    amount: last_trade.amount,
                    price: last_trade.price,
                    timestamp: last_trade.timestamp as i64,
                    fee: 0.0,                    // Not available in LastTrade
                    fee_currency: String::new(), // Not available in LastTrade
                    liquidity: Liquidity::Taker, // Default
                    mark_price: 0.0,             // Not available in LastTrade
                    index_price: last_trade.index_price,
                    instrument_kind: None, // Not available in LastTrade
                    trade_seq: Some(last_trade.trade_seq),
                    user_role: None,
                    block_trade: None,
                    underlying_price: None,
                    iv: last_trade.iv,
                    label: None,
                    profit_loss: None,
                    tick_direction: Some(last_trade.tick_direction),
                    self_trade: None,
                }
            })
            .collect();

        Ok(trades)
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
    /// let client = DeribitHttpClient::new();
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
    /// let client = DeribitHttpClient::new();
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
    /// let client = DeribitHttpClient::new();
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

    /// Get delivery prices
    ///
    /// Retrieves delivery prices for the given index.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `index_name` - Index identifier (e.g., "btc_usd", "eth_usd")
    /// * `count` - Number of requested items (optional, default 20)
    /// * `offset` - Offset for pagination (optional, default 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(); // testnet
    /// let delivery_prices = client.get_delivery_prices("btc_usd", Some(5), Some(0)).await?;
    /// for price in delivery_prices.data {
    ///     println!("Date: {} - Price: {}", price.date, price.delivery_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_delivery_prices(
        &self,
        index_name: &str,
        count: Option<u32>,
        offset: Option<u32>,
    ) -> Result<DeliveryPricesResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_delivery_prices?index_name={}",
            self.base_url(),
            urlencoding::encode(index_name)
        );

        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        if let Some(offset) = offset {
            url.push_str(&format!("&offset={}", offset));
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
                "Get delivery prices failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<DeliveryPricesResponse> = response
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
            HttpError::InvalidResponse("No delivery prices data in response".to_string())
        })
    }

    /// Get expirations
    ///
    /// Retrieves expirations for instruments. This method can be used to see instrument expirations.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, any, grouped)
    /// * `kind` - Instrument kind (future, option, any)
    /// * `currency_pair` - Currency pair identifier (optional)
    ///
    pub async fn get_expirations(
        &self,
        currency: &str,
        kind: &str,
        currency_pair: Option<&str>,
    ) -> Result<ExpirationsResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_expirations?currency={}&kind={}",
            self.base_url(),
            urlencoding::encode(currency),
            urlencoding::encode(kind)
        );

        if let Some(currency_pair) = currency_pair {
            url.push_str(&format!(
                "&currency_pair={}",
                urlencoding::encode(currency_pair)
            ));
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
                "Get expirations failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<ExpirationsResponse> = response
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
            HttpError::InvalidResponse("No expirations data in response".to_string())
        })
    }

    /// Get funding rate history
    ///
    /// Retrieves hourly historical interest rate for requested PERPETUAL instrument.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `start_timestamp` - The earliest timestamp to return result from (milliseconds since UNIX epoch)
    /// * `end_timestamp` - The most recent timestamp to return result from (milliseconds since UNIX epoch)
    ///
    pub async fn get_funding_rate_history(
        &self,
        instrument_name: &str,
        start_timestamp: u64,
        end_timestamp: u64,
    ) -> Result<Vec<FundingRateData>, HttpError> {
        let url = format!(
            "{}/public/get_funding_rate_history?instrument_name={}&start_timestamp={}&end_timestamp={}",
            self.base_url(),
            urlencoding::encode(instrument_name),
            start_timestamp,
            end_timestamp
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
                "Get funding rate history failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<FundingRateData>> = response
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
            HttpError::InvalidResponse("No funding rate history data in response".to_string())
        })
    }

    /// Get funding rate value
    ///
    /// Retrieves interest rate value for requested period. Applicable only for PERPETUAL instruments.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `start_timestamp` - The earliest timestamp to return result from (milliseconds since UNIX epoch)
    /// * `end_timestamp` - The most recent timestamp to return result from (milliseconds since UNIX epoch)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(); // testnet
    /// let funding_rate = client.get_funding_rate_value("BTC-PERPETUAL", 1569888000000, 1569974400000).await?;
    /// println!("Funding rate for period: {}", funding_rate);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_funding_rate_value(
        &self,
        instrument_name: &str,
        start_timestamp: u64,
        end_timestamp: u64,
    ) -> Result<f64, HttpError> {
        let url = format!(
            "{}/public/get_funding_rate_value?instrument_name={}&start_timestamp={}&end_timestamp={}",
            self.base_url(),
            urlencoding::encode(instrument_name),
            start_timestamp,
            end_timestamp
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
                "Get funding rate value failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<f64> = response
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
            HttpError::InvalidResponse("No funding rate value in response".to_string())
        })
    }

    /// Get last settlements by currency
    ///
    /// Retrieves historical settlement, delivery and bankruptcy events coming from all instruments within a given currency.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `settlement_type` - Settlement type (settlement, delivery, bankruptcy) - optional
    /// * `count` - Number of requested items (optional, default 20)
    /// * `continuation` - Continuation token for pagination (optional)
    /// * `search_start_timestamp` - The latest timestamp to return result from (optional)
    ///
    pub async fn get_last_settlements_by_currency(
        &self,
        currency: &str,
        settlement_type: Option<&str>,
        count: Option<u32>,
        continuation: Option<&str>,
        search_start_timestamp: Option<u64>,
    ) -> Result<SettlementsResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_last_settlements_by_currency?currency={}",
            self.base_url(),
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
                "Get last settlements by currency failed: {}",
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

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No settlements data in response".to_string())
        })
    }

    /// Get last settlements by instrument
    ///
    /// Retrieves historical public settlement, delivery and bankruptcy events filtered by instrument name.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `settlement_type` - Settlement type (settlement, delivery, bankruptcy) - optional
    /// * `count` - Number of requested items (optional, default 20)
    /// * `continuation` - Continuation token for pagination (optional)
    /// * `search_start_timestamp` - The latest timestamp to return result from (optional)
    ///
    pub async fn get_last_settlements_by_instrument(
        &self,
        instrument_name: &str,
        settlement_type: Option<&str>,
        count: Option<u32>,
        continuation: Option<&str>,
        search_start_timestamp: Option<u64>,
    ) -> Result<SettlementsResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_last_settlements_by_instrument?instrument_name={}",
            self.base_url(),
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
                "Get last settlements by instrument failed: {}",
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

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No settlements data in response".to_string())
        })
    }

    /// Get last trades by currency
    ///
    /// Retrieves the latest trades that have occurred for instruments in a specific currency.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `kind` - Instrument kind (future, option, spot, etc.) - optional
    /// * `count` - Number of requested items (optional, default 10)
    /// * `include_old` - Include trades older than 7 days (optional)
    /// * `sorting` - Direction of results sorting (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(); // testnet
    /// let trades = client.get_last_trades_by_currency("BTC", Some("future"), Some(10), Some(false), Some("desc")).await?;
    /// for trade in trades.trades {
    ///     println!("Trade: {} {} at {}", trade.amount, trade.instrument_name, trade.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_last_trades_by_currency(
        &self,
        currency: &str,
        kind: Option<&str>,
        count: Option<u32>,
        include_old: Option<bool>,
        sorting: Option<&str>,
    ) -> Result<LastTradesResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_last_trades_by_currency?currency={}",
            self.base_url(),
            urlencoding::encode(currency)
        );

        if let Some(kind) = kind {
            url.push_str(&format!("&kind={}", urlencoding::encode(kind)));
        }

        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        if let Some(include_old) = include_old {
            url.push_str(&format!("&include_old={}", include_old));
        }

        if let Some(sorting) = sorting {
            url.push_str(&format!("&sorting={}", urlencoding::encode(sorting)));
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
                "Get last trades by currency failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<LastTradesResponse> = response
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

    /// Get last trades by currency and time
    ///
    /// Retrieves the latest trades that have occurred for instruments in a specific currency within a time range.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `start_timestamp` - The earliest timestamp to return result from (milliseconds since UNIX epoch)
    /// * `end_timestamp` - The most recent timestamp to return result from (milliseconds since UNIX epoch)
    /// * `kind` - Instrument kind (future, option, spot, etc.) - optional
    /// * `count` - Number of requested items (optional, default 10)
    /// * `include_old` - Include trades older than 7 days (optional)
    /// * `sorting` - Direction of results sorting (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(); // testnet
    /// let trades = client.get_last_trades_by_currency_and_time("BTC", 1569888000000, 1569974400000, Some("future"), Some(10), Some(false), Some("desc")).await?;
    /// for trade in trades.trades {
    ///     println!("Trade: {} {} at {}", trade.amount, trade.instrument_name, trade.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn get_last_trades_by_currency_and_time(
        &self,
        currency: &str,
        start_timestamp: u64,
        end_timestamp: u64,
        kind: Option<&str>,
        count: Option<u32>,
        include_old: Option<bool>,
        sorting: Option<&str>,
    ) -> Result<LastTradesResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_last_trades_by_currency_and_time?currency={}&start_timestamp={}&end_timestamp={}",
            self.base_url(),
            urlencoding::encode(currency),
            start_timestamp,
            end_timestamp
        );

        if let Some(kind) = kind {
            url.push_str(&format!("&kind={}", urlencoding::encode(kind)));
        }

        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        if let Some(include_old) = include_old {
            url.push_str(&format!("&include_old={}", include_old));
        }

        if let Some(sorting) = sorting {
            url.push_str(&format!("&sorting={}", urlencoding::encode(sorting)));
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
                "Get last trades by currency and time failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<LastTradesResponse> = response
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

    /// Get last trades by instrument and time
    ///
    /// Retrieves the latest trades that have occurred for a specific instrument within a time range.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - Instrument name
    /// * `start_timestamp` - The earliest timestamp to return result from (milliseconds since UNIX epoch)
    /// * `end_timestamp` - The most recent timestamp to return result from (milliseconds since UNIX epoch)
    /// * `count` - Number of requested items (optional, default 10)
    /// * `include_old` - Include trades older than 7 days (optional)
    /// * `sorting` - Direction of results sorting (optional)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(); // testnet
    /// let trades = client.get_last_trades_by_instrument_and_time("BTC-PERPETUAL", 1569888000000, 1569974400000, Some(10), Some(false), Some("desc")).await?;
    /// for trade in trades.trades {
    ///     println!("Trade: {} at {} ({})", trade.amount, trade.price, trade.direction);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_last_trades_by_instrument_and_time(
        &self,
        instrument_name: &str,
        start_timestamp: u64,
        end_timestamp: u64,
        count: Option<u32>,
        include_old: Option<bool>,
        sorting: Option<&str>,
    ) -> Result<LastTradesResponse, HttpError> {
        let mut url = format!(
            "{}/public/get_last_trades_by_instrument_and_time?instrument_name={}&start_timestamp={}&end_timestamp={}",
            self.base_url(),
            urlencoding::encode(instrument_name),
            start_timestamp,
            end_timestamp
        );

        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        if let Some(include_old) = include_old {
            url.push_str(&format!("&include_old={}", include_old));
        }

        if let Some(sorting) = sorting {
            url.push_str(&format!("&sorting={}", urlencoding::encode(sorting)));
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
                "Get last trades by instrument and time failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<LastTradesResponse> = response
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

    /// Get order book by instrument ID
    ///
    /// Retrieves the order book for the specified instrument by its ID.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Arguments
    ///
    /// * `instrument_id` - Instrument ID
    /// * `depth` - The number of entries to return for bid and ask order book entries (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(); // testnet
    /// let order_book = client.get_order_book_by_instrument_id(42, Some(5)).await?;
    /// println!("Order book for {}: {} bids, {} asks",
    ///          order_book.instrument_name,
    ///          order_book.bids.len(),
    ///          order_book.asks.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_order_book_by_instrument_id(
        &self,
        instrument_id: u32,
        depth: Option<u32>,
    ) -> Result<OrderBook, HttpError> {
        let mut url = format!(
            "{}/public/get_order_book_by_instrument_id?instrument_id={}",
            self.base_url(),
            instrument_id
        );

        if let Some(depth) = depth {
            url.push_str(&format!("&depth={}", depth));
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
                "Get order book by instrument ID failed: {}",
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
}
