/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Transaction type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TransactionType {
    /// Deposit transaction
    Deposit,
    /// Withdrawal transaction
    Withdrawal,
    /// Trade transaction (default)
    #[default]
    Trade,
    /// Transfer transaction
    Transfer,
    /// Fee transaction
    Fee,
    /// Funding transaction
    Funding,
    /// Bonus transaction
    Bonus,
    /// Dividend transaction
    Dividend,
    /// Liquidation transaction
    Liquidation,
    /// Insurance transaction
    Insurance,
}

/// Generic transaction log entry
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TransactionLogEntry {
    /// Unique transaction identifier
    pub id: u64,
    /// Currency of the transaction
    pub currency: String,
    /// Transaction amount
    pub amount: f64,
    /// Account balance after transaction
    pub balance: f64,
    /// Transaction timestamp
    pub timestamp: u64,
    /// Type of transaction
    pub transaction_type: TransactionType,
    /// Additional transaction information
    pub info: Option<String>,
}

impl Default for TransactionLogEntry {
    fn default() -> Self {
        Self {
            id: 0,
            currency: String::new(),
            amount: 0.0,
            balance: 0.0,
            timestamp: 0,
            transaction_type: TransactionType::default(),
            info: None,
        }
    }
}
