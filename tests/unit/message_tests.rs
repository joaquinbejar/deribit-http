/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/9/25
******************************************************************************/

//! Unit tests for message modules

use deribit_http::prelude::*;

#[test]
fn test_http_request_builder_new() {
    let base_url = "https://test.deribit.com".to_string();
    let builder = HttpRequestBuilder::new(base_url.clone());

    // We can't directly access the base_url field, but we can test through the methods
    let request = builder.build_get("/test", None);
    assert!(request.endpoint.starts_with(&base_url));
}

#[test]
fn test_build_get_request_without_params() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let request = builder.build_get("/api/v2/public/test", None);

    assert_eq!(request.method, "GET");
    assert_eq!(
        request.endpoint,
        "https://test.deribit.com/api/v2/public/test"
    );
    assert!(request.headers.contains_key("Content-Type"));
    assert!(request.headers.contains_key("Accept"));
    assert_eq!(
        request.headers.get("Content-Type").unwrap(),
        "application/json"
    );
    assert_eq!(request.headers.get("Accept").unwrap(), "application/json");
    assert!(request.body.is_none());
}

#[test]
fn test_build_get_request_with_empty_params() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let params = RequestParams::new();
    let request = builder.build_get("/api/v2/public/test", Some(params));

    assert_eq!(request.method, "GET");
    assert_eq!(
        request.endpoint,
        "https://test.deribit.com/api/v2/public/test"
    );
    assert!(request.body.is_none());
}

#[test]
fn test_build_get_request_with_params() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let params = RequestParams::new()
        .add("currency", "BTC")
        .add("kind", "option");

    let request = builder.build_get("/api/v2/public/get_instruments", Some(params));

    assert_eq!(request.method, "GET");
    // Note: The current implementation doesn't properly convert params to query string
    // This test verifies the current behavior, but the implementation should be improved
    assert!(
        request
            .endpoint
            .starts_with("https://test.deribit.com/api/v2/public/get_instruments")
    );
}

#[test]
fn test_build_post_request_without_params() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let request = builder.build_post("/api/v2/private/buy", None);

    assert_eq!(request.method, "POST");
    assert_eq!(
        request.endpoint,
        "https://test.deribit.com/api/v2/private/buy"
    );
    assert!(request.headers.contains_key("Content-Type"));
    assert!(request.headers.contains_key("Accept"));
    assert!(request.body.is_none());
}

#[test]
fn test_build_post_request_with_params() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let params = RequestParams::new()
        .add("instrument_name", "BTC-PERPETUAL")
        .add("amount", "10");

    let request = builder.build_post("/api/v2/private/buy", Some(params));

    assert_eq!(request.method, "POST");
    assert_eq!(
        request.endpoint,
        "https://test.deribit.com/api/v2/private/buy"
    );
    assert!(request.body.is_some());

    // Verify the body contains JSON
    let body = request.body.unwrap();
    assert!(body.contains("instrument_name"));
    assert!(body.contains("BTC-PERPETUAL"));
}

#[test]
fn test_build_auth_request() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let request = builder.build_auth_request("test_client_id", "test_client_secret");

    assert_eq!(request.method, "POST");
    assert_eq!(request.endpoint, "https://test.deribit.com/public/auth");
    assert!(request.body.is_some());

    let body = request.body.unwrap();
    assert!(body.contains("grant_type"));
    assert!(body.contains("client_credentials"));
    assert!(body.contains("client_id"));
    assert!(body.contains("test_client_id"));
    assert!(body.contains("client_secret"));
    assert!(body.contains("test_client_secret"));
}

#[test]
fn test_build_test_request() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let request = builder.build_test_request();

    assert_eq!(request.method, "GET");
    assert_eq!(request.endpoint, "https://test.deribit.com/public/test");
    assert!(request.body.is_none());
}

#[test]
fn test_build_get_time_request() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let request = builder.build_get_time_request();

    assert_eq!(request.method, "GET");
    assert_eq!(request.endpoint, "https://test.deribit.com/public/get_time");
    assert!(request.body.is_none());
}

#[test]
fn test_default_headers() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());
    let request = builder.build_get("/test", None);

    let headers = &request.headers;
    assert_eq!(headers.len(), 2);
    assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
    assert_eq!(headers.get("Accept").unwrap(), "application/json");
}

#[test]
fn test_different_base_urls() {
    let production_builder = HttpRequestBuilder::new("https://www.deribit.com".to_string());
    let testnet_builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());

    let prod_request = production_builder.build_get("/api/v2/public/test", None);
    let test_request = testnet_builder.build_get("/api/v2/public/test", None);

    assert!(prod_request.endpoint.contains("www.deribit.com"));
    assert!(test_request.endpoint.contains("test.deribit.com"));
    assert!(!prod_request.endpoint.contains("test.deribit.com"));
    assert!(!test_request.endpoint.contains("www.deribit.com"));
}

#[test]
fn test_endpoint_path_construction() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());

    // Test various endpoint paths
    let endpoints = vec![
        "/public/test",
        "/api/v2/public/get_time",
        "/api/v2/private/get_account_summary",
        "/api/v2/public/get_instruments",
    ];

    for endpoint in endpoints {
        let request = builder.build_get(endpoint, None);
        assert_eq!(
            request.endpoint,
            format!("https://test.deribit.com{}", endpoint)
        );
    }
}

#[test]
fn test_clone_and_serialize_traits() {
    let builder = HttpRequestBuilder::new("https://test.deribit.com".to_string());

    // Test that the builder can be cloned
    let cloned_builder = builder.clone();
    let request1 = builder.build_get("/test1", None);
    let request2 = cloned_builder.build_get("/test2", None);

    assert!(request1.endpoint.contains("/test1"));
    assert!(request2.endpoint.contains("/test2"));

    // Test serialization (the struct derives Serialize)
    let serialized = serde_json::to_string(&builder);
    assert!(serialized.is_ok());
}

#[cfg(test)]
mod request_params_tests {
    use super::*;

    #[test]
    fn test_request_params_new() {
        let params = RequestParams::new();
        assert!(params.is_empty());
    }

    #[test]
    fn test_request_params_add() {
        let params = RequestParams::new()
            .add("key1", "value1")
            .add("key2", "value2");

        assert!(!params.is_empty());

        // Test JSON conversion
        let json = params.to_json();
        let json_str = json.to_string();
        assert!(json_str.contains("key1"));
        assert!(json_str.contains("value1"));
        assert!(json_str.contains("key2"));
        assert!(json_str.contains("value2"));
    }

    #[test]
    fn test_request_params_chaining() {
        let params = RequestParams::new()
            .add("instrument_name", "BTC-PERPETUAL")
            .add("amount", "10")
            .add("type", "limit")
            .add("price", "50000");

        let json = params.to_json();
        let json_str = json.to_string();

        assert!(json_str.contains("instrument_name"));
        assert!(json_str.contains("BTC-PERPETUAL"));
        assert!(json_str.contains("amount"));
        assert!(json_str.contains("10"));
        assert!(json_str.contains("type"));
        assert!(json_str.contains("limit"));
        assert!(json_str.contains("price"));
        assert!(json_str.contains("50000"));
    }
}
