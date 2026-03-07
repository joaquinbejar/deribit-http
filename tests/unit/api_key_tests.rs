//! Unit tests for API key management models

use deribit_http::model::{ApiKeyInfo, CreateApiKeyRequest, EditApiKeyRequest};

#[test]
fn test_api_key_info_deserialization_full() {
    let json = r#"{
        "timestamp": 1560238048714,
        "max_scope": "account:read block_trade:read_write trade:read wallet:none",
        "id": 5,
        "enabled": true,
        "enabled_features": ["restricted_block_trades"],
        "default": false,
        "public_key": "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAM7FWhKquNqLmTOV4hfYT5r3AjrYiORTT6Tn5HIfFNV8=\n-----END PUBLIC KEY-----",
        "client_secret": "9c:6d:c9:02:fd:9f:75:6e:14:bb:71:c5:74:95:86:c8",
        "client_id": "wcVoQGam",
        "name": "my_key",
        "ip_whitelist": ["192.168.1.1", "10.0.0.1"]
    }"#;

    let info: ApiKeyInfo = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(info.id, 5);
    assert_eq!(info.client_id, "wcVoQGam");
    assert_eq!(
        info.client_secret,
        "9c:6d:c9:02:fd:9f:75:6e:14:bb:71:c5:74:95:86:c8"
    );
    assert_eq!(info.name, "my_key");
    assert_eq!(
        info.max_scope,
        "account:read block_trade:read_write trade:read wallet:none"
    );
    assert!(info.enabled);
    assert!(!info.default);
    assert_eq!(info.timestamp, 1560238048714);
    assert_eq!(info.enabled_features.len(), 1);
    assert_eq!(info.enabled_features[0], "restricted_block_trades");
    assert!(info.public_key.is_some());
    assert!(info.ip_whitelist.is_some());
    assert_eq!(info.ip_whitelist.as_ref().map(|v| v.len()), Some(2));
}

#[test]
fn test_api_key_info_deserialization_minimal() {
    let json = r#"{
        "timestamp": 1560242676023,
        "max_scope": "account:read_write block_trade:read trade:read_write wallet:read_write",
        "id": 3,
        "enabled": false,
        "default": false,
        "client_secret": "B6RsF9rrLY5ezEGBQkyLlV-UC7whyPJ34BMA-kKYpes",
        "client_id": "1sXMQBhM",
        "name": ""
    }"#;

    let info: ApiKeyInfo = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(info.id, 3);
    assert_eq!(info.client_id, "1sXMQBhM");
    assert!(!info.enabled);
    assert!(info.name.is_empty());
    assert!(info.enabled_features.is_empty());
    assert!(info.ip_whitelist.is_none());
    assert!(info.public_key.is_none());
}

#[test]
fn test_api_key_info_list_deserialization() {
    let json = r#"[
        {
            "timestamp": 1560236001108,
            "max_scope": "account:read block_trade:read trade:read_write wallet:read",
            "id": 1,
            "enabled": false,
            "default": false,
            "client_secret": "SjM57m1T2CfXZ4vZ76X1APjqRlJdtzHI8IwVXoQnfoM",
            "client_id": "TiA4AyLPq3",
            "name": "",
            "enabled_features": []
        },
        {
            "timestamp": 1560236287708,
            "max_scope": "account:read_write block_trade:read_write trade:read_write wallet:read_write",
            "id": 2,
            "enabled": true,
            "default": true,
            "client_secret": "mwNOvbUVyQczytQ5IVM8CbzmgqNJ81WvLKfu6MXcJPs",
            "client_id": "aD-KFx-H",
            "name": "",
            "enabled_features": []
        }
    ]"#;

    let keys: Vec<ApiKeyInfo> = serde_json::from_str(json).expect("Failed to deserialize");
    assert_eq!(keys.len(), 2);

    assert_eq!(keys[0].id, 1);
    assert_eq!(keys[0].client_id, "TiA4AyLPq3");
    assert!(!keys[0].enabled);
    assert!(!keys[0].default);

    assert_eq!(keys[1].id, 2);
    assert_eq!(keys[1].client_id, "aD-KFx-H");
    assert!(keys[1].enabled);
    assert!(keys[1].default);
}

#[test]
fn test_api_key_info_serialization() {
    let info = ApiKeyInfo {
        id: 1,
        client_id: "test_client".to_string(),
        client_secret: "test_secret".to_string(),
        name: "test_key".to_string(),
        max_scope: "account:read".to_string(),
        enabled: true,
        default: false,
        timestamp: 1234567890,
        enabled_features: vec!["block_trade_approval".to_string()],
        ip_whitelist: Some(vec!["127.0.0.1".to_string()]),
        public_key: None,
    };

    let json = serde_json::to_string(&info).expect("Failed to serialize");
    assert!(json.contains("\"id\":1"));
    assert!(json.contains("\"client_id\":\"test_client\""));
    assert!(json.contains("\"enabled\":true"));
    assert!(json.contains("\"block_trade_approval\""));
}

#[test]
fn test_create_api_key_request_default() {
    let request = CreateApiKeyRequest::default();
    assert!(request.max_scope.is_empty());
    assert!(request.name.is_none());
    assert!(request.public_key.is_none());
    assert!(request.enabled_features.is_none());
}

#[test]
fn test_create_api_key_request_with_values() {
    let request = CreateApiKeyRequest {
        max_scope: "account:read trade:read_write".to_string(),
        name: Some("my_trading_key".to_string()),
        public_key: Some("-----BEGIN PUBLIC KEY-----\ntest\n-----END PUBLIC KEY-----".to_string()),
        enabled_features: Some(vec!["restricted_block_trades".to_string()]),
    };

    assert_eq!(request.max_scope, "account:read trade:read_write");
    assert_eq!(request.name, Some("my_trading_key".to_string()));
    assert!(request.public_key.is_some());
    assert!(request.enabled_features.is_some());
    assert_eq!(request.enabled_features.as_ref().map(|v| v.len()), Some(1));
}

#[test]
fn test_edit_api_key_request_default() {
    let request = EditApiKeyRequest::default();
    assert_eq!(request.id, 0);
    assert!(request.max_scope.is_empty());
    assert!(request.name.is_none());
    assert!(request.enabled.is_none());
    assert!(request.enabled_features.is_none());
    assert!(request.ip_whitelist.is_none());
}

#[test]
fn test_edit_api_key_request_with_values() {
    let request = EditApiKeyRequest {
        id: 123,
        max_scope: "account:read_write wallet:read".to_string(),
        name: Some("updated_name".to_string()),
        enabled: Some(false),
        enabled_features: Some(vec!["block_trade_approval".to_string()]),
        ip_whitelist: Some(vec!["10.0.0.1".to_string(), "192.168.1.1".to_string()]),
    };

    assert_eq!(request.id, 123);
    assert_eq!(request.max_scope, "account:read_write wallet:read");
    assert_eq!(request.name, Some("updated_name".to_string()));
    assert_eq!(request.enabled, Some(false));
    assert!(request.enabled_features.is_some());
    assert!(request.ip_whitelist.is_some());
    assert_eq!(request.ip_whitelist.as_ref().map(|v| v.len()), Some(2));
}

#[test]
fn test_api_key_info_with_empty_enabled_features() {
    let json = r#"{
        "timestamp": 1560238048714,
        "max_scope": "account:read",
        "id": 1,
        "enabled": true,
        "enabled_features": [],
        "default": false,
        "client_secret": "secret",
        "client_id": "client",
        "name": "key"
    }"#;

    let info: ApiKeyInfo = serde_json::from_str(json).expect("Failed to deserialize");
    assert!(info.enabled_features.is_empty());
}

#[test]
fn test_api_key_info_without_optional_fields() {
    // Test deserialization when optional fields are missing entirely
    let json = r#"{
        "timestamp": 1560238048714,
        "max_scope": "account:read",
        "id": 1,
        "enabled": true,
        "default": false,
        "client_secret": "secret",
        "client_id": "client",
        "name": "key"
    }"#;

    let info: ApiKeyInfo = serde_json::from_str(json).expect("Failed to deserialize");
    assert!(info.enabled_features.is_empty());
    assert!(info.ip_whitelist.is_none());
    assert!(info.public_key.is_none());
}
