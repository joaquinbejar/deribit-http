/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Transfer state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferState {
    /// Transfer is prepared but not yet confirmed
    Prepared,
    /// Transfer has been confirmed
    Confirmed,
    /// Transfer has been cancelled
    Cancelled,
    /// Transfer is waiting for admin approval
    WaitingForAdmin,
    /// Transfer failed due to insufficient funds
    InsufficientFunds,
    /// Transfer failed due to withdrawal limit
    WithdrawalLimit,
}

impl Default for TransferState {
    fn default() -> Self {
        Self::Prepared
    }
}

/// Transfer information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    /// Transfer ID
    pub id: i64,
    /// Currency being transferred
    pub currency: String,
    /// Transfer amount
    pub amount: f64,
    /// Transfer fee
    pub fee: f64,
    /// Destination address
    pub address: String,
    /// Blockchain transaction ID
    pub transaction_id: Option<String>,
    /// Current transfer state
    pub state: TransferState,
    /// Creation timestamp (milliseconds since Unix epoch)
    pub created_timestamp: i64,
    /// Last update timestamp (milliseconds since Unix epoch)
    pub updated_timestamp: i64,
    /// Confirmation timestamp (milliseconds since Unix epoch)
    pub confirmed_timestamp: Option<i64>,
    /// Transfer type description
    pub transfer_type: Option<String>,
}

impl Transfer {
    /// Create a new transfer
    pub fn new(
        id: i64,
        currency: String,
        amount: f64,
        fee: f64,
        address: String,
        created_timestamp: i64,
    ) -> Self {
        Self {
            id,
            currency,
            amount,
            fee,
            address,
            transaction_id: None,
            state: TransferState::Prepared,
            created_timestamp,
            updated_timestamp: created_timestamp,
            confirmed_timestamp: None,
            transfer_type: None,
        }
    }

    /// Set transaction ID
    pub fn with_transaction_id(mut self, tx_id: String) -> Self {
        self.transaction_id = Some(tx_id);
        self
    }

    /// Set transfer state
    pub fn with_state(mut self, state: TransferState) -> Self {
        self.state = state;
        self
    }

    /// Set transfer type
    pub fn with_type(mut self, transfer_type: String) -> Self {
        self.transfer_type = Some(transfer_type);
        self
    }

    /// Confirm the transfer
    pub fn confirm(&mut self, timestamp: i64) {
        self.state = TransferState::Confirmed;
        self.confirmed_timestamp = Some(timestamp);
        self.updated_timestamp = timestamp;
    }

    /// Cancel the transfer
    pub fn cancel(&mut self, timestamp: i64) {
        self.state = TransferState::Cancelled;
        self.updated_timestamp = timestamp;
    }

    /// Check if transfer is confirmed
    pub fn is_confirmed(&self) -> bool {
        matches!(self.state, TransferState::Confirmed)
    }

    /// Check if transfer is cancelled
    pub fn is_cancelled(&self) -> bool {
        matches!(self.state, TransferState::Cancelled)
    }

    /// Check if transfer is pending
    pub fn is_pending(&self) -> bool {
        matches!(
            self.state,
            TransferState::Prepared | TransferState::WaitingForAdmin
        )
    }

    /// Get net amount (amount - fee)
    pub fn net_amount(&self) -> f64 {
        self.amount - self.fee
    }
}

/// Collection of transfers
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfers {
    /// List of transfers
    pub transfers: Vec<Transfer>,
}

impl Transfers {
    /// Create a new transfers collection
    pub fn new() -> Self {
        Self {
            transfers: Vec::new(),
        }
    }

    /// Add a transfer
    pub fn add(&mut self, transfer: Transfer) {
        self.transfers.push(transfer);
    }

    /// Get transfers by currency
    pub fn by_currency(&self, currency: String) -> Vec<&Transfer> {
        self.transfers
            .iter()
            .filter(|t| t.currency == currency)
            .collect()
    }

    /// Get transfers by state
    pub fn by_state(&self, state: TransferState) -> Vec<&Transfer> {
        self.transfers.iter().filter(|t| t.state == state).collect()
    }

    /// Get pending transfers
    pub fn pending(&self) -> Vec<&Transfer> {
        self.transfers.iter().filter(|t| t.is_pending()).collect()
    }

    /// Get confirmed transfers
    pub fn confirmed(&self) -> Vec<&Transfer> {
        self.transfers.iter().filter(|t| t.is_confirmed()).collect()
    }

    /// Calculate total amount by currency
    pub fn total_amount(&self, currency: String) -> f64 {
        self.transfers
            .iter()
            .filter(|t| t.currency == currency)
            .map(|t| t.amount)
            .sum()
    }

    /// Calculate total fees by currency
    pub fn total_fees(&self, currency: String) -> f64 {
        self.transfers
            .iter()
            .filter(|t| t.currency == currency)
            .map(|t| t.fee)
            .sum()
    }
}
impl Default for Transfers {
    fn default() -> Self {
        Self::new()
    }
}

/// Subaccount transfer information
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubaccountTransfer {
    /// Transfer amount
    pub amount: f64,
    /// Currency being transferred
    pub currency: String,
    /// Destination subaccount ID
    pub destination: i64,
    /// Transfer ID
    pub id: i64,
    /// Source subaccount ID
    pub source: i64,
    /// Transfer state
    pub state: TransferState,
    /// Transfer timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Type of transfer
    pub transfer_type: String,
}

impl SubaccountTransfer {
    /// Create a new subaccount transfer
    pub fn new(
        id: i64,
        amount: f64,
        currency: String,
        source: i64,
        destination: i64,
        timestamp: i64,
    ) -> Self {
        Self {
            amount,
            currency,
            destination,
            id,
            source,
            state: TransferState::Prepared,
            timestamp,
            transfer_type: "subaccount".to_string(),
        }
    }

    /// Set transfer state
    pub fn with_state(mut self, state: TransferState) -> Self {
        self.state = state;
        self
    }

    /// Set transfer type
    pub fn with_type(mut self, transfer_type: String) -> Self {
        self.transfer_type = transfer_type;
        self
    }

    /// Check if transfer is between main account and subaccount
    pub fn is_main_subaccount_transfer(&self) -> bool {
        self.source == 0 || self.destination == 0
    }

    /// Check if transfer is between subaccounts
    pub fn is_subaccount_to_subaccount(&self) -> bool {
        self.source != 0 && self.destination != 0
    }
}
