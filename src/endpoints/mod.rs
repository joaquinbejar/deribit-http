pub mod private;
pub mod public;

// Re-export public endpoint data structures
pub use public::{
    AprDataPoint, AprHistoryResponse, BookSummary, ContractSizeResponse, Currency,
    DeliveryPriceData, DeliveryPricesResponse, ExpirationsResponse, FundingChartData,
    FundingDataPoint, FundingRateData, HelloResponse, IndexData, IndexPriceData, Instrument,
    LastTrade, LastTradesResponse, OrderBook, Settlement, SettlementsResponse, StatusResponse,
    TestResponse, TickerData, TickerStats, Trade, TradingViewChartData, WithdrawalPriority,
};

// Re-export private endpoint data structures
pub use private::{
    AccountSummary, BuyOrderRequest, Deposit, DepositsResponse, EditOrderRequest, OrderInfo,
    OrderResponse, OrderType, PortfolioInfo, Position, SellOrderRequest, Subaccount, TimeInForce,
    TradeExecution, TransactionLog, TransactionLogEntry, TransferResult, UserTrade, Withdrawal,
    WithdrawalsResponse,
};
