pub mod private;
pub mod public;

// Re-export public endpoint data structures
pub use public::{
    AprDataPoint, AprHistoryResponse, BookSummary, ContractSizeResponse, Currency, 
    FundingChartData, FundingDataPoint, HelloResponse, IndexData, IndexPriceData,
    Instrument, OrderBook, StatusResponse, TestResponse, TickerData, TickerStats, 
    TradingViewChartData, Trade, WithdrawalPriority,
};

// Re-export private endpoint data structures  
pub use private::{
    AccountSummary, BuyOrderRequest, Deposit, DepositsResponse, EditOrderRequest, 
    OrderInfo, OrderResponse, OrderType, PortfolioInfo, Position, SellOrderRequest, 
    Subaccount, TimeInForce, TradeExecution, TransactionLog, TransactionLogEntry, 
    TransferResult, UserTrade, Withdrawal, WithdrawalsResponse,
};