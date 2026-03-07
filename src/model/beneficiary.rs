//! Address beneficiary models for wallet endpoints

use serde::{Deserialize, Serialize};

/// Address beneficiary information returned by save/get/list operations.
///
/// Contains all information about a beneficiary associated with
/// a cryptocurrency address for travel rule compliance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressBeneficiary {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub currency: String,
    /// Address in proper format for the currency
    pub address: String,
    /// User ID associated with this beneficiary
    #[serde(default)]
    pub user_id: Option<u64>,
    /// Whether the user agreed to share information with third parties
    pub agreed: bool,
    /// Whether this is a personal wallet owned by the user
    pub personal: bool,
    /// Whether the address belongs to an unhosted wallet
    pub unhosted: bool,
    /// Name of the beneficiary VASP (Virtual Asset Service Provider)
    #[serde(default)]
    pub beneficiary_vasp_name: Option<String>,
    /// DID (Decentralized Identifier) of the beneficiary VASP
    #[serde(default)]
    pub beneficiary_vasp_did: Option<String>,
    /// Website of the beneficiary VASP
    #[serde(default)]
    pub beneficiary_vasp_website: Option<String>,
    /// First name of the beneficiary (if a person)
    #[serde(default)]
    pub beneficiary_first_name: Option<String>,
    /// Last name of the beneficiary (if a person)
    #[serde(default)]
    pub beneficiary_last_name: Option<String>,
    /// Company name of the beneficiary (if a company)
    #[serde(default)]
    pub beneficiary_company_name: Option<String>,
    /// Geographical address of the beneficiary
    #[serde(default)]
    pub beneficiary_address: Option<String>,
    /// Tag for XRP addresses (optional)
    #[serde(default)]
    pub tag: Option<String>,
    /// Creation timestamp in milliseconds since Unix epoch
    #[serde(default, alias = "created")]
    pub creation_timestamp: Option<u64>,
    /// Update timestamp in milliseconds since Unix epoch
    #[serde(default, alias = "updated")]
    pub update_timestamp: Option<u64>,
}

/// Request parameters for saving an address beneficiary.
///
/// All fields required by the API are marked as non-optional.
/// Optional fields use `Option<T>`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SaveAddressBeneficiaryRequest {
    /// Currency symbol (required)
    pub currency: String,
    /// Address in currency format (required)
    pub address: String,
    /// Tag for XRP addresses (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// User agrees to share information with third parties (required)
    pub agreed: bool,
    /// Whether this is a personal wallet (required)
    pub personal: bool,
    /// Whether the address belongs to an unhosted wallet (required)
    pub unhosted: bool,
    /// Name of the beneficiary VASP (required)
    pub beneficiary_vasp_name: String,
    /// DID of the beneficiary VASP (required)
    pub beneficiary_vasp_did: String,
    /// Website of the beneficiary VASP (optional, required if VASP not in known list)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_website: Option<String>,
    /// First name of the beneficiary (optional, for persons)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_first_name: Option<String>,
    /// Last name of the beneficiary (optional, for persons)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_last_name: Option<String>,
    /// Company name of the beneficiary (optional, for companies)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_company_name: Option<String>,
    /// Geographical address of the beneficiary (required)
    pub beneficiary_address: String,
}

/// Request parameters for listing address beneficiaries with filtering and pagination.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListAddressBeneficiariesRequest {
    /// Filter by currency symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Filter by address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Tag for XRP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Filter by creation timestamp (before), in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<u64>,
    /// Filter by creation timestamp (after), in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<u64>,
    /// Filter by update timestamp (before), in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_before: Option<u64>,
    /// Filter by update timestamp (after), in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<u64>,
    /// Filter by personal wallet flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
    /// Filter by unhosted wallet flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhosted: Option<bool>,
    /// Filter by beneficiary VASP name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_name: Option<String>,
    /// Filter by beneficiary VASP DID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_did: Option<String>,
    /// Filter by beneficiary VASP website
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_website: Option<String>,
    /// Maximum number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

/// Paginated response for listing address beneficiaries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListAddressBeneficiariesResponse {
    /// List of address beneficiaries
    pub data: Vec<AddressBeneficiary>,
    /// Total count of results available
    #[serde(default)]
    pub count: Option<u64>,
    /// Continuation token for fetching the next page
    #[serde(default)]
    pub continuation: Option<String>,
}

/// Deposit identifier for `set_clearance_originator`.
///
/// Identifies a specific deposit transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositId {
    /// Currency symbol
    pub currency: String,
    /// User ID of the (sub)account
    pub user_id: u64,
    /// Deposit address in currency format
    pub address: String,
    /// Transaction hash in proper format for the currency
    pub tx_hash: String,
}

/// Originator information for `set_clearance_originator`.
///
/// Contains details about the originator of a deposit for travel rule compliance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Originator {
    /// Whether the user is the originator of the deposit
    pub is_personal: bool,
    /// Company name (if originator is a legal entity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// First name (if originator is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name (if originator is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Geographical address of the originator
    pub address: String,
}

/// Result of `set_clearance_originator` operation.
///
/// Contains the deposit information with updated clearance state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearanceDepositResult {
    /// Currency symbol
    pub currency: String,
    /// User ID
    #[serde(default)]
    pub user_id: Option<u64>,
    /// Deposit address
    pub address: String,
    /// Amount of funds in the currency
    pub amount: f64,
    /// Deposit state: "pending", "completed", "rejected", "replaced"
    pub state: String,
    /// Transaction ID in proper format for the currency
    #[serde(default)]
    pub transaction_id: Option<String>,
    /// Source address of the deposit
    #[serde(default)]
    pub source_address: Option<String>,
    /// Timestamp when deposit was received, in milliseconds
    #[serde(default)]
    pub received_timestamp: Option<u64>,
    /// Timestamp when deposit was last updated, in milliseconds
    #[serde(default)]
    pub updated_timestamp: Option<u64>,
    /// Optional note
    #[serde(default)]
    pub note: Option<String>,
    /// Clearance state: "in_progress", "pending_admin_decision", "pending_user_input",
    /// "success", "failed", "cancelled", "refund_initiated", "refunded"
    #[serde(default)]
    pub clearance_state: Option<String>,
    /// Refund transaction ID if applicable
    #[serde(default)]
    pub refund_transaction_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_beneficiary_deserialize() {
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
        assert!(beneficiary.agreed);
        assert!(!beneficiary.personal);
        assert!(!beneficiary.unhosted);
        assert_eq!(
            beneficiary.beneficiary_vasp_name,
            Some("Money's Gone".to_string())
        );
        assert_eq!(beneficiary.beneficiary_first_name, Some("John".to_string()));
        assert_eq!(beneficiary.creation_timestamp, Some(1536569522277));
    }

    #[test]
    fn test_save_request_serialize() {
        let request = SaveAddressBeneficiaryRequest {
            currency: "BTC".to_string(),
            address: "bc1qtest".to_string(),
            tag: None,
            agreed: true,
            personal: false,
            unhosted: false,
            beneficiary_vasp_name: "Test VASP".to_string(),
            beneficiary_vasp_did: "did:test:123".to_string(),
            beneficiary_vasp_website: Some("https://test.com".to_string()),
            beneficiary_first_name: Some("John".to_string()),
            beneficiary_last_name: Some("Doe".to_string()),
            beneficiary_company_name: None,
            beneficiary_address: "Test Address".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"currency\":\"BTC\""));
        assert!(json.contains("\"agreed\":true"));
        assert!(!json.contains("\"tag\"")); // Should be skipped when None
        assert!(!json.contains("beneficiary_company_name")); // Should be skipped when None
    }

    #[test]
    fn test_list_response_deserialize() {
        let json = r#"{
            "data": [
                {
                    "currency": "BTC",
                    "address": "bc1qtest",
                    "agreed": true,
                    "personal": false,
                    "unhosted": false
                }
            ],
            "continuation": "xY7T6cutS3t2B9YtaDkE6TS379oKnkzTvmEDUnEUP2Msa9xKWNNaT",
            "count": 1
        }"#;

        let response: ListAddressBeneficiariesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.count, Some(1));
        assert!(response.continuation.is_some());
    }

    #[test]
    fn test_clearance_deposit_result_deserialize() {
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
            "note": "Note",
            "clearance_state": "in_progress"
        }"#;

        let result: ClearanceDepositResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.currency, "BTC");
        assert_eq!(result.user_id, Some(123));
        let amount_diff = (result.amount - 0.4).abs();
        assert!(amount_diff < f64::EPSILON);
        assert_eq!(result.state, "completed");
        assert_eq!(result.clearance_state, Some("in_progress".to_string()));
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
        assert!(json.contains("\"tx_hash\":"));
    }

    #[test]
    fn test_originator_serialize() {
        let originator = Originator {
            is_personal: false,
            company_name: Some("Company Name".to_string()),
            first_name: Some("First".to_string()),
            last_name: Some("Last".to_string()),
            address: "NL, Amsterdam, Street, 1".to_string(),
        };

        let json = serde_json::to_string(&originator).unwrap();
        assert!(json.contains("\"is_personal\":false"));
        assert!(json.contains("\"company_name\":\"Company Name\""));
        assert!(json.contains("\"address\":\"NL, Amsterdam, Street, 1\""));
    }

    #[test]
    fn test_list_request_default() {
        let request = ListAddressBeneficiariesRequest::default();
        assert!(request.currency.is_none());
        assert!(request.limit.is_none());
        assert!(request.continuation.is_none());
    }
}
