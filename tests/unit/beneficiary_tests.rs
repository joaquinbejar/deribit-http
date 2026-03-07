//! Unit tests for address beneficiary models

use deribit_http::model::{
    AddressBeneficiary, ClearanceDepositResult, DepositId, ListAddressBeneficiariesRequest,
    ListAddressBeneficiariesResponse, Originator, SaveAddressBeneficiaryRequest,
};

#[test]
fn test_address_beneficiary_deserialize_full() {
    let json = r#"{
        "currency": "BTC",
        "address": "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf0uyj",
        "user_id": 1016,
        "agreed": true,
        "personal": false,
        "unhosted": false,
        "beneficiary_vasp_name": "Money's Gone",
        "beneficiary_vasp_did": "did:example:123456789abcdefghi",
        "beneficiary_vasp_website": "https://example.com",
        "beneficiary_first_name": "John",
        "beneficiary_last_name": "Doe",
        "beneficiary_company_name": "Example Corp",
        "beneficiary_address": "NL, Amsterdam, Street, 1",
        "created": 1536569522277,
        "updated": 1536569522277
    }"#;

    let beneficiary: AddressBeneficiary = serde_json::from_str(json).unwrap();
    assert_eq!(beneficiary.currency, "BTC");
    assert_eq!(
        beneficiary.address,
        "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf0uyj"
    );
    assert_eq!(beneficiary.user_id, Some(1016));
    assert!(beneficiary.agreed);
    assert!(!beneficiary.personal);
    assert!(!beneficiary.unhosted);
    assert_eq!(
        beneficiary.beneficiary_vasp_name,
        Some("Money's Gone".to_string())
    );
    assert_eq!(
        beneficiary.beneficiary_vasp_did,
        Some("did:example:123456789abcdefghi".to_string())
    );
    assert_eq!(beneficiary.beneficiary_first_name, Some("John".to_string()));
    assert_eq!(beneficiary.beneficiary_last_name, Some("Doe".to_string()));
    assert_eq!(
        beneficiary.beneficiary_company_name,
        Some("Example Corp".to_string())
    );
    assert_eq!(beneficiary.creation_timestamp, Some(1536569522277));
    assert_eq!(beneficiary.update_timestamp, Some(1536569522277));
}

#[test]
fn test_address_beneficiary_deserialize_minimal() {
    let json = r#"{
        "currency": "ETH",
        "address": "0x123",
        "agreed": true,
        "personal": true,
        "unhosted": true
    }"#;

    let beneficiary: AddressBeneficiary = serde_json::from_str(json).unwrap();
    assert_eq!(beneficiary.currency, "ETH");
    assert_eq!(beneficiary.address, "0x123");
    assert!(beneficiary.agreed);
    assert!(beneficiary.personal);
    assert!(beneficiary.unhosted);
    assert!(beneficiary.user_id.is_none());
    assert!(beneficiary.beneficiary_vasp_name.is_none());
    assert!(beneficiary.creation_timestamp.is_none());
}

#[test]
fn test_save_request_serialize_full() {
    let request = SaveAddressBeneficiaryRequest {
        currency: "BTC".to_string(),
        address: "bc1qtest".to_string(),
        tag: Some("tag123".to_string()),
        agreed: true,
        personal: false,
        unhosted: false,
        beneficiary_vasp_name: "Test VASP".to_string(),
        beneficiary_vasp_did: "did:test:123".to_string(),
        beneficiary_vasp_website: Some("https://test.com".to_string()),
        beneficiary_first_name: Some("John".to_string()),
        beneficiary_last_name: Some("Doe".to_string()),
        beneficiary_company_name: Some("Test Corp".to_string()),
        beneficiary_address: "Test Address".to_string(),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"currency\":\"BTC\""));
    assert!(json.contains("\"address\":\"bc1qtest\""));
    assert!(json.contains("\"tag\":\"tag123\""));
    assert!(json.contains("\"agreed\":true"));
    assert!(json.contains("\"personal\":false"));
    assert!(json.contains("\"beneficiary_vasp_name\":\"Test VASP\""));
    assert!(json.contains("\"beneficiary_company_name\":\"Test Corp\""));
}

#[test]
fn test_save_request_serialize_minimal() {
    let request = SaveAddressBeneficiaryRequest {
        currency: "BTC".to_string(),
        address: "bc1qtest".to_string(),
        tag: None,
        agreed: true,
        personal: false,
        unhosted: false,
        beneficiary_vasp_name: "VASP".to_string(),
        beneficiary_vasp_did: "did:test".to_string(),
        beneficiary_vasp_website: None,
        beneficiary_first_name: None,
        beneficiary_last_name: None,
        beneficiary_company_name: None,
        beneficiary_address: "Address".to_string(),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"currency\":\"BTC\""));
    assert!(!json.contains("\"tag\"")); // Should be skipped when None
    assert!(!json.contains("\"beneficiary_vasp_website\"")); // Should be skipped
    assert!(!json.contains("\"beneficiary_first_name\"")); // Should be skipped
    assert!(!json.contains("\"beneficiary_company_name\"")); // Should be skipped
}

#[test]
fn test_save_request_default() {
    let request = SaveAddressBeneficiaryRequest::default();
    assert!(request.currency.is_empty());
    assert!(request.address.is_empty());
    assert!(request.tag.is_none());
    assert!(!request.agreed);
    assert!(!request.personal);
    assert!(!request.unhosted);
}

#[test]
fn test_list_request_default() {
    let request = ListAddressBeneficiariesRequest::default();
    assert!(request.currency.is_none());
    assert!(request.address.is_none());
    assert!(request.limit.is_none());
    assert!(request.continuation.is_none());
    assert!(request.personal.is_none());
    assert!(request.unhosted.is_none());
}

#[test]
fn test_list_request_serialize_with_filters() {
    let request = ListAddressBeneficiariesRequest {
        currency: Some("BTC".to_string()),
        address: Some("bc1q".to_string()),
        personal: Some(false),
        unhosted: Some(true),
        limit: Some(50),
        continuation: Some("token123".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"currency\":\"BTC\""));
    assert!(json.contains("\"personal\":false"));
    assert!(json.contains("\"unhosted\":true"));
    assert!(json.contains("\"limit\":50"));
    assert!(json.contains("\"continuation\":\"token123\""));
}

#[test]
fn test_list_response_deserialize() {
    let json = r#"{
        "data": [
            {
                "currency": "BTC",
                "address": "bc1qtest1",
                "agreed": true,
                "personal": false,
                "unhosted": false
            },
            {
                "currency": "BTC",
                "address": "bc1qtest2",
                "agreed": true,
                "personal": true,
                "unhosted": false
            }
        ],
        "continuation": "xY7T6cutS3t2B9YtaDkE6TS379oKnkzTvmEDUnEUP2Msa9xKWNNaT",
        "count": 2
    }"#;

    let response: ListAddressBeneficiariesResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.count, Some(2));
    assert_eq!(
        response.continuation,
        Some("xY7T6cutS3t2B9YtaDkE6TS379oKnkzTvmEDUnEUP2Msa9xKWNNaT".to_string())
    );
    assert_eq!(response.data[0].address, "bc1qtest1");
    assert_eq!(response.data[1].address, "bc1qtest2");
    assert!(!response.data[0].personal);
    assert!(response.data[1].personal);
}

#[test]
fn test_list_response_deserialize_empty() {
    let json = r#"{
        "data": [],
        "count": 0
    }"#;

    let response: ListAddressBeneficiariesResponse = serde_json::from_str(json).unwrap();
    assert!(response.data.is_empty());
    assert_eq!(response.count, Some(0));
    assert!(response.continuation.is_none());
}

#[test]
fn test_deposit_id_serialize() {
    let deposit_id = DepositId {
        currency: "BTC".to_string(),
        user_id: 123,
        address: "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz".to_string(),
        tx_hash: "230669110fdaf0a0dbcdc079b6b8b43d5af29cc73683835b9bc6b3406c065fda".to_string(),
    };

    let json = serde_json::to_string(&deposit_id).unwrap();
    assert!(json.contains("\"currency\":\"BTC\""));
    assert!(json.contains("\"user_id\":123"));
    assert!(json.contains("\"address\":\"2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz\""));
    assert!(json.contains(
        "\"tx_hash\":\"230669110fdaf0a0dbcdc079b6b8b43d5af29cc73683835b9bc6b3406c065fda\""
    ));
}

#[test]
fn test_deposit_id_deserialize() {
    let json = r#"{
        "currency": "ETH",
        "user_id": 456,
        "address": "0x123456",
        "tx_hash": "0xabcdef"
    }"#;

    let deposit_id: DepositId = serde_json::from_str(json).unwrap();
    assert_eq!(deposit_id.currency, "ETH");
    assert_eq!(deposit_id.user_id, 456);
    assert_eq!(deposit_id.address, "0x123456");
    assert_eq!(deposit_id.tx_hash, "0xabcdef");
}

#[test]
fn test_originator_serialize_company() {
    let originator = Originator {
        is_personal: false,
        company_name: Some("Company Name".to_string()),
        first_name: None,
        last_name: None,
        address: "NL, Amsterdam, Street, 1".to_string(),
    };

    let json = serde_json::to_string(&originator).unwrap();
    assert!(json.contains("\"is_personal\":false"));
    assert!(json.contains("\"company_name\":\"Company Name\""));
    assert!(json.contains("\"address\":\"NL, Amsterdam, Street, 1\""));
    assert!(!json.contains("\"first_name\"")); // Should be skipped when None
    assert!(!json.contains("\"last_name\"")); // Should be skipped when None
}

#[test]
fn test_originator_serialize_person() {
    let originator = Originator {
        is_personal: true,
        company_name: None,
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        address: "US, New York, 123 Main St".to_string(),
    };

    let json = serde_json::to_string(&originator).unwrap();
    assert!(json.contains("\"is_personal\":true"));
    assert!(json.contains("\"first_name\":\"John\""));
    assert!(json.contains("\"last_name\":\"Doe\""));
    assert!(!json.contains("\"company_name\"")); // Should be skipped when None
}

#[test]
fn test_originator_deserialize() {
    let json = r#"{
        "is_personal": false,
        "company_name": "Test Corp",
        "first_name": "Jane",
        "last_name": "Smith",
        "address": "UK, London"
    }"#;

    let originator: Originator = serde_json::from_str(json).unwrap();
    assert!(!originator.is_personal);
    assert_eq!(originator.company_name, Some("Test Corp".to_string()));
    assert_eq!(originator.first_name, Some("Jane".to_string()));
    assert_eq!(originator.last_name, Some("Smith".to_string()));
    assert_eq!(originator.address, "UK, London");
}

#[test]
fn test_clearance_deposit_result_deserialize_full() {
    let json = r#"{
        "currency": "BTC",
        "user_id": 123,
        "address": "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz",
        "amount": 0.4,
        "state": "completed",
        "transaction_id": "230669110fdaf0a0dbcdc079b6b8b43d5af29cc73683835b9bc6b3406c065fda",
        "source_address": "A3BqqD5GRJ8wHy1PYyCXTe9ke5226Fha123",
        "received_timestamp": 1550574558607,
        "updated_timestamp": 1550574558807,
        "note": "Test note",
        "clearance_state": "in_progress",
        "refund_transaction_id": null
    }"#;

    let result: ClearanceDepositResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.currency, "BTC");
    assert_eq!(result.user_id, Some(123));
    assert_eq!(result.address, "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBz");
    let amount_diff = (result.amount - 0.4).abs();
    assert!(amount_diff < f64::EPSILON);
    assert_eq!(result.state, "completed");
    assert!(result.transaction_id.is_some());
    assert_eq!(result.note, Some("Test note".to_string()));
    assert_eq!(result.clearance_state, Some("in_progress".to_string()));
    assert_eq!(result.received_timestamp, Some(1550574558607));
    assert_eq!(result.updated_timestamp, Some(1550574558807));
}

#[test]
fn test_clearance_deposit_result_deserialize_minimal() {
    let json = r#"{
        "currency": "ETH",
        "address": "0x123",
        "amount": 1.5,
        "state": "pending"
    }"#;

    let result: ClearanceDepositResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.currency, "ETH");
    assert_eq!(result.address, "0x123");
    let amount_diff = (result.amount - 1.5).abs();
    assert!(amount_diff < f64::EPSILON);
    assert_eq!(result.state, "pending");
    assert!(result.user_id.is_none());
    assert!(result.clearance_state.is_none());
    assert!(result.note.is_none());
}

#[test]
fn test_clearance_states() {
    // Test all valid clearance states deserialize correctly
    let states = [
        "in_progress",
        "pending_admin_decision",
        "pending_user_input",
        "success",
        "failed",
        "cancelled",
        "refund_initiated",
        "refunded",
    ];

    for state in &states {
        let json = format!(
            r#"{{"currency": "BTC", "address": "test", "amount": 0.1, "state": "completed", "clearance_state": "{}"}}"#,
            state
        );
        let result: ClearanceDepositResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result.clearance_state, Some(state.to_string()));
    }
}

#[test]
fn test_deposit_states() {
    // Test all valid deposit states
    let states = ["pending", "completed", "rejected", "replaced"];

    for state in &states {
        let json = format!(
            r#"{{"currency": "BTC", "address": "test", "amount": 0.1, "state": "{}"}}"#,
            state
        );
        let result: ClearanceDepositResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result.state, *state);
    }
}
