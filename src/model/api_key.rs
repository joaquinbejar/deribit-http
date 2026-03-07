//! API key management models and types

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// API key information returned by API key management endpoints.
///
/// Contains all details about an API key including credentials,
/// permissions, and configuration.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    /// Unique identifier for the API key
    pub id: u64,
    /// Client identifier used for authentication
    pub client_id: String,
    /// Client secret or MD5 fingerprint of public key used for authentication
    pub client_secret: String,
    /// API key name that can be displayed in transaction log
    pub name: String,
    /// Describes maximal access for tokens generated with this key.
    ///
    /// Possible values include combinations of:
    /// - `trade:[read, read_write, none]`
    /// - `wallet:[read, read_write, none]`
    /// - `account:[read, read_write, none]`
    /// - `block_trade:[read, read_write, none]`
    /// - `block_rfq:[read, read_write, none]`
    pub max_scope: String,
    /// Whether the API key is enabled and can be used for authentication
    pub enabled: bool,
    /// Whether this API key is the default (deprecated, will be removed)
    pub default: bool,
    /// Timestamp when the key was created or last modified, in milliseconds since Unix epoch
    pub timestamp: u64,
    /// List of enabled advanced on-key features.
    ///
    /// Available options:
    /// - `restricted_block_trades`: Limit block_trade read scope to trades made with this key
    /// - `block_trade_approval`: Block trades require additional user approval
    #[serde(default)]
    pub enabled_features: Vec<String>,
    /// List of IP addresses whitelisted for this key
    #[serde(default)]
    pub ip_whitelist: Option<Vec<String>>,
    /// PEM encoded public key (Ed25519/RSA) used for asymmetric signatures
    pub public_key: Option<String>,
}

/// Request parameters for creating a new API key.
///
/// # Example
///
/// ```rust
/// use deribit_http::model::CreateApiKeyRequest;
///
/// let request = CreateApiKeyRequest {
///     max_scope: "account:read trade:read_write".to_string(),
///     name: Some("my_trading_key".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct CreateApiKeyRequest {
    /// Describes maximal access for tokens generated with this key.
    ///
    /// Required. Possible values include combinations of:
    /// - `trade:[read, read_write, none]`
    /// - `wallet:[read, read_write, none]`
    /// - `account:[read, read_write, none]`
    /// - `block_trade:[read, read_write, none]`
    pub max_scope: String,
    /// Name of key (only letters, numbers and underscores; max 16 characters)
    pub name: Option<String>,
    /// ED25519 or RSA PEM encoded public key for asymmetric API key authentication
    pub public_key: Option<String>,
    /// List of enabled advanced on-key features.
    ///
    /// Available options:
    /// - `restricted_block_trades`
    /// - `block_trade_approval`
    pub enabled_features: Option<Vec<String>>,
}

/// Request parameters for editing an existing API key.
///
/// At least one optional parameter must be provided along with the required fields.
///
/// # Example
///
/// ```rust
/// use deribit_http::model::EditApiKeyRequest;
///
/// let request = EditApiKeyRequest {
///     id: 123,
///     max_scope: "account:read_write trade:read_write".to_string(),
///     name: Some("updated_key_name".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct EditApiKeyRequest {
    /// ID of the API key to edit
    pub id: u64,
    /// Describes maximal access for tokens generated with this key.
    ///
    /// Required. Possible values include combinations of:
    /// - `trade:[read, read_write, none]`
    /// - `wallet:[read, read_write, none]`
    /// - `account:[read, read_write, none]`
    /// - `block_trade:[read, read_write, none]`
    pub max_scope: String,
    /// Name of key (only letters, numbers and underscores; max 16 characters)
    pub name: Option<String>,
    /// Enable or disable the API key
    pub enabled: Option<bool>,
    /// List of enabled advanced on-key features
    pub enabled_features: Option<Vec<String>>,
    /// List of IP addresses to whitelist for this key
    pub ip_whitelist: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_info_deserialization() {
        let json = r#"{
            "timestamp": 1560238048714,
            "max_scope": "account:read block_trade:read_write trade:read wallet:none",
            "id": 5,
            "enabled": true,
            "enabled_features": [],
            "default": false,
            "public_key": "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAM7FWhKquNqLmTOV4hfYT5r3AjrYiORTT6Tn5HIfFNV8=\n-----END PUBLIC KEY-----",
            "client_secret": "9c:6d:c9:02:fd:9f:75:6e:14:bb:71:c5:74:95:86:c8",
            "client_id": "wcVoQGam",
            "name": ""
        }"#;

        let info: ApiKeyInfo = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(info.id, 5);
        assert_eq!(info.client_id, "wcVoQGam");
        assert!(info.enabled);
        assert!(!info.default);
        assert!(info.public_key.is_some());
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
        assert!(!keys[0].enabled);
        assert_eq!(keys[1].id, 2);
        assert!(keys[1].enabled);
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
    fn test_edit_api_key_request_default() {
        let request = EditApiKeyRequest::default();
        assert_eq!(request.id, 0);
        assert!(request.max_scope.is_empty());
        assert!(request.name.is_none());
        assert!(request.enabled.is_none());
    }
}
