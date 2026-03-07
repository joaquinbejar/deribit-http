/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 2025
******************************************************************************/
//! Request models for wallet operations.

use crate::model::wallet::{AddressBookType, WithdrawalPriorityLevel};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request parameters for adding an address to the address book.
///
/// Used with the `/private/add_to_address_book` endpoint.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToAddressBookRequest {
    /// Currency symbol (e.g., "BTC", "ETH", "USDC")
    pub currency: String,
    /// Address book entry type
    #[serde(rename = "type")]
    pub address_type: AddressBookType,
    /// Address in proper format for the currency
    pub address: String,
    /// User-defined label for the address
    pub label: Option<String>,
    /// Tag for XRP addresses (destination tag)
    pub tag: Option<String>,
}

impl AddToAddressBookRequest {
    /// Creates a new request to add an address to the address book.
    #[must_use]
    pub fn new(currency: String, address_type: AddressBookType, address: String) -> Self {
        Self {
            currency,
            address_type,
            address,
            label: None,
            tag: None,
        }
    }

    /// Sets the label for the address book entry.
    #[must_use]
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Sets the tag for XRP addresses.
    #[must_use]
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }
}

/// Request parameters for updating an address in the address book.
///
/// Used with the `/private/update_in_address_book` endpoint.
/// This endpoint allows providing beneficiary information for travel rule compliance.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInAddressBookRequest {
    /// Currency symbol (e.g., "BTC", "ETH", "USDC")
    pub currency: String,
    /// Address book entry type
    #[serde(rename = "type")]
    pub address_type: AddressBookType,
    /// Address in proper format for the currency
    pub address: String,
    /// User-defined label for the address
    pub label: String,
    /// Whether the user agrees to share information with third parties
    pub agreed: bool,
    /// Whether the address belongs to the user (personal/un-hosted wallet)
    pub personal: bool,
    /// Name of the beneficiary VASP (Virtual Asset Service Provider)
    pub beneficiary_vasp_name: String,
    /// DID (Decentralized Identifier) of the beneficiary VASP
    pub beneficiary_vasp_did: String,
    /// Geographical address of the beneficiary
    pub beneficiary_address: String,
    /// Website of the beneficiary VASP (required if VASP not in known list)
    pub beneficiary_vasp_website: Option<String>,
    /// First name of the beneficiary (if a person)
    pub beneficiary_first_name: Option<String>,
    /// Last name of the beneficiary (if a person)
    pub beneficiary_last_name: Option<String>,
    /// Company name of the beneficiary (if a company)
    pub beneficiary_company_name: Option<String>,
    /// Tag for XRP addresses (destination tag)
    pub tag: Option<String>,
}

impl UpdateInAddressBookRequest {
    /// Creates a new request to update an address in the address book.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency symbol (e.g., "BTC", "ETH")
    /// * `address_type` - Type of address book entry
    /// * `address` - The cryptocurrency address
    /// * `label` - User-defined label for the address
    /// * `agreed` - Whether user agrees to share info with third parties
    /// * `personal` - Whether the address belongs to the user
    /// * `beneficiary_vasp_name` - Name of the beneficiary VASP
    /// * `beneficiary_vasp_did` - DID of the beneficiary VASP
    /// * `beneficiary_address` - Geographical address of the beneficiary
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        currency: String,
        address_type: AddressBookType,
        address: String,
        label: String,
        agreed: bool,
        personal: bool,
        beneficiary_vasp_name: String,
        beneficiary_vasp_did: String,
        beneficiary_address: String,
    ) -> Self {
        Self {
            currency,
            address_type,
            address,
            label,
            agreed,
            personal,
            beneficiary_vasp_name,
            beneficiary_vasp_did,
            beneficiary_address,
            beneficiary_vasp_website: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_company_name: None,
            tag: None,
        }
    }

    /// Sets the beneficiary VASP website.
    #[must_use]
    pub fn with_beneficiary_vasp_website(mut self, website: String) -> Self {
        self.beneficiary_vasp_website = Some(website);
        self
    }

    /// Sets the beneficiary first name.
    #[must_use]
    pub fn with_beneficiary_first_name(mut self, first_name: String) -> Self {
        self.beneficiary_first_name = Some(first_name);
        self
    }

    /// Sets the beneficiary last name.
    #[must_use]
    pub fn with_beneficiary_last_name(mut self, last_name: String) -> Self {
        self.beneficiary_last_name = Some(last_name);
        self
    }

    /// Sets the beneficiary company name.
    #[must_use]
    pub fn with_beneficiary_company_name(mut self, company_name: String) -> Self {
        self.beneficiary_company_name = Some(company_name);
        self
    }

    /// Sets the tag for XRP addresses.
    #[must_use]
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }
}

/// Request parameters for withdrawing funds.
///
/// Used with the `/private/withdraw` endpoint.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawRequest {
    /// Currency symbol (e.g., "BTC", "ETH", "USDC")
    pub currency: String,
    /// Withdrawal address (must be in address book)
    pub address: String,
    /// Amount to withdraw
    pub amount: f64,
    /// Withdrawal priority level (affects fees and confirmation time)
    pub priority: Option<WithdrawalPriorityLevel>,
}

impl WithdrawRequest {
    /// Creates a new withdrawal request.
    #[must_use]
    pub fn new(currency: String, address: String, amount: f64) -> Self {
        Self {
            currency,
            address,
            amount,
            priority: None,
        }
    }

    /// Sets the withdrawal priority level.
    #[must_use]
    pub fn with_priority(mut self, priority: WithdrawalPriorityLevel) -> Self {
        self.priority = Some(priority);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_address_book_request_new() {
        let req = AddToAddressBookRequest::new(
            "BTC".to_string(),
            AddressBookType::Withdrawal,
            "bc1qtest123".to_string(),
        );
        assert_eq!(req.currency, "BTC");
        assert_eq!(req.address_type, AddressBookType::Withdrawal);
        assert_eq!(req.address, "bc1qtest123");
        assert!(req.label.is_none());
        assert!(req.tag.is_none());
    }

    #[test]
    fn test_add_to_address_book_request_with_options() {
        let req = AddToAddressBookRequest::new(
            "XRP".to_string(),
            AddressBookType::Transfer,
            "rTest123".to_string(),
        )
        .with_label("My XRP wallet".to_string())
        .with_tag("12345".to_string());

        assert_eq!(req.label, Some("My XRP wallet".to_string()));
        assert_eq!(req.tag, Some("12345".to_string()));
    }

    #[test]
    fn test_add_to_address_book_request_serialization() {
        let req = AddToAddressBookRequest::new(
            "BTC".to_string(),
            AddressBookType::Withdrawal,
            "bc1qtest".to_string(),
        )
        .with_label("Test".to_string());

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"currency\":\"BTC\""));
        assert!(json.contains("\"type\":\"withdrawal\""));
        assert!(json.contains("\"address\":\"bc1qtest\""));
        assert!(json.contains("\"label\":\"Test\""));
    }

    #[test]
    fn test_update_in_address_book_request_new() {
        let req = UpdateInAddressBookRequest::new(
            "BTC".to_string(),
            AddressBookType::Withdrawal,
            "bc1qtest".to_string(),
            "Main wallet".to_string(),
            true,
            false,
            "Test VASP".to_string(),
            "did:example:123".to_string(),
            "123 Main St".to_string(),
        );

        assert_eq!(req.currency, "BTC");
        assert_eq!(req.address_type, AddressBookType::Withdrawal);
        assert!(req.agreed);
        assert!(!req.personal);
        assert!(req.beneficiary_vasp_website.is_none());
    }

    #[test]
    fn test_update_in_address_book_request_with_options() {
        let req = UpdateInAddressBookRequest::new(
            "ETH".to_string(),
            AddressBookType::Transfer,
            "0xtest".to_string(),
            "Test".to_string(),
            true,
            false,
            "VASP".to_string(),
            "did:test".to_string(),
            "Address".to_string(),
        )
        .with_beneficiary_first_name("John".to_string())
        .with_beneficiary_last_name("Doe".to_string())
        .with_beneficiary_vasp_website("https://vasp.example.com".to_string());

        assert_eq!(req.beneficiary_first_name, Some("John".to_string()));
        assert_eq!(req.beneficiary_last_name, Some("Doe".to_string()));
        assert_eq!(
            req.beneficiary_vasp_website,
            Some("https://vasp.example.com".to_string())
        );
    }

    #[test]
    fn test_withdraw_request_new() {
        let req = WithdrawRequest::new("BTC".to_string(), "bc1qtest".to_string(), 0.5);
        assert_eq!(req.currency, "BTC");
        assert_eq!(req.address, "bc1qtest");
        assert!((req.amount - 0.5).abs() < f64::EPSILON);
        assert!(req.priority.is_none());
    }

    #[test]
    fn test_withdraw_request_with_priority() {
        let req = WithdrawRequest::new("BTC".to_string(), "bc1qtest".to_string(), 1.0)
            .with_priority(WithdrawalPriorityLevel::High);

        assert_eq!(req.priority, Some(WithdrawalPriorityLevel::High));
    }

    #[test]
    fn test_withdraw_request_serialization() {
        let req = WithdrawRequest::new("ETH".to_string(), "0xtest".to_string(), 2.5)
            .with_priority(WithdrawalPriorityLevel::Mid);

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"currency\":\"ETH\""));
        assert!(json.contains("\"address\":\"0xtest\""));
        assert!(json.contains("\"priority\":\"mid\""));
    }
}
