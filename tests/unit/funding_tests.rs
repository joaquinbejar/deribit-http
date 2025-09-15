use deribit_http::model::funding::{FundingChartData, FundingDataPoint, FundingRateData};
use serde_json;

#[cfg(test)]
mod funding_chart_data_tests {
    use super::*;

    #[test]
    fn test_funding_chart_data_new() {
        let chart_data = FundingChartData::new();
        
        assert_eq!(chart_data.current_interest, 0.0);
        assert_eq!(chart_data.interest_8h, 0.0);
        assert!(chart_data.data.is_empty());
    }

    #[test]
    fn test_funding_chart_data_default() {
        let chart_data = FundingChartData::default();
        
        assert_eq!(chart_data.current_interest, 0.0);
        assert_eq!(chart_data.interest_8h, 0.0);
        assert!(chart_data.data.is_empty());
    }

    #[test]
    fn test_funding_chart_data_with_data() {
        let mut chart_data = FundingChartData::new();
        chart_data.current_interest = 0.0001;
        chart_data.interest_8h = 0.0008;
        
        let data_point = FundingDataPoint::new(50000.0, 0.0001, 1640995200000);
        chart_data.data.push(data_point);
        
        assert_eq!(chart_data.current_interest, 0.0001);
        assert_eq!(chart_data.interest_8h, 0.0008);
        assert_eq!(chart_data.data.len(), 1);
    }

    #[test]
    fn test_funding_chart_data_serialization() {
        let mut chart_data = FundingChartData::new();
        chart_data.current_interest = 0.0001;
        chart_data.interest_8h = 0.0008;
        
        let data_point = FundingDataPoint::new(50000.0, 0.0001, 1640995200000);
        chart_data.data.push(data_point);
        
        let json = serde_json::to_string(&chart_data).unwrap();
        let deserialized: FundingChartData = serde_json::from_str(&json).unwrap();
        
        assert_eq!(chart_data.current_interest, deserialized.current_interest);
        assert_eq!(chart_data.interest_8h, deserialized.interest_8h);
        assert_eq!(chart_data.data.len(), deserialized.data.len());
    }

    #[test]
    fn test_funding_chart_data_clone() {
        let mut chart_data = FundingChartData::new();
        chart_data.current_interest = 0.0001;
        
        let cloned = chart_data.clone();
        assert_eq!(chart_data.current_interest, cloned.current_interest);
        assert_eq!(chart_data.interest_8h, cloned.interest_8h);
    }

    #[test]
    fn test_funding_chart_data_edge_cases() {
        let mut chart_data = FundingChartData::new();
        
        // Test with extreme values
        chart_data.current_interest = f64::MAX;
        chart_data.interest_8h = f64::MIN;
        
        assert_eq!(chart_data.current_interest, f64::MAX);
        assert_eq!(chart_data.interest_8h, f64::MIN);
        
        // Test with negative values
        chart_data.current_interest = -0.0001;
        chart_data.interest_8h = -0.0008;
        
        assert_eq!(chart_data.current_interest, -0.0001);
        assert_eq!(chart_data.interest_8h, -0.0008);
    }
}

#[cfg(test)]
mod funding_data_point_tests {
    use super::*;

    #[test]
    fn test_funding_data_point_new() {
        let data_point = FundingDataPoint::new(50000.0, 0.0001, 1640995200000);
        
        assert_eq!(data_point.index_price, 50000.0);
        assert_eq!(data_point.interest_8h, 0.0001);
        assert_eq!(data_point.timestamp, 1640995200000);
    }

    #[test]
    fn test_funding_data_point_serialization() {
        let data_point = FundingDataPoint::new(50000.0, 0.0001, 1640995200000);
        
        let json = serde_json::to_string(&data_point).unwrap();
        let deserialized: FundingDataPoint = serde_json::from_str(&json).unwrap();
        
        assert_eq!(data_point.index_price, deserialized.index_price);
        assert_eq!(data_point.interest_8h, deserialized.interest_8h);
        assert_eq!(data_point.timestamp, deserialized.timestamp);
    }

    #[test]
    fn test_funding_data_point_clone() {
        let data_point = FundingDataPoint::new(50000.0, 0.0001, 1640995200000);
        let cloned = data_point.clone();
        
        assert_eq!(data_point.index_price, cloned.index_price);
        assert_eq!(data_point.interest_8h, cloned.interest_8h);
        assert_eq!(data_point.timestamp, cloned.timestamp);
    }

    #[test]
    fn test_funding_data_point_edge_cases() {
        // Test with zero values
        let zero_point = FundingDataPoint::new(0.0, 0.0, 0);
        assert_eq!(zero_point.index_price, 0.0);
        assert_eq!(zero_point.interest_8h, 0.0);
        assert_eq!(zero_point.timestamp, 0);
        
        // Test with negative values
        let negative_point = FundingDataPoint::new(-1000.0, -0.001, 1640995200000);
        assert_eq!(negative_point.index_price, -1000.0);
        assert_eq!(negative_point.interest_8h, -0.001);
        
        // Test with extreme values
        let extreme_point = FundingDataPoint::new(f64::MAX, f64::MIN, u64::MAX);
        assert_eq!(extreme_point.index_price, f64::MAX);
        assert_eq!(extreme_point.interest_8h, f64::MIN);
        assert_eq!(extreme_point.timestamp, u64::MAX);
    }

    #[test]
    fn test_funding_data_point_debug_display() {
        let data_point = FundingDataPoint::new(50000.0, 0.0001, 1640995200000);
        
        // Test Debug trait (uses DebugPretty)
        let debug_str = format!("{:?}", data_point);
        assert!(debug_str.contains("50000"));
        assert!(debug_str.contains("0.0001"));
        
        // Test Display trait
        let display_str = format!("{}", data_point);
        assert!(!display_str.is_empty());
    }
}

#[cfg(test)]
mod funding_rate_data_tests {
    use super::*;

    #[test]
    fn test_funding_rate_data_new() {
        let rate_data = FundingRateData::new(
            1640995200000,
            50000.0,
            0.0001,
            0.000012,
            49950.0,
        );
        
        assert_eq!(rate_data.timestamp, 1640995200000);
        assert_eq!(rate_data.index_price, 50000.0);
        assert_eq!(rate_data.interest_8h, 0.0001);
        assert_eq!(rate_data.interest_1h, 0.000012);
        assert_eq!(rate_data.prev_index_price, 49950.0);
    }

    #[test]
    fn test_funding_rate_data_serialization() {
        let rate_data = FundingRateData::new(
            1640995200000,
            50000.0,
            0.0001,
            0.000012,
            49950.0,
        );
        
        let json = serde_json::to_string(&rate_data).unwrap();
        let deserialized: FundingRateData = serde_json::from_str(&json).unwrap();
        
        assert_eq!(rate_data.timestamp, deserialized.timestamp);
        assert_eq!(rate_data.index_price, deserialized.index_price);
        assert_eq!(rate_data.interest_8h, deserialized.interest_8h);
        assert_eq!(rate_data.interest_1h, deserialized.interest_1h);
        assert_eq!(rate_data.prev_index_price, deserialized.prev_index_price);
    }

    #[test]
    fn test_funding_rate_data_clone() {
        let rate_data = FundingRateData::new(
            1640995200000,
            50000.0,
            0.0001,
            0.000012,
            49950.0,
        );
        
        let cloned = rate_data.clone();
        
        assert_eq!(rate_data.timestamp, cloned.timestamp);
        assert_eq!(rate_data.index_price, cloned.index_price);
        assert_eq!(rate_data.interest_8h, cloned.interest_8h);
        assert_eq!(rate_data.interest_1h, cloned.interest_1h);
        assert_eq!(rate_data.prev_index_price, cloned.prev_index_price);
    }

    #[test]
    fn test_funding_rate_data_edge_cases() {
        // Test with extreme values
        let extreme_data = FundingRateData::new(
            u64::MAX,
            f64::MAX,
            f64::MIN,
            0.0,
            f64::INFINITY,
        );
        
        assert_eq!(extreme_data.timestamp, u64::MAX);
        assert_eq!(extreme_data.index_price, f64::MAX);
        assert_eq!(extreme_data.interest_8h, f64::MIN);
        assert_eq!(extreme_data.interest_1h, 0.0);
        assert_eq!(extreme_data.prev_index_price, f64::INFINITY);
        
        // Test with zero values
        let zero_data = FundingRateData::new(0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(zero_data.timestamp, 0);
        assert_eq!(zero_data.index_price, 0.0);
        assert_eq!(zero_data.interest_8h, 0.0);
        assert_eq!(zero_data.interest_1h, 0.0);
        assert_eq!(zero_data.prev_index_price, 0.0);
        
        // Test with negative values
        let negative_data = FundingRateData::new(
            1640995200000,
            -50000.0,
            -0.0001,
            -0.000012,
            -49950.0,
        );
        
        assert_eq!(negative_data.index_price, -50000.0);
        assert_eq!(negative_data.interest_8h, -0.0001);
        assert_eq!(negative_data.interest_1h, -0.000012);
        assert_eq!(negative_data.prev_index_price, -49950.0);
    }

    #[test]
    fn test_funding_rate_data_debug_display() {
        let rate_data = FundingRateData::new(
            1640995200000,
            50000.0,
            0.0001,
            0.000012,
            49950.0,
        );
        
        // Test Debug trait (uses DebugPretty)
        let debug_str = format!("{:?}", rate_data);
        assert!(debug_str.contains("50000"));
        assert!(debug_str.contains("0.0001"));
        
        // Test Display trait
        let display_str = format!("{}", rate_data);
        assert!(!display_str.is_empty());
    }

    #[test]
    fn test_funding_rate_data_interest_comparison() {
        let rate_data = FundingRateData::new(
            1640995200000,
            50000.0,
            0.0008,  // 8h rate
            0.0001,  // 1h rate
            49950.0,
        );
        
        // Typically 8h rate should be higher than 1h rate
        assert!(rate_data.interest_8h > rate_data.interest_1h);
        
        // Test price change calculation
        let price_change = rate_data.index_price - rate_data.prev_index_price;
        assert_eq!(price_change, 50.0); // 50000 - 49950
    }

    #[test]
    fn test_funding_rate_data_realistic_values() {
        // Test with realistic funding rate values
        let realistic_data = FundingRateData::new(
            1640995200000,  // Jan 1, 2022
            45000.0,        // BTC price
            0.0001,         // 0.01% 8h funding
            0.0000125,      // ~0.001% 1h funding
            44950.0,        // Previous price
        );
        
        assert!(realistic_data.interest_8h > 0.0);
        assert!(realistic_data.interest_1h > 0.0);
        assert!(realistic_data.interest_8h > realistic_data.interest_1h);
        assert!(realistic_data.index_price > realistic_data.prev_index_price);
        
        // Funding rates should be small percentages
        assert!(realistic_data.interest_8h < 0.01); // Less than 1%
        assert!(realistic_data.interest_1h < 0.001); // Less than 0.1%
    }
}