pub mod private;
pub mod public;

// Re-export Currency and common models from deribit-base
pub use deribit_base::model::currency::Currency;
pub use deribit_base::prelude::{BookSummary, FundingRateData, TestResponse};

// Re-export private endpoint data structures
pub use deribit_base::prelude::{
    DepositsResponse, TransactionLog, TransactionLogEntry, TransferResult, WithdrawalsResponse,
};
