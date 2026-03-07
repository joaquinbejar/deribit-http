/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 2025
******************************************************************************/
//! Unit tests for wallet models and types

use deribit_http::model::request::wallet::{
    AddToAddressBookRequest, UpdateInAddressBookRequest, WithdrawRequest,
};
use deribit_http::model::response::wallet::AddressBookResponse;
use deribit_http::model::wallet::{
    AddressBookEntry, AddressBookType, DepositAddress, WithdrawalPriorityLevel,
};

// ============================================================================
// AddressBookType Tests
// ============================================================================

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
    assert_eq!(
        format!("{}", AddressBookType::DepositSource),
        "deposit_source"
    );
}

#[test]
fn test_address_book_type_serialization() {
    let json = serde_json::to_string(&AddressBookType::Withdrawal).unwrap();
    assert_eq!(json, "\"withdrawal\"");

    let json = serde_json::to_string(&AddressBookType::Transfer).unwrap();
    assert_eq!(json, "\"transfer\"");

    let json = serde_json::to_string(&AddressBookType::DepositSource).unwrap();
    assert_eq!(json, "\"deposit_source\"");
}

#[test]
fn test_address_book_type_deserialization() {
    let deserialized: AddressBookType = serde_json::from_str("\"transfer\"").unwrap();
    assert_eq!(deserialized, AddressBookType::Transfer);

    let deserialized: AddressBookType = serde_json::from_str("\"withdrawal\"").unwrap();
    assert_eq!(deserialized, AddressBookType::Withdrawal);

    let deserialized: AddressBookType = serde_json::from_str("\"deposit_source\"").unwrap();
    assert_eq!(deserialized, AddressBookType::DepositSource);
}

#[test]
fn test_address_book_type_equality() {
    assert_eq!(AddressBookType::Transfer, AddressBookType::Transfer);
    assert_ne!(AddressBookType::Transfer, AddressBookType::Withdrawal);
}

// ============================================================================
// WithdrawalPriorityLevel Tests
// ============================================================================

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
fn test_withdrawal_priority_level_display() {
    assert_eq!(format!("{}", WithdrawalPriorityLevel::Mid), "mid");
    assert_eq!(format!("{}", WithdrawalPriorityLevel::High), "high");
}

#[test]
fn test_withdrawal_priority_level_serialization() {
    let json = serde_json::to_string(&WithdrawalPriorityLevel::Mid).unwrap();
    assert_eq!(json, "\"mid\"");

    let json = serde_json::to_string(&WithdrawalPriorityLevel::VeryHigh).unwrap();
    assert_eq!(json, "\"very_high\"");
}

#[test]
fn test_withdrawal_priority_level_deserialization() {
    let deserialized: WithdrawalPriorityLevel = serde_json::from_str("\"high\"").unwrap();
    assert_eq!(deserialized, WithdrawalPriorityLevel::High);

    let deserialized: WithdrawalPriorityLevel = serde_json::from_str("\"insane\"").unwrap();
    assert_eq!(deserialized, WithdrawalPriorityLevel::Insane);
}

// ============================================================================
// DepositAddress Tests
// ============================================================================

#[test]
fn test_deposit_address_new() {
    let addr = DepositAddress::new("bc1qtest123".to_string(), "BTC".to_string());
    assert_eq!(addr.address, "bc1qtest123");
    assert_eq!(addr.currency, "BTC");
    assert!(addr.address_type.is_none());
    assert!(addr.creation_timestamp.is_none());
    assert!(addr.status.is_none());
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
    assert!(json.contains("\"creation_timestamp\":1234567890000"));
}

#[test]
fn test_deposit_address_deserialization() {
    let json = r#"{
        "address": "bc1qtest",
        "currency": "BTC",
        "type": "deposit",
        "creation_timestamp": 1234567890000,
        "status": "active"
    }"#;

    let addr: DepositAddress = serde_json::from_str(json).unwrap();
    assert_eq!(addr.address, "bc1qtest");
    assert_eq!(addr.currency, "BTC");
    assert_eq!(addr.address_type, Some("deposit".to_string()));
    assert_eq!(addr.creation_timestamp, Some(1234567890000));
    assert_eq!(addr.status, Some("active".to_string()));
}

#[test]
fn test_deposit_address_deserialization_minimal() {
    let json = r#"{
        "address": "0xminimal",
        "currency": "USDC"
    }"#;

    let addr: DepositAddress = serde_json::from_str(json).unwrap();
    assert_eq!(addr.address, "0xminimal");
    assert_eq!(addr.currency, "USDC");
    assert!(addr.address_type.is_none());
}

// ============================================================================
// AddressBookEntry Tests
// ============================================================================

#[test]
fn test_address_book_entry_new() {
    let entry = AddressBookEntry::new("bc1qtest456".to_string(), "BTC".to_string());
    assert_eq!(entry.address, "bc1qtest456");
    assert_eq!(entry.currency, "BTC");
    assert!(entry.label.is_none());
    assert!(entry.agreed.is_none());
    assert!(entry.personal.is_none());
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
    assert!(json.contains("\"personal\":false"));
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

// ============================================================================
// AddToAddressBookRequest Tests
// ============================================================================

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

// ============================================================================
// UpdateInAddressBookRequest Tests
// ============================================================================

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
    assert!(req.beneficiary_first_name.is_none());
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
    .with_beneficiary_vasp_website("https://vasp.example.com".to_string())
    .with_beneficiary_company_name("Test Corp".to_string())
    .with_tag("12345".to_string());

    assert_eq!(req.beneficiary_first_name, Some("John".to_string()));
    assert_eq!(req.beneficiary_last_name, Some("Doe".to_string()));
    assert_eq!(
        req.beneficiary_vasp_website,
        Some("https://vasp.example.com".to_string())
    );
    assert_eq!(req.beneficiary_company_name, Some("Test Corp".to_string()));
    assert_eq!(req.tag, Some("12345".to_string()));
}

#[test]
fn test_update_in_address_book_request_serialization() {
    let req = UpdateInAddressBookRequest::new(
        "BTC".to_string(),
        AddressBookType::Withdrawal,
        "bc1qtest".to_string(),
        "Label".to_string(),
        true,
        false,
        "VASP Name".to_string(),
        "did:123".to_string(),
        "Address".to_string(),
    );

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"currency\":\"BTC\""));
    assert!(json.contains("\"type\":\"withdrawal\""));
    assert!(json.contains("\"agreed\":true"));
    assert!(json.contains("\"personal\":false"));
    assert!(json.contains("\"beneficiary_vasp_name\":\"VASP Name\""));
}

// ============================================================================
// WithdrawRequest Tests
// ============================================================================

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

#[test]
fn test_withdraw_request_serialization_without_priority() {
    let req = WithdrawRequest::new("BTC".to_string(), "bc1qtest".to_string(), 1.0);

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"currency\":\"BTC\""));
    assert!(!json.contains("\"priority\""));
}

// ============================================================================
// AddressBookResponse Tests
// ============================================================================

#[test]
fn test_address_book_response_new() {
    let entries = vec![
        AddressBookEntry::new("bc1qtest1".to_string(), "BTC".to_string()),
        AddressBookEntry::new("bc1qtest2".to_string(), "BTC".to_string()),
    ];
    let response = AddressBookResponse::new(entries);
    assert_eq!(response.len(), 2);
    assert!(!response.is_empty());
}

#[test]
fn test_address_book_response_default() {
    let response = AddressBookResponse::default();
    assert!(response.is_empty());
    assert_eq!(response.len(), 0);
}

#[test]
fn test_address_book_response_serialization() {
    let entries = vec![AddressBookEntry::new(
        "0xtest".to_string(),
        "ETH".to_string(),
    )];
    let response = AddressBookResponse::new(entries);

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("\"entries\""));
    assert!(json.contains("\"address\":\"0xtest\""));
}

#[test]
fn test_address_book_response_deserialization() {
    let json = r#"{
        "entries": [
            {
                "address": "bc1qtest",
                "currency": "BTC",
                "label": "Test wallet"
            }
        ]
    }"#;

    let response: AddressBookResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response.entries[0].address, "bc1qtest");
    assert_eq!(response.entries[0].currency, "BTC");
}

// ============================================================================
// API Response Format Tests
// ============================================================================

#[test]
fn test_deposit_address_api_response_format() {
    // Test that we can deserialize the exact format returned by Deribit API
    let json = r#"{
        "address": "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz",
        "currency": "BTC",
        "type": "deposit",
        "creation_timestamp": 1550574558607
    }"#;

    let addr: DepositAddress = serde_json::from_str(json).unwrap();
    assert_eq!(addr.address, "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz");
    assert_eq!(addr.currency, "BTC");
}

#[test]
fn test_address_book_entry_api_response_format() {
    // Test that we can deserialize the exact format returned by Deribit API
    let json = r#"{
        "address": "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf0uyj",
        "agreed": true,
        "beneficiary_address": "NL, Amsterdam, Street, 1",
        "beneficiary_first_name": "John",
        "beneficiary_last_name": "Doe",
        "beneficiary_vasp_did": "did:example:123456789abcdefghi",
        "beneficiary_vasp_name": "Money's Gone",
        "creation_timestamp": 1550574558607,
        "currency": "BTC",
        "personal": false,
        "type": "withdrawal"
    }"#;

    let entry: AddressBookEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.address, "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf0uyj");
    assert_eq!(entry.currency, "BTC");
    assert_eq!(entry.agreed, Some(true));
    assert_eq!(entry.personal, Some(false));
    assert_eq!(entry.beneficiary_first_name, Some("John".to_string()));
}
