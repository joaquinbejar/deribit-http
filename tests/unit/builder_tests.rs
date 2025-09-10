//! Unit tests for HTTP message builder

use deribit_http::message::builder::HttpMessageBuilder;

#[cfg(test)]
mod message_builder_tests {
    use super::*;

    #[test]
    fn test_http_message_builder_new() {
        let base_url = "https://test.deribit.com".to_string();
        let _builder = HttpMessageBuilder::new(base_url.clone());

        // Test passes if no panic occurs during construction
    }

    #[test]
    fn test_request_builder_access() {
        let base_url = "https://test.deribit.com".to_string();
        let builder = HttpMessageBuilder::new(base_url.clone());

        let _request_builder = builder.request_builder();
        // Test passes if no panic occurs during access
    }

    #[test]
    fn test_response_handler_access() {
        let base_url = "https://test.deribit.com".to_string();
        let builder = HttpMessageBuilder::new(base_url.clone());

        let _response_handler = builder.response_handler();
        // Test passes if no panic occurs during access
    }

    #[test]
    fn test_clone_trait() {
        let base_url = "https://test.deribit.com".to_string();
        let builder = HttpMessageBuilder::new(base_url.clone());
        let _cloned_builder = builder.clone();

        // Test passes if no panic occurs during cloning
    }

    #[test]
    fn test_serialize_deserialize() {
        let base_url = "https://test.deribit.com".to_string();
        let builder = HttpMessageBuilder::new(base_url.clone());

        // Test serialization
        let serialized = serde_json::to_string(&builder).unwrap();
        let _deserialized: HttpMessageBuilder = serde_json::from_str(&serialized).unwrap();

        // Test passes if serialization/deserialization completes without error
    }

    #[test]
    fn test_different_base_urls() {
        let testnet_url = "https://test.deribit.com".to_string();
        let mainnet_url = "https://www.deribit.com".to_string();

        let _testnet_builder = HttpMessageBuilder::new(testnet_url);
        let _mainnet_builder = HttpMessageBuilder::new(mainnet_url);

        // Test passes if builders can be created with different URLs
    }

    #[test]
    fn test_builder_components_independence() {
        let base_url = "https://test.deribit.com".to_string();
        let builder = HttpMessageBuilder::new(base_url.clone());

        let _request_builder = builder.request_builder();
        let _response_handler = builder.response_handler();

        // Test passes if both components are accessible independently
    }
}
