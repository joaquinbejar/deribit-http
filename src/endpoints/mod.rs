pub mod private;
pub mod public;

pub mod types;

// Re-export public endpoint data structures
pub use types::{
    AprDataPoint, AprHistoryResponse, BookSummary, ContractSizeResponse, Currency,
    DeliveryPriceData, DeliveryPricesResponse, ExpirationsResponse, FundingChartData,
    FundingDataPoint, FundingRateData, HelloResponse, IndexData, IndexPriceData, LastTrade,
    LastTradesResponse, SettlementsResponse, StatusResponse, TestResponse, TradingViewChartData,
};

// Re-export private endpoint data structures
pub use types::{
    AccountSummary, BuyOrderRequest, Deposit, DepositsResponse, EditOrderRequest, MassQuoteRequest,
    MassQuoteResponse, OrderInfo, OrderResponse, PortfolioInfo, QuoteResult, SellOrderRequest,
    Subaccount, TradeExecution, TransactionLog, TransactionLogEntry, TransferResult, UserTrade,
    Withdrawal, WithdrawalsResponse,
};
