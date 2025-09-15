use deribit_http::model::withdrawal::*;
use serde_json;

// Helper function to create a mock withdrawal priority
fn create_mock_withdrawal_priority() -> WithdrawalPriority {
    WithdrawalPriority::new("custom".to_string(), 0.75)
}

#[cfg(test)]
mod withdrawal_priority_tests {
    use super::*;

    #[test]
    fn test_withdrawal_priority_creation() {
        let priority = create_mock_withdrawal_priority();
        assert_eq!(priority.name, "custom");
        assert_eq!(priority.value, 0.75);
    }

    #[test]
    fn test_withdrawal_priority_new() {
        let priority = WithdrawalPriority::new("test".to_string(), 2.0);
        assert_eq!(priority.name, "test");
        assert_eq!(priority.value, 2.0);
    }

    #[test]
    fn test_withdrawal_priority_very_low() {
        let priority = WithdrawalPriority::very_low();
        assert_eq!(priority.name, "very_low");
        assert_eq!(priority.value, 0.15);
    }

    #[test]
    fn test_withdrawal_priority_low() {
        let priority = WithdrawalPriority::low();
        assert_eq!(priority.name, "low");
        assert_eq!(priority.value, 0.5);
    }

    #[test]
    fn test_withdrawal_priority_medium() {
        let priority = WithdrawalPriority::medium();
        assert_eq!(priority.name, "medium");
        assert_eq!(priority.value, 1.0);
    }

    #[test]
    fn test_withdrawal_priority_high() {
        let priority = WithdrawalPriority::high();
        assert_eq!(priority.name, "high");
        assert_eq!(priority.value, 1.2);
    }

    #[test]
    fn test_withdrawal_priority_very_high() {
        let priority = WithdrawalPriority::very_high();
        assert_eq!(priority.name, "very_high");
        assert_eq!(priority.value, 1.5);
    }

    #[test]
    fn test_withdrawal_priority_serialization() {
        let priority = create_mock_withdrawal_priority();
        let serialized = serde_json::to_string(&priority).unwrap();
        
        assert!(serialized.contains("\"name\":\"custom\""));
        assert!(serialized.contains("\"value\":0.75"));
    }

    #[test]
    fn test_withdrawal_priority_deserialization() {
        let json = r#"{
            "name": "custom",
            "value": 0.75
        }"#;
        
        let priority: WithdrawalPriority = serde_json::from_str(json).unwrap();
        assert_eq!(priority.name, "custom");
        assert_eq!(priority.value, 0.75);
    }

    #[test]
    fn test_withdrawal_priority_clone() {
        let priority = create_mock_withdrawal_priority();
        let cloned = priority.clone();
        assert_eq!(priority, cloned);
    }

    #[test]
    fn test_withdrawal_priority_round_trip_serialization() {
        let original = create_mock_withdrawal_priority();
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: WithdrawalPriority = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_all_predefined_priorities() {
        let priorities = vec![
            WithdrawalPriority::very_low(),
            WithdrawalPriority::low(),
            WithdrawalPriority::medium(),
            WithdrawalPriority::high(),
            WithdrawalPriority::very_high(),
        ];
        
        // Test that all priorities have different values
        let mut values: Vec<f64> = priorities.iter().map(|p| p.value).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Check that values are in ascending order
        assert_eq!(values[0], 0.15); // very_low
        assert_eq!(values[1], 0.5);  // low
        assert_eq!(values[2], 1.0);  // medium
        assert_eq!(values[3], 1.2);  // high
        assert_eq!(values[4], 1.5);  // very_high
    }

    #[test]
    fn test_withdrawal_priority_serialization_all_types() {
        let priorities = vec![
            WithdrawalPriority::very_low(),
            WithdrawalPriority::low(),
            WithdrawalPriority::medium(),
            WithdrawalPriority::high(),
            WithdrawalPriority::very_high(),
        ];
        
        for priority in priorities {
            let serialized = serde_json::to_string(&priority).unwrap();
            let deserialized: WithdrawalPriority = serde_json::from_str(&serialized).unwrap();
            assert_eq!(priority, deserialized);
        }
    }

    #[test]
    fn test_withdrawal_priority_edge_cases() {
        // Test with zero value
        let zero_priority = WithdrawalPriority::new("zero".to_string(), 0.0);
        assert_eq!(zero_priority.value, 0.0);
        
        // Test with negative value
        let negative_priority = WithdrawalPriority::new("negative".to_string(), -1.0);
        assert_eq!(negative_priority.value, -1.0);
        
        // Test with very large value
        let large_priority = WithdrawalPriority::new("large".to_string(), 1000000.0);
        assert_eq!(large_priority.value, 1000000.0);
        
        // Test with empty name
        let empty_name_priority = WithdrawalPriority::new("".to_string(), 1.0);
        assert_eq!(empty_name_priority.name, "");
        
        // Test with special characters in name
        let special_priority = WithdrawalPriority::new("test-priority_123!@#".to_string(), 1.0);
        assert_eq!(special_priority.name, "test-priority_123!@#");
    }

    #[test]
    fn test_withdrawal_priority_partial_eq() {
        let priority1 = WithdrawalPriority::new("test".to_string(), 1.0);
        let priority2 = WithdrawalPriority::new("test".to_string(), 1.0);
        let priority3 = WithdrawalPriority::new("different".to_string(), 1.0);
        let priority4 = WithdrawalPriority::new("test".to_string(), 2.0);
        
        assert_eq!(priority1, priority2);
        assert_ne!(priority1, priority3);
        assert_ne!(priority1, priority4);
    }

    #[test]
    fn test_withdrawal_priority_deserialization_with_float_precision() {
        let json = r#"{
            "name": "precise",
            "value": 1.23456789
        }"#;
        
        let priority: WithdrawalPriority = serde_json::from_str(json).unwrap();
        assert_eq!(priority.name, "precise");
        assert_eq!(priority.value, 1.23456789);
    }

    #[test]
    fn test_withdrawal_priority_serialization_format() {
        let priority = WithdrawalPriority::new("format_test".to_string(), 1.5);
        let serialized = serde_json::to_string_pretty(&priority).unwrap();
        
        // Check that the JSON is properly formatted
        assert!(serialized.contains("\"name\": \"format_test\""));
        assert!(serialized.contains("\"value\": 1.5"));
    }
}