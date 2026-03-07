/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 2025
******************************************************************************/
//! Wallet-related models for deposit addresses and address book operations.

use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Address book entry type.
///
/// Specifies the type of address book entry for wallet operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum AddressBookType {
    /// Address used for transfers between accounts
    Transfer = 0,
    /// Address used for external withdrawals
    Withdrawal = 1,
    /// Address used as deposit source identification
    DepositSource = 2,
}

impl AddressBookType {
    /// Returns the string representation of the address book type.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            AddressBookType::Transfer => "transfer",
            AddressBookType::Withdrawal => "withdrawal",
            AddressBookType::DepositSource => "deposit_source",
        }
    }
}

impl std::fmt::Display for AddressBookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Withdrawal priority level for blockchain transactions.
///
/// Higher priority levels result in faster transaction confirmation
/// but incur higher fees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
#[derive(Default)]
pub enum WithdrawalPriorityLevel {
    /// Lowest priority with minimal fees
    VeryLow = 0,
    /// Low priority
    Low = 1,
    /// Medium priority (default for most currencies)
    Mid = 2,
    /// High priority (default for BTC)
    #[default]
    High = 3,
    /// Very high priority
    VeryHigh = 4,
    /// Extreme high priority
    ExtremeHigh = 5,
    /// Insane priority with maximum fees for fastest confirmation
    Insane = 6,
}

impl WithdrawalPriorityLevel {
    /// Returns the string representation of the priority level.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            WithdrawalPriorityLevel::VeryLow => "very_low",
            WithdrawalPriorityLevel::Low => "low",
            WithdrawalPriorityLevel::Mid => "mid",
            WithdrawalPriorityLevel::High => "high",
            WithdrawalPriorityLevel::VeryHigh => "very_high",
            WithdrawalPriorityLevel::ExtremeHigh => "extreme_high",
            WithdrawalPriorityLevel::Insane => "insane",
        }
    }
}

impl std::fmt::Display for WithdrawalPriorityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Deposit address information returned by the Deribit API.
///
/// Contains details about a cryptocurrency deposit address
/// including the address itself and associated metadata.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DepositAddress {
    /// The cryptocurrency deposit address
    pub address: String,
    /// Currency symbol (e.g., "BTC", "ETH", "USDC")
    pub currency: String,
    /// Address type (e.g., "deposit")
    #[serde(rename = "type")]
    pub address_type: Option<String>,
    /// Timestamp when the address was created, in milliseconds since Unix epoch
    pub creation_timestamp: Option<u64>,
    /// Status of the address
    pub status: Option<String>,
}

impl DepositAddress {
    /// Creates a new deposit address with the required fields.
    #[must_use]
    pub fn new(address: String, currency: String) -> Self {
        Self {
            address,
            currency,
            address_type: None,
            creation_timestamp: None,
            status: None,
        }
    }
}

/// Address book entry containing wallet address information.
///
/// Represents an entry in the user's address book, which can be used
/// for withdrawals, transfers, or deposit source identification.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct AddressBookEntry {
    /// Address in proper format for the currency
    pub address: String,
    /// Currency symbol (e.g., "BTC", "ETH", "USDC")
    pub currency: String,
    /// Type of address book entry
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    /// User-defined label for the address
    pub label: Option<String>,
    /// Timestamp when the entry was created, in milliseconds since Unix epoch
    pub creation_timestamp: Option<u64>,
    /// Timestamp when the entry was last updated, in milliseconds since Unix epoch
    pub update_timestamp: Option<u64>,
    /// Whether the user agreed to share information with third parties
    pub agreed: Option<bool>,
    /// Whether the address belongs to the user (personal/un-hosted wallet)
    pub personal: Option<bool>,
    /// Whether the address belongs to an unhosted wallet
    pub unhosted: Option<bool>,
    /// Tag for XRP addresses (destination tag)
    pub tag: Option<String>,
    /// Name of the beneficiary VASP (Virtual Asset Service Provider)
    pub beneficiary_vasp_name: Option<String>,
    /// DID (Decentralized Identifier) of the beneficiary VASP
    pub beneficiary_vasp_did: Option<String>,
    /// Website of the beneficiary VASP
    pub beneficiary_vasp_website: Option<String>,
    /// First name of the beneficiary (if a person)
    pub beneficiary_first_name: Option<String>,
    /// Last name of the beneficiary (if a person)
    pub beneficiary_last_name: Option<String>,
    /// Company name of the beneficiary (if a company)
    pub beneficiary_company_name: Option<String>,
    /// Geographical address of the beneficiary
    pub beneficiary_address: Option<String>,
}

impl AddressBookEntry {
    /// Creates a new address book entry with the required fields.
    #[must_use]
    pub fn new(address: String, currency: String) -> Self {
        Self {
            address,
            currency,
            entry_type: None,
            label: None,
            creation_timestamp: None,
            update_timestamp: None,
            agreed: None,
            personal: None,
            unhosted: None,
            tag: None,
            beneficiary_vasp_name: None,
            beneficiary_vasp_did: None,
            beneficiary_vasp_website: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_company_name: None,
            beneficiary_address: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_book_type_as_str() {
        assert_eq!(AddressBookType::Transfer.as_str(), "transfer");
        assert_eq!(AddressBookType::Withdrawal.as_str(), "withdrawal");
        assert_eq!(AddressBookType::DepositSource.as_str(), "deposit_source");
    }

    #[test]
    fn test_address_book_type_display() {
        assert_eq!(format!("{}", AddressBookType::Transfer), "transfer");
        assert_eq!(format!("{}", AddressBookType::Withdrawal), "withdrawal");
    }

    #[test]
    fn test_address_book_type_serialization() {
        let json = serde_json::to_string(&AddressBookType::Withdrawal).unwrap();
        assert_eq!(json, "\"withdrawal\"");

        let deserialized: AddressBookType = serde_json::from_str("\"transfer\"").unwrap();
        assert_eq!(deserialized, AddressBookType::Transfer);
    }

    #[test]
    fn test_withdrawal_priority_level_as_str() {
        assert_eq!(WithdrawalPriorityLevel::VeryLow.as_str(), "very_low");
        assert_eq!(WithdrawalPriorityLevel::Low.as_str(), "low");
        assert_eq!(WithdrawalPriorityLevel::Mid.as_str(), "mid");
        assert_eq!(WithdrawalPriorityLevel::High.as_str(), "high");
        assert_eq!(WithdrawalPriorityLevel::VeryHigh.as_str(), "very_high");
        assert_eq!(
            WithdrawalPriorityLevel::ExtremeHigh.as_str(),
            "extreme_high"
        );
        assert_eq!(WithdrawalPriorityLevel::Insane.as_str(), "insane");
    }

    #[test]
    fn test_withdrawal_priority_level_default() {
        assert_eq!(
            WithdrawalPriorityLevel::default(),
            WithdrawalPriorityLevel::High
        );
    }

    #[test]
    fn test_withdrawal_priority_level_serialization() {
        let json = serde_json::to_string(&WithdrawalPriorityLevel::Mid).unwrap();
        assert_eq!(json, "\"mid\"");

        let deserialized: WithdrawalPriorityLevel = serde_json::from_str("\"high\"").unwrap();
        assert_eq!(deserialized, WithdrawalPriorityLevel::High);
    }

    #[test]
    fn test_deposit_address_new() {
        let addr = DepositAddress::new("bc1qtest123".to_string(), "BTC".to_string());
        assert_eq!(addr.address, "bc1qtest123");
        assert_eq!(addr.currency, "BTC");
        assert!(addr.address_type.is_none());
    }

    #[test]
    fn test_deposit_address_serialization() {
        let addr = DepositAddress {
            address: "0xtest123".to_string(),
            currency: "ETH".to_string(),
            address_type: Some("deposit".to_string()),
            creation_timestamp: Some(1234567890000),
            status: Some("active".to_string()),
        };

        let json = serde_json::to_string(&addr).unwrap();
        assert!(json.contains("\"address\":\"0xtest123\""));
        assert!(json.contains("\"currency\":\"ETH\""));
        assert!(json.contains("\"type\":\"deposit\""));
    }

    #[test]
    fn test_address_book_entry_new() {
        let entry = AddressBookEntry::new("bc1qtest456".to_string(), "BTC".to_string());
        assert_eq!(entry.address, "bc1qtest456");
        assert_eq!(entry.currency, "BTC");
        assert!(entry.label.is_none());
        assert!(entry.agreed.is_none());
    }

    #[test]
    fn test_address_book_entry_serialization() {
        let entry = AddressBookEntry {
            address: "bc1qtest789".to_string(),
            currency: "BTC".to_string(),
            entry_type: Some("withdrawal".to_string()),
            label: Some("Main wallet".to_string()),
            creation_timestamp: Some(1234567890000),
            update_timestamp: None,
            agreed: Some(true),
            personal: Some(false),
            unhosted: None,
            tag: None,
            beneficiary_vasp_name: Some("Test VASP".to_string()),
            beneficiary_vasp_did: None,
            beneficiary_vasp_website: None,
            beneficiary_first_name: Some("John".to_string()),
            beneficiary_last_name: Some("Doe".to_string()),
            beneficiary_company_name: None,
            beneficiary_address: Some("123 Main St".to_string()),
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("\"address\":\"bc1qtest789\""));
        assert!(json.contains("\"type\":\"withdrawal\""));
        assert!(json.contains("\"label\":\"Main wallet\""));
        assert!(json.contains("\"agreed\":true"));
    }

    #[test]
    fn test_address_book_entry_deserialization() {
        let json = r#"{
            "address": "bc1qtest",
            "currency": "BTC",
            "type": "transfer",
            "label": "Test",
            "creation_timestamp": 1234567890000,
            "agreed": true,
            "personal": false,
            "beneficiary_first_name": "Jane",
            "beneficiary_last_name": "Smith"
        }"#;

        let entry: AddressBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.address, "bc1qtest");
        assert_eq!(entry.currency, "BTC");
        assert_eq!(entry.entry_type, Some("transfer".to_string()));
        assert_eq!(entry.label, Some("Test".to_string()));
        assert_eq!(entry.agreed, Some(true));
        assert_eq!(entry.personal, Some(false));
        assert_eq!(entry.beneficiary_first_name, Some("Jane".to_string()));
    }
}
