//! Prelude module for deribit-http
//!
//! This module re-exports commonly used types and traits for convenience.

// Re-export main client
pub use crate::client::DeribitHttpClient;

// Re-export configuration types
pub use crate::config::{ApiCredentials, HttpConfig};

// Re-export error types
pub use crate::error::HttpError;

// Re-export authentication types
pub use crate::auth::{ApiKeyAuth, AuthManager, AuthRequest};
pub use crate::model::http_types::AuthToken;

// Re-export endpoint response types
pub use crate::endpoints::{
    AccountSummary, BuyOrderRequest, Deposit, DepositsResponse, EditOrderRequest, FundingChartData,
    FundingDataPoint, OrderInfo, OrderResponse, OrderType, PortfolioInfo, Position,
    SellOrderRequest, Subaccount, TimeInForce, TradeExecution, TradingViewChartData,
    TransactionLog, TransactionLogEntry, TransferResult, UserTrade, Withdrawal,
    WithdrawalsResponse,
};

// Re-export message types
pub use crate::message::{HttpMessageBuilder, HttpRequestBuilder, HttpResponseHandler};

// Re-export model types
pub use crate::model::{ApiError, ApiResponse, http_types};

// Re-export session types
pub use crate::session::HttpSession;

// Re-export common types from deribit-base
pub use deribit_base::prelude::*;
