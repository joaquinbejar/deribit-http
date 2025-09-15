//! Unit tests for book model

use deribit_http::model::book::{
    BookSummary, BookSummaries, OrderBook, OrderBookEntry
};
use serde_json;

#[cfg(test)]
mod book_summary_tests {
    use super::*;

    fn create_mock_book_summary() -> BookSummary {
        BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        )
    }

    #[test]
    fn test_book_summary_new() {
        let summary = create_mock_book_summary();
        
        assert_eq!(summary.instrument_name, "BTC-PERPETUAL");
        assert_eq!(summary.base_currency, "BTC");
        assert_eq!(summary.quote_currency, "USD");
        assert_eq!(summary.mark_price, 50000.0);
        assert_eq!(summary.creation_timestamp, 1640995200000);
        assert_eq!(summary.volume, 0.0);
        assert_eq!(summary.volume_usd, 0.0);
        assert_eq!(summary.open_interest, 0.0);
        assert_eq!(summary.price_change, None);
        assert_eq!(summary.underlying_index, None);
    }

    #[test]
    fn test_book_summary_with_volume() {
        let summary = create_mock_book_summary()
            .with_volume(1000.0, 50000000.0);
        
        assert_eq!(summary.volume, 1000.0);
        assert_eq!(summary.volume_usd, 50000000.0);
    }

    #[test]
    fn test_book_summary_with_prices() {
        let summary = create_mock_book_summary()
            .with_prices(
                Some(49950.0),
                Some(50050.0),
                Some(50000.0),
                Some(51000.0),
                Some(49000.0),
            );
        
        assert_eq!(summary.bid_price, Some(49950.0));
        assert_eq!(summary.ask_price, Some(50050.0));
        assert_eq!(summary.last, Some(50000.0));
        assert_eq!(summary.high, Some(51000.0));
        assert_eq!(summary.low, Some(49000.0));
        assert_eq!(summary.mid_price, Some(50000.0)); // (49950 + 50050) / 2
    }

    #[test]
    fn test_book_summary_with_prices_no_mid() {
        let summary = create_mock_book_summary()
            .with_prices(None, Some(50050.0), Some(50000.0), None, None);
        
        assert_eq!(summary.bid_price, None);
        assert_eq!(summary.ask_price, Some(50050.0));
        assert_eq!(summary.mid_price, None); // No mid price without bid
    }

    #[test]
    fn test_book_summary_with_open_interest() {
        let summary = create_mock_book_summary()
            .with_open_interest(1500.0);
        
        assert_eq!(summary.open_interest, 1500.0);
    }

    #[test]
    fn test_book_summary_with_price_change() {
        let summary = create_mock_book_summary()
            .with_price_change(2.5);
        
        assert_eq!(summary.price_change, Some(2.5));
    }

    #[test]
    fn test_book_summary_with_iv() {
        let summary = create_mock_book_summary()
            .with_iv(0.75);
        
        assert_eq!(summary.mark_iv, Some(0.75));
    }

    #[test]
    fn test_book_summary_with_funding() {
        let summary = create_mock_book_summary()
            .with_funding(0.0001, 0.0008);
        
        assert_eq!(summary.current_funding, Some(0.0001));
        assert_eq!(summary.funding_8h, Some(0.0008));
    }

    #[test]
    fn test_book_summary_with_delivery_price() {
        let summary = create_mock_book_summary()
            .with_delivery_price(50500.0);
        
        assert_eq!(summary.estimated_delivery_price, Some(50500.0));
    }

    #[test]
    fn test_book_summary_spread() {
        let summary = create_mock_book_summary()
            .with_prices(Some(49950.0), Some(50050.0), None, None, None);
        
        assert_eq!(summary.spread(), Some(100.0)); // 50050 - 49950
    }

    #[test]
    fn test_book_summary_spread_none() {
        let summary = create_mock_book_summary();
        assert_eq!(summary.spread(), None);
    }

    #[test]
    fn test_book_summary_spread_percentage() {
        let summary = create_mock_book_summary()
            .with_prices(Some(49950.0), Some(50050.0), None, None, None);
        
        assert_eq!(summary.spread_percentage(), Some(0.2)); // (100 / 50000) * 100
    }

    #[test]
    fn test_book_summary_spread_percentage_none() {
        let summary = create_mock_book_summary();
        assert_eq!(summary.spread_percentage(), None);
    }

    #[test]
    fn test_book_summary_is_perpetual() {
        let perpetual = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(perpetual.is_perpetual());

        let future = BookSummary::new(
            "BTC-25DEC21".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(!future.is_perpetual());
    }

    #[test]
    fn test_book_summary_is_option() {
        let call_option = BookSummary::new(
            "BTC-25DEC21-50000-C".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(call_option.is_option());

        let put_option = BookSummary::new(
            "BTC-25DEC21-50000-P".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(put_option.is_option());

        let perpetual = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(!perpetual.is_option());
    }

    #[test]
    fn test_book_summary_is_future() {
        let future = BookSummary::new(
            "BTC-25DEC21".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(future.is_future());

        let perpetual = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(!perpetual.is_future());

        let option = BookSummary::new(
            "BTC-25DEC21-50000-C".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        assert!(!option.is_future());
    }

    #[test]
    fn test_book_summary_price_change_absolute() {
        let summary = create_mock_book_summary()
            .with_price_change(2.0)
            .with_prices(None, None, Some(51000.0), None, None);
        
        assert_eq!(summary.price_change_absolute(), Some(1020.0)); // 51000 * (2.0 / 100.0)
    }

    #[test]
    fn test_book_summary_price_change_absolute_no_last() {
        let summary = create_mock_book_summary()
            .with_price_change(2.0);
        
        assert_eq!(summary.price_change_absolute(), Some(1000.0)); // 50000 * (2.0 / 100.0)
    }

    #[test]
    fn test_book_summary_serialization() {
        let summary = create_mock_book_summary()
            .with_volume(1000.0, 50000000.0)
            .with_prices(Some(49950.0), Some(50050.0), Some(50000.0), Some(51000.0), Some(49000.0));
        
        let json = serde_json::to_string(&summary).unwrap();
        let deserialized: BookSummary = serde_json::from_str(&json).unwrap();
        
        assert_eq!(summary, deserialized);
    }
}

#[cfg(test)]
mod book_summaries_tests {
    use super::*;

    fn create_mock_summaries() -> BookSummaries {
        let mut summaries = BookSummaries::new();
        
        summaries.add(BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        ).with_volume(1000.0, 50000000.0));
        
        summaries.add(BookSummary::new(
            "ETH-PERPETUAL".to_string(),
            "ETH".to_string(),
            "USD".to_string(),
            3000.0,
            1640995200000,
        ).with_volume(500.0, 1500000.0));
        
        summaries.add(BookSummary::new(
            "BTC-25DEC21-50000-C".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            2000.0,
            1640995200000,
        ).with_open_interest(100.0));
        
        summaries
    }

    #[test]
    fn test_book_summaries_new() {
        let summaries = BookSummaries::new();
        assert!(summaries.summaries.is_empty());
    }

    #[test]
    fn test_book_summaries_default() {
        let summaries = BookSummaries::default();
        assert!(summaries.summaries.is_empty());
    }

    #[test]
    fn test_book_summaries_add() {
        let mut summaries = BookSummaries::new();
        let summary = BookSummary::new(
            "BTC-PERPETUAL".to_string(),
            "BTC".to_string(),
            "USD".to_string(),
            50000.0,
            1640995200000,
        );
        
        summaries.add(summary);
        assert_eq!(summaries.summaries.len(), 1);
    }

    #[test]
    fn test_book_summaries_by_currency() {
        let summaries = create_mock_summaries();
        let btc_summaries = summaries.by_currency("BTC".to_string());
        
        assert_eq!(btc_summaries.len(), 2);
        assert!(btc_summaries.iter().all(|s| s.base_currency == "BTC"));
    }

    #[test]
    fn test_book_summaries_perpetuals() {
        let summaries = create_mock_summaries();
        let perpetuals = summaries.perpetuals();
        
        assert_eq!(perpetuals.len(), 2);
        assert!(perpetuals.iter().all(|s| s.is_perpetual()));
    }

    #[test]
    fn test_book_summaries_options() {
        let summaries = create_mock_summaries();
        let options = summaries.options();
        
        assert_eq!(options.len(), 1);
        assert!(options.iter().all(|s| s.is_option()));
    }

    #[test]
    fn test_book_summaries_futures() {
        let summaries = create_mock_summaries();
        let futures = summaries.futures();
        
        assert_eq!(futures.len(), 0); // No futures in mock data
    }

    #[test]
    fn test_book_summaries_sort_by_volume() {
        let mut summaries = create_mock_summaries();
        summaries.sort_by_volume();
        
        // Should be sorted by volume_usd descending
        assert!(summaries.summaries[0].volume_usd >= summaries.summaries[1].volume_usd);
    }

    #[test]
    fn test_book_summaries_sort_by_open_interest() {
        let mut summaries = create_mock_summaries();
        summaries.sort_by_open_interest();
        
        // Should be sorted by open_interest descending
        assert!(summaries.summaries[0].open_interest >= summaries.summaries[1].open_interest);
    }

    #[test]
    fn test_book_summaries_serialization() {
        let summaries = create_mock_summaries();
        
        let json = serde_json::to_string(&summaries).unwrap();
        let deserialized: BookSummaries = serde_json::from_str(&json).unwrap();
        
        assert_eq!(summaries, deserialized);
    }
}

#[cfg(test)]
mod order_book_entry_tests {
    use super::*;

    #[test]
    fn test_order_book_entry_new() {
        let entry = OrderBookEntry::new(50000.0, 1.5);
        
        assert_eq!(entry.price, 50000.0);
        assert_eq!(entry.amount, 1.5);
    }

    #[test]
    fn test_order_book_entry_notional() {
        let entry = OrderBookEntry::new(50000.0, 1.5);
        
        assert_eq!(entry.notional(), 75000.0); // 50000 * 1.5
    }

    #[test]
    fn test_order_book_entry_serialization() {
        let entry = OrderBookEntry::new(50000.0, 1.5);
        
        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: OrderBookEntry = serde_json::from_str(&json).unwrap();
        
        assert_eq!(entry.price, deserialized.price);
        assert_eq!(entry.amount, deserialized.amount);
    }
}

#[cfg(test)]
mod order_book_tests {
    use super::*;

    fn create_mock_order_book() -> OrderBook {
        let mut book = OrderBook::new(
            "BTC-PERPETUAL".to_string(),
            1640995200000,
            12345,
        );
        
        book.bids = vec![
            OrderBookEntry::new(49950.0, 1.0),
            OrderBookEntry::new(49900.0, 2.0),
            OrderBookEntry::new(49850.0, 1.5),
        ];
        
        book.asks = vec![
            OrderBookEntry::new(50050.0, 0.8),
            OrderBookEntry::new(50100.0, 1.2),
            OrderBookEntry::new(50150.0, 2.0),
        ];
        
        book
    }

    #[test]
    fn test_order_book_new() {
        let book = OrderBook::new(
            "BTC-PERPETUAL".to_string(),
            1640995200000,
            12345,
        );
        
        assert_eq!(book.instrument_name, "BTC-PERPETUAL");
        assert_eq!(book.timestamp, 1640995200000);
        assert_eq!(book.change_id, 12345);
        assert_eq!(book.prev_change_id, None);
        assert!(book.bids.is_empty());
        assert!(book.asks.is_empty());
    }

    #[test]
    fn test_order_book_best_bid() {
        let book = create_mock_order_book();
        assert_eq!(book.best_bid(), Some(49950.0));
    }

    #[test]
    fn test_order_book_best_ask() {
        let book = create_mock_order_book();
        assert_eq!(book.best_ask(), Some(50050.0));
    }

    #[test]
    fn test_order_book_spread() {
        let book = create_mock_order_book();
        assert_eq!(book.spread(), Some(100.0)); // 50050 - 49950
    }

    #[test]
    fn test_order_book_mid_price() {
        let book = create_mock_order_book();
        assert_eq!(book.mid_price(), Some(50000.0)); // (50050 + 49950) / 2
    }

    #[test]
    fn test_order_book_total_bid_volume() {
        let book = create_mock_order_book();
        assert_eq!(book.total_bid_volume(), 4.5); // 1.0 + 2.0 + 1.5
    }

    #[test]
    fn test_order_book_total_ask_volume() {
        let book = create_mock_order_book();
        assert_eq!(book.total_ask_volume(), 4.0); // 0.8 + 1.2 + 2.0
    }

    #[test]
    fn test_order_book_volume_at_price() {
        let book = create_mock_order_book();
        
        assert_eq!(book.volume_at_price(49950.0, true), 1.0);
        assert_eq!(book.volume_at_price(50050.0, false), 0.8);
        assert_eq!(book.volume_at_price(99999.0, true), 0.0); // Non-existent price
    }

    #[test]
    fn test_order_book_empty_best_prices() {
        let book = OrderBook::new(
            "BTC-PERPETUAL".to_string(),
            1640995200000,
            12345,
        );
        
        assert_eq!(book.best_bid(), None);
        assert_eq!(book.best_ask(), None);
        assert_eq!(book.spread(), None);
        assert_eq!(book.mid_price(), None);
    }

    #[test]
    fn test_order_book_serialization() {
        let book = create_mock_order_book();
        
        let json = serde_json::to_string(&book).unwrap();
        let deserialized: OrderBook = serde_json::from_str(&json).unwrap();
        
        assert_eq!(book.instrument_name, deserialized.instrument_name);
        assert_eq!(book.timestamp, deserialized.timestamp);
        assert_eq!(book.change_id, deserialized.change_id);
        assert_eq!(book.bids.len(), deserialized.bids.len());
        assert_eq!(book.asks.len(), deserialized.asks.len());
    }
}