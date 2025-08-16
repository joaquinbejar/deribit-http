//! # Deribit HTTP Client
//!
//! This crate provides a HTTP REST API client for the Deribit trading platform.
//! It implements the common traits from `deribit-base` and provides methods
//! for all REST API endpoints.

pub mod auth;
pub mod client;
pub mod config;
pub mod connection;
pub mod endpoints;
pub mod error;
pub mod message;
pub mod model;
pub mod prelude;
pub mod rate_limit;
pub mod session;

// Constants
pub mod constants;

// Re-export main client and error types
pub use client::*;
pub use error::*;

// Re-export specific types to avoid conflicts
pub use auth::{ApiKeyAuth, AuthManager};
pub use config::ApiCredentials;
pub use connection::*;
pub use endpoints::{
    AccountSummary, BuyOrderRequest, Deposit, DepositsResponse, EditOrderRequest, FundingChartData,
    FundingDataPoint, Instrument, OrderBook, OrderInfo, OrderResponse, OrderType, PortfolioInfo,
    Position, SellOrderRequest, Subaccount, TickerData, TickerStats, TimeInForce, Trade,
    TradeExecution, TradingViewChartData, TransactionLog, TransactionLogEntry, TransferResult,
    UserTrade, Withdrawal, WithdrawalsResponse,
};
pub use message::{HttpMessageBuilder, HttpRequestBuilder, HttpResponseHandler};
pub use model::{ApiError, ApiResponse, AuthToken, http_types};
pub use session::*;

// Re-export common types from deribit-base (avoiding conflicts)
pub use deribit_base::prelude::*;

// Allow ambiguous re-exports for remaining conflicts
#[allow(ambiguous_glob_reexports)]
pub use config::HttpConfig;

#[allow(ambiguous_glob_reexports)]
pub use auth::AuthRequest;
