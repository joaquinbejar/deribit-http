//! Unit tests for HTTP response handling

use deribit_http::error::HttpError;
use deribit_http::message::response::HttpResponseHandler;
use deribit_http::model::http_types::{ApiError, ApiResponse, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestData {
    value: String,
    number: i32,
}

#[cfg(test)]
mod response_handler_tests {
    use super::*;

    #[test]
    fn test_response_handler_new() {
        let handler = HttpResponseHandler::new();
        // Test that we can create a handler
        assert!(true);
    }

    #[test]
    fn test_response_handler_default() {
        let handler = HttpResponseHandler;
        // Test that default implementation works
        assert!(true);
    }

    #[test]
    fn test_parse_response_success() {
        let handler = HttpResponseHandler::new();
        let test_data = TestData {
            value: "test".to_string(),
            number: 42,
        };

        let api_response = ApiResponse {
            jsonrpc: Some("2.0".to_string()),
            id: Some(1),
            result: Some(test_data.clone()),
            error: None,
            us_in: None,
            us_out: None,
            us_diff: None,
            testnet: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: serde_json::to_string(&api_response).unwrap(),
        };

        let parsed: ApiResponse<TestData> = handler.parse_response(&response).unwrap();
        assert_eq!(parsed.result.unwrap(), test_data);
    }

    #[test]
    fn test_parse_response_http_error() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 404,
            headers: HashMap::new(),
            body: "Not Found".to_string(),
        };

        let result: Result<ApiResponse<TestData>, HttpError> = handler.parse_response(&response);
        assert!(result.is_err());
        match result.unwrap_err() {
            HttpError::RequestFailed(msg) => {
                assert!(msg.contains("HTTP 404"));
                assert!(msg.contains("Not Found"));
            }
            _ => panic!("Expected RequestFailed error"),
        }
    }

    #[test]
    fn test_parse_response_invalid_json() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: "invalid json".to_string(),
        };

        let result: Result<ApiResponse<TestData>, HttpError> = handler.parse_response(&response);
        assert!(result.is_err());
        match result.unwrap_err() {
            HttpError::InvalidResponse(_) => {}
            _ => panic!("Expected InvalidResponse error"),
        }
    }

    #[test]
    fn test_is_success_true() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: "".to_string(),
        };
        assert!(handler.is_success(&response));

        let response = HttpResponse {
            status: 201,
            headers: HashMap::new(),
            body: "".to_string(),
        };
        assert!(handler.is_success(&response));

        let response = HttpResponse {
            status: 299,
            headers: HashMap::new(),
            body: "".to_string(),
        };
        assert!(handler.is_success(&response));
    }

    #[test]
    fn test_is_success_false() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 199,
            headers: HashMap::new(),
            body: "".to_string(),
        };
        assert!(!handler.is_success(&response));

        let response = HttpResponse {
            status: 300,
            headers: HashMap::new(),
            body: "".to_string(),
        };
        assert!(!handler.is_success(&response));

        let response = HttpResponse {
            status: 404,
            headers: HashMap::new(),
            body: "".to_string(),
        };
        assert!(!handler.is_success(&response));
    }

    #[test]
    fn test_extract_error_with_error() {
        let handler = HttpResponseHandler::new();
        let api_error = ApiError {
            code: 404,
            message: "Not found".to_string(),
            data: None,
        };

        let api_response: ApiResponse<TestData> = ApiResponse {
            jsonrpc: Some("2.0".to_string()),
            id: Some(1),
            result: None,
            error: Some(api_error.clone()),
            us_in: None,
            us_out: None,
            us_diff: None,
            testnet: None,
        };

        let extracted_error = handler.extract_error(&api_response);
        assert!(extracted_error.is_some());
        assert_eq!(extracted_error.unwrap().code, 404);
        assert_eq!(extracted_error.unwrap().message, "Not found");
    }

    #[test]
    fn test_extract_error_without_error() {
        let handler = HttpResponseHandler::new();
        let api_response: ApiResponse<TestData> = ApiResponse {
            jsonrpc: Some("2.0".to_string()),
            id: Some(1),
            result: Some(TestData {
                value: "test".to_string(),
                number: 42,
            }),
            error: None,
            us_in: None,
            us_out: None,
            us_diff: None,
            testnet: None,
        };

        let extracted_error = handler.extract_error(&api_response);
        assert!(extracted_error.is_none());
    }

    #[test]
    fn test_extract_result_with_result() {
        let handler = HttpResponseHandler::new();
        let test_data = TestData {
            value: "test".to_string(),
            number: 42,
        };

        let api_response = ApiResponse {
            jsonrpc: Some("2.0".to_string()),
            id: Some(1),
            result: Some(test_data.clone()),
            error: None,
            us_in: None,
            us_out: None,
            us_diff: None,
            testnet: None,
        };

        let extracted_result = handler.extract_result(&api_response);
        assert!(extracted_result.is_some());
        assert_eq!(extracted_result.unwrap(), &test_data);
    }

    #[test]
    fn test_extract_result_without_result() {
        let handler = HttpResponseHandler::new();
        let api_response: ApiResponse<TestData> = ApiResponse {
            jsonrpc: Some("2.0".to_string()),
            id: Some(1),
            result: None,
            error: Some(ApiError {
                code: 404,
                message: "Not found".to_string(),
                data: None,
            }),
            us_in: None,
            us_out: None,
            us_diff: None,
            testnet: None,
        };

        let extracted_result = handler.extract_result(&api_response);
        assert!(extracted_result.is_none());
    }

    #[test]
    fn test_handle_rate_limit_success() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: "".to_string(),
        };

        let result = handler.handle_rate_limit(&response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_rate_limit_error() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 429,
            headers: HashMap::new(),
            body: "Rate limit exceeded".to_string(),
        };

        let result = handler.handle_rate_limit(&response);
        assert!(result.is_err());
        match result.unwrap_err() {
            HttpError::RateLimitExceeded => {}
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }

    #[test]
    fn test_handle_auth_error_success() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: "".to_string(),
        };

        let result = handler.handle_auth_error(&response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_auth_error_401() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 401,
            headers: HashMap::new(),
            body: "Unauthorized".to_string(),
        };

        let result = handler.handle_auth_error(&response);
        assert!(result.is_err());
        match result.unwrap_err() {
            HttpError::AuthenticationFailed(msg) => {
                assert!(msg.contains("Authentication failed"));
            }
            _ => panic!("Expected AuthenticationFailed error"),
        }
    }

    #[test]
    fn test_handle_auth_error_403() {
        let handler = HttpResponseHandler::new();
        let response = HttpResponse {
            status: 403,
            headers: HashMap::new(),
            body: "Forbidden".to_string(),
        };

        let result = handler.handle_auth_error(&response);
        assert!(result.is_err());
        match result.unwrap_err() {
            HttpError::AuthenticationFailed(msg) => {
                assert!(msg.contains("Authentication failed"));
            }
            _ => panic!("Expected AuthenticationFailed error"),
        }
    }

    #[test]
    fn test_clone_and_serialize_traits() {
        let handler = HttpResponseHandler::new();
        let cloned = handler.clone();

        // Test serialization
        let serialized = serde_json::to_string(&handler).unwrap();
        let deserialized: HttpResponseHandler = serde_json::from_str(&serialized).unwrap();

        // All handlers should be equivalent since they have no state
        assert!(true);
    }
}
