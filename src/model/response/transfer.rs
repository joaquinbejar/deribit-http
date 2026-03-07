/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
//! Transfer response models for internal transfers between subaccounts.

use serde::{Deserialize, Serialize};

/// State of an internal transfer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InternalTransferState {
    /// Transfer is prepared but not yet confirmed
    #[default]
    Prepared,
    /// Transfer has been confirmed
    Confirmed,
    /// Transfer has been cancelled
    Cancelled,
    /// Transfer is waiting for admin approval
    WaitingForAdmin,
}

/// Direction of an internal transfer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TransferDirection {
    /// Outgoing payment
    #[default]
    Payment,
    /// Incoming receipt
    Income,
}

/// Type of internal transfer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InternalTransferType {
    /// Transfer between subaccounts
    #[default]
    Subaccount,
    /// Transfer to/from user
    User,
}

/// Internal transfer information (between subaccounts or users)
///
/// This model represents transfers within the Deribit platform,
/// not blockchain withdrawals/deposits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalTransfer {
    /// Transfer ID
    pub id: i64,
    /// Currency being transferred (e.g., "BTC", "ETH")
    pub currency: String,
    /// Transfer amount
    pub amount: f64,
    /// Direction of the transfer (payment or income)
    pub direction: TransferDirection,
    /// The other party in the transfer (username or subaccount name)
    pub other_side: String,
    /// Current transfer state
    pub state: InternalTransferState,
    /// Type of transfer (subaccount or user)
    #[serde(rename = "type")]
    pub transfer_type: InternalTransferType,
    /// Creation timestamp in milliseconds since Unix epoch
    pub created_timestamp: i64,
    /// Last update timestamp in milliseconds since Unix epoch
    pub updated_timestamp: i64,
}

impl InternalTransfer {
    /// Returns `true` if the transfer is confirmed
    #[must_use]
    pub fn is_confirmed(&self) -> bool {
        self.state == InternalTransferState::Confirmed
    }

    /// Returns `true` if the transfer is cancelled
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state == InternalTransferState::Cancelled
    }

    /// Returns `true` if the transfer is pending (prepared or waiting for admin)
    #[must_use]
    pub fn is_pending(&self) -> bool {
        matches!(
            self.state,
            InternalTransferState::Prepared | InternalTransferState::WaitingForAdmin
        )
    }

    /// Returns `true` if this is an outgoing payment
    #[must_use]
    pub fn is_payment(&self) -> bool {
        self.direction == TransferDirection::Payment
    }

    /// Returns `true` if this is an incoming transfer
    #[must_use]
    pub fn is_income(&self) -> bool {
        self.direction == TransferDirection::Income
    }
}

/// Response for get_transfers endpoint
///
/// Contains a paginated list of internal transfers with total count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransfersResponse {
    /// Total number of transfers matching the query
    pub count: u32,
    /// List of transfer items
    pub data: Vec<InternalTransfer>,
}

impl TransfersResponse {
    /// Returns `true` if there are no transfers
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of transfers in this response
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_transfer_deserialization() {
        let json = r#"{
            "id": 2,
            "created_timestamp": 1550579457727,
            "updated_timestamp": 1550579457727,
            "currency": "BTC",
            "amount": 0.2,
            "direction": "payment",
            "other_side": "new_user_1_1",
            "state": "confirmed",
            "type": "subaccount"
        }"#;

        let transfer: InternalTransfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.id, 2);
        assert_eq!(transfer.currency, "BTC");
        assert!((transfer.amount - 0.2).abs() < f64::EPSILON);
        assert_eq!(transfer.direction, TransferDirection::Payment);
        assert_eq!(transfer.other_side, "new_user_1_1");
        assert_eq!(transfer.state, InternalTransferState::Confirmed);
        assert_eq!(transfer.transfer_type, InternalTransferType::Subaccount);
        assert!(transfer.is_confirmed());
        assert!(transfer.is_payment());
    }

    #[test]
    fn test_transfers_response_deserialization() {
        let json = r#"{
            "count": 1,
            "data": [{
                "id": 2,
                "created_timestamp": 1550579457727,
                "updated_timestamp": 1550579457727,
                "currency": "BTC",
                "amount": 0.2,
                "direction": "payment",
                "other_side": "new_user_1_1",
                "state": "confirmed",
                "type": "subaccount"
            }]
        }"#;

        let response: TransfersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.count, 1);
        assert_eq!(response.len(), 1);
        assert!(!response.is_empty());
    }

    #[test]
    fn test_transfer_state_helpers() {
        let mut transfer = InternalTransfer {
            id: 1,
            currency: "BTC".to_string(),
            amount: 1.0,
            direction: TransferDirection::Payment,
            other_side: "test".to_string(),
            state: InternalTransferState::Prepared,
            transfer_type: InternalTransferType::Subaccount,
            created_timestamp: 0,
            updated_timestamp: 0,
        };

        assert!(transfer.is_pending());
        assert!(!transfer.is_confirmed());
        assert!(!transfer.is_cancelled());

        transfer.state = InternalTransferState::Confirmed;
        assert!(!transfer.is_pending());
        assert!(transfer.is_confirmed());

        transfer.state = InternalTransferState::Cancelled;
        assert!(transfer.is_cancelled());
    }
}
