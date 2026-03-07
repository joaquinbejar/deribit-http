//! Unit tests for config module

use deribit_http::config::{ApiCredentials, HttpConfig};
use std::time::Duration;

#[test]
fn test_http_config_default() {
    let config = HttpConfig::default();

    assert!(config.testnet);
    assert!(config.base_url.as_str().contains("test.deribit.com"));
}

#[test]
fn test_http_config_testnet() {
    let config = HttpConfig::testnet();

    assert!(config.testnet);
    assert!(config.base_url.as_str().contains("test.deribit.com"));
}

#[test]
fn test_http_config_production() {
    let config = HttpConfig::production();

    assert!(!config.testnet);
    assert!(config.base_url.as_str().contains("www.deribit.com"));
}

#[test]
fn test_http_config_with_oauth2() {
    let config = HttpConfig::testnet().with_oauth2(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
    );

    assert!(config.has_credentials());
    let creds = config.credentials().unwrap();
    assert_eq!(creds.client_id, Some("test_client_id".to_string()));
    assert_eq!(creds.client_secret, Some("test_client_secret".to_string()));
}

#[test]
fn test_http_config_with_timeout() {
    let config = HttpConfig::testnet().with_timeout(Duration::from_secs(60));

    assert_eq!(config.timeout, Duration::from_secs(60));
}

#[test]
fn test_http_config_with_user_agent() {
    let config = HttpConfig::testnet().with_user_agent("MyBot/1.0".to_string());

    assert_eq!(config.user_agent, "MyBot/1.0");
}

#[test]
fn test_http_config_credentials_method() {
    // Note: has_credentials() may return true if env vars are set
    let config = HttpConfig::testnet();
    // Just test that the method works without panicking
    let _ = config.has_credentials();
    let _ = config.credentials();
}

#[test]
fn test_http_config_has_credentials_true() {
    let config = HttpConfig::testnet().with_oauth2("id".to_string(), "secret".to_string());
    assert!(config.has_credentials());
}

#[test]
fn test_api_credentials_clone() {
    let creds = ApiCredentials {
        client_id: Some("client_123".to_string()),
        client_secret: Some("secret_456".to_string()),
    };
    let cloned = creds.clone();

    assert_eq!(creds.client_id, cloned.client_id);
    assert_eq!(creds.client_secret, cloned.client_secret);
}

#[test]
fn test_http_config_chain_methods() {
    let config = HttpConfig::testnet()
        .with_timeout(Duration::from_secs(45))
        .with_user_agent("TestBot/2.0".to_string())
        .with_oauth2("my_id".to_string(), "my_secret".to_string());

    assert!(config.testnet);
    assert_eq!(config.timeout, Duration::from_secs(45));
    assert_eq!(config.user_agent, "TestBot/2.0");
    assert!(config.has_credentials());
}

#[test]
fn test_http_config_with_max_retries() {
    let config = HttpConfig::testnet().with_max_retries(5);

    assert_eq!(config.max_retries, 5);
}

#[test]
fn test_http_config_base_url_testnet() {
    let config = HttpConfig::testnet();
    assert!(config.base_url.as_str().contains("test.deribit.com"));
}

#[test]
fn test_http_config_base_url_production() {
    let config = HttpConfig::production();
    assert!(config.base_url.as_str().contains("www.deribit.com"));
}
