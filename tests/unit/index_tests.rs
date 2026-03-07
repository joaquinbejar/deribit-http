use deribit_http::model::index::{IndexChartDataPoint, IndexData, IndexPriceData};
use serde_json;

#[cfg(test)]
mod index_chart_data_point_tests {
    use super::*;

    #[test]
    fn test_index_chart_data_point_new() {
        let point = IndexChartDataPoint::new(1573228800000, 8751.7138636);

        assert_eq!(point.timestamp, 1573228800000);
        assert!((point.price - 8751.7138636).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_chart_data_point_serialization() {
        let point = IndexChartDataPoint::new(1573228800000, 8751.7138636);

        let json = serde_json::to_string(&point).expect("serialization should succeed");
        // Should serialize as tuple [timestamp, price]
        assert!(json.contains("1573228800000"));
        assert!(json.contains("8751.7138636"));
    }

    #[test]
    fn test_index_chart_data_point_deserialization() {
        // API returns data as [timestamp, price] tuple
        let json = "[1573228800000, 8751.7138636]";
        let point: IndexChartDataPoint =
            serde_json::from_str(json).expect("deserialization should succeed");

        assert_eq!(point.timestamp, 1573228800000);
        assert!((point.price - 8751.7138636).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_chart_data_point_roundtrip() {
        let original = IndexChartDataPoint::new(1573228800000, 8751.7138636);

        let json = serde_json::to_string(&original).expect("serialization should succeed");
        let deserialized: IndexChartDataPoint =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(original.timestamp, deserialized.timestamp);
        assert!((original.price - deserialized.price).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_chart_data_point_clone() {
        let point = IndexChartDataPoint::new(1573228800000, 8751.7138636);
        let cloned = point;

        assert_eq!(point.timestamp, cloned.timestamp);
        assert!((point.price - cloned.price).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_chart_data_point_copy() {
        let point = IndexChartDataPoint::new(1573228800000, 8751.7138636);
        let copied = point;

        // Both should have same values (Copy trait)
        assert_eq!(point.timestamp, copied.timestamp);
        assert!((point.price - copied.price).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_chart_data_point_partial_eq() {
        let point1 = IndexChartDataPoint::new(1573228800000, 8751.7138636);
        let point2 = IndexChartDataPoint::new(1573228800000, 8751.7138636);
        let point3 = IndexChartDataPoint::new(1573232400000, 8751.7138636);

        assert_eq!(point1, point2);
        assert_ne!(point1, point3);
    }

    #[test]
    fn test_index_chart_data_point_edge_cases() {
        // Test with zero values
        let zero_point = IndexChartDataPoint::new(0, 0.0);
        assert_eq!(zero_point.timestamp, 0);
        assert!((zero_point.price - 0.0).abs() < f64::EPSILON);

        // Test with large timestamp (far future)
        let future_point = IndexChartDataPoint::new(4102444800000, 100000.0);
        assert_eq!(future_point.timestamp, 4102444800000);
        assert!((future_point.price - 100000.0).abs() < f64::EPSILON);

        // Test with very small price
        let small_price = IndexChartDataPoint::new(1573228800000, 0.00000001);
        assert!((small_price.price - 0.00000001).abs() < f64::EPSILON);

        // Test with very large price
        let large_price = IndexChartDataPoint::new(1573228800000, 1000000000.0);
        assert!((large_price.price - 1000000000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_chart_data_point_debug() {
        let point = IndexChartDataPoint::new(1573228800000, 8751.7138636);

        let debug_str = format!("{:?}", point);
        assert!(debug_str.contains("IndexChartDataPoint"));
        assert!(debug_str.contains("1573228800000"));
    }

    #[test]
    fn test_index_chart_data_vec_deserialization() {
        // Test deserializing an array of points (as returned by API)
        let json = r#"[
            [1573228800000, 8751.7138636],
            [1573232400000, 8752.0],
            [1573236000000, 8753.5]
        ]"#;

        let points: Vec<IndexChartDataPoint> =
            serde_json::from_str(json).expect("deserialization should succeed");

        assert_eq!(points.len(), 3);
        assert_eq!(points[0].timestamp, 1573228800000);
        assert!((points[0].price - 8751.7138636).abs() < f64::EPSILON);
        assert_eq!(points[1].timestamp, 1573232400000);
        assert!((points[1].price - 8752.0).abs() < f64::EPSILON);
        assert_eq!(points[2].timestamp, 1573236000000);
        assert!((points[2].price - 8753.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_chart_data_empty_vec() {
        let json = "[]";
        let points: Vec<IndexChartDataPoint> =
            serde_json::from_str(json).expect("deserialization should succeed");

        assert!(points.is_empty());
    }

    #[test]
    fn test_index_chart_data_realistic_btc_prices() {
        // Test with realistic BTC price data
        let point = IndexChartDataPoint::new(1709683200000, 67234.56);

        assert_eq!(point.timestamp, 1709683200000);
        assert!((point.price - 67234.56).abs() < f64::EPSILON);

        // Verify the timestamp is reasonable (March 2024)
        assert!(point.timestamp > 1700000000000);
        assert!(point.timestamp < 2000000000000);
    }
}

#[cfg(test)]
mod index_data_tests {
    use super::*;

    #[test]
    fn test_index_data_deserialization() {
        let json = r#"{
            "btc": 50000.0,
            "eth": 3000.0,
            "edp": 50100.0
        }"#;

        let data: IndexData = serde_json::from_str(json).expect("deserialization should succeed");

        assert_eq!(data.btc, Some(50000.0));
        assert_eq!(data.eth, Some(3000.0));
        assert!((data.edp - 50100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_data_optional_fields() {
        let json = r#"{
            "edp": 50100.0
        }"#;

        let data: IndexData = serde_json::from_str(json).expect("deserialization should succeed");

        assert!(data.btc.is_none());
        assert!(data.eth.is_none());
        assert!(data.usdc.is_none());
        assert!(data.usdt.is_none());
        assert!(data.eurr.is_none());
        assert!((data.edp - 50100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_data_clone() {
        let json = r#"{
            "btc": 50000.0,
            "edp": 50100.0
        }"#;

        let data: IndexData = serde_json::from_str(json).expect("deserialization should succeed");
        let cloned = data.clone();

        assert_eq!(data.btc, cloned.btc);
        assert!((data.edp - cloned.edp).abs() < f64::EPSILON);
    }
}

#[cfg(test)]
mod index_price_data_tests {
    use super::*;

    #[test]
    fn test_index_price_data_deserialization() {
        let json = r#"{
            "index_price": 50000.0,
            "estimated_delivery_price": 50100.0
        }"#;

        let data: IndexPriceData =
            serde_json::from_str(json).expect("deserialization should succeed");

        assert!((data.index_price - 50000.0).abs() < f64::EPSILON);
        assert!((data.estimated_delivery_price - 50100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_index_price_data_serialization_roundtrip() {
        let json = r#"{"index_price":50000.0,"estimated_delivery_price":50100.0}"#;

        let data: IndexPriceData =
            serde_json::from_str(json).expect("deserialization should succeed");
        let serialized = serde_json::to_string(&data).expect("serialization should succeed");
        let deserialized: IndexPriceData =
            serde_json::from_str(&serialized).expect("deserialization should succeed");

        assert!((data.index_price - deserialized.index_price).abs() < f64::EPSILON);
        assert!(
            (data.estimated_delivery_price - deserialized.estimated_delivery_price).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn test_index_price_data_clone() {
        let json = r#"{
            "index_price": 50000.0,
            "estimated_delivery_price": 50100.0
        }"#;

        let data: IndexPriceData =
            serde_json::from_str(json).expect("deserialization should succeed");
        let cloned = data.clone();

        assert!((data.index_price - cloned.index_price).abs() < f64::EPSILON);
        assert!(
            (data.estimated_delivery_price - cloned.estimated_delivery_price).abs() < f64::EPSILON
        );
    }
}
