/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Deposit information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// Deposit address
    pub address: String,
    /// Deposit amount
    pub amount: f64,
    /// Currency of the deposit
    pub currency: String,
    /// Current state of the deposit
    pub state: String,
    /// Timestamp when deposit was received
    pub received_timestamp: u64,
    /// Transaction ID on the blockchain
    pub transaction_id: Option<String>,
    /// Timestamp when deposit was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<u64>,
}