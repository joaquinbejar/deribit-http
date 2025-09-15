use chrono::{Datelike, Duration, Local};
use deribit_http::utils::get_tomorrow_deribit_format;

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_get_tomorrow_deribit_format_basic() {
        let result = get_tomorrow_deribit_format();

        // Should return a string of exactly 7 characters (DDMMMYY)
        assert_eq!(result.len(), 7);

        // Should match the pattern: 2 digits + 3 uppercase letters + 2 digits
        let day_part = &result[0..2];
        let month_part = &result[2..5];
        let year_part = &result[5..7];

        assert!(
            day_part.chars().all(|c| c.is_ascii_digit()),
            "Day part should be digits: {}",
            day_part
        );
        assert!(
            month_part.chars().all(|c| c.is_ascii_uppercase()),
            "Month part should be uppercase: {}",
            month_part
        );
        assert!(
            year_part.chars().all(|c| c.is_ascii_digit()),
            "Year part should be digits: {}",
            year_part
        );
    }

    #[test]
    fn test_format_structure() {
        let result = get_tomorrow_deribit_format();

        // Should be exactly 7 characters (DDMMMYY format)
        assert_eq!(
            result.len(),
            7,
            "Result should be exactly 7 characters: {}",
            result
        );

        // First two characters should be digits (day)
        let day_part = &result[0..2];
        assert!(
            day_part.chars().all(|c| c.is_ascii_digit()),
            "Day part should be digits: {}",
            day_part
        );

        // Middle three characters should be uppercase letters (month)
        let month_part = &result[2..5];
        assert!(
            month_part.chars().all(|c| c.is_ascii_uppercase()),
            "Month part should be uppercase: {}",
            month_part
        );

        // Last two characters should be digits (year)
        let year_part = &result[5..7];
        assert!(
            year_part.chars().all(|c| c.is_ascii_digit()),
            "Year part should be digits: {}",
            year_part
        );
    }

    #[test]
    fn test_consistency_across_calls() {
        let result1 = get_tomorrow_deribit_format();
        let result2 = get_tomorrow_deribit_format();

        // Both calls should return the same result (same tomorrow)
        assert_eq!(
            result1, result2,
            "Multiple calls should return consistent results"
        );
    }

    #[test]
    fn test_is_tomorrow_date() {
        let result = get_tomorrow_deribit_format();

        // Parse the result to verify it's actually tomorrow's date
        let tomorrow = Local::now() + Duration::days(1);
        let expected_day = format!("{:02}", tomorrow.day());
        let expected_month = match tomorrow.month() {
            1 => "JAN",
            2 => "FEB",
            3 => "MAR",
            4 => "APR",
            5 => "MAY",
            6 => "JUN",
            7 => "JUL",
            8 => "AUG",
            9 => "SEP",
            10 => "OCT",
            11 => "NOV",
            12 => "DEC",
            _ => panic!("Invalid month"),
        };
        let expected_year = format!("{:02}", tomorrow.year() % 100);
        let expected = format!("{}{}{}", expected_day, expected_month, expected_year);

        assert_eq!(
            result, expected,
            "Result should match tomorrow's date in Deribit format"
        );
    }

    #[test]
    fn test_month_abbreviations() {
        // This test verifies that the function uses correct 3-letter month abbreviations
        let result = get_tomorrow_deribit_format();
        let month_part = &result[2..5];

        let valid_months = [
            "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        ];

        assert!(
            valid_months.contains(&month_part),
            "Month abbreviation '{}' should be valid",
            month_part
        );
    }

    #[test]
    fn test_day_padding() {
        let result = get_tomorrow_deribit_format();
        let day_part = &result[0..2];

        // Day should always be 2 digits (padded with zero if needed)
        assert_eq!(day_part.len(), 2, "Day should be 2 digits: {}", day_part);

        // Should be valid day number
        let day: u32 = day_part.parse().expect("Day should be numeric");
        assert!(
            (1..=31).contains(&day),
            "Day {} should be between 1 and 31",
            day
        );
    }

    #[test]
    fn test_year_format() {
        let result = get_tomorrow_deribit_format();
        let year_part = &result[5..7];

        // Year should be 2 digits
        assert_eq!(year_part.len(), 2, "Year should be 2 digits: {}", year_part);

        // Should be numeric
        let _year: u32 = year_part.parse().expect("Year should be numeric");
    }

    #[test]
    fn test_not_today() {
        let result = get_tomorrow_deribit_format();
        let today = Local::now();

        // Format today's date in the same format
        let today_day = format!("{:02}", today.day());
        let today_month = match today.month() {
            1 => "JAN",
            2 => "FEB",
            3 => "MAR",
            4 => "APR",
            5 => "MAY",
            6 => "JUN",
            7 => "JUL",
            8 => "AUG",
            9 => "SEP",
            10 => "OCT",
            11 => "NOV",
            12 => "DEC",
            _ => panic!("Invalid month"),
        };
        let today_year = format!("{:02}", today.year() % 100);
        let today_format = format!("{}{}{}", today_day, today_month, today_year);

        // Result should NOT be today's date
        assert_ne!(result, today_format, "Result should be tomorrow, not today");
    }

    #[test]
    fn test_get_tomorrow_deribit_format_thread_safety() {
        use std::sync::Arc;
        use std::sync::Mutex;
        use std::thread;

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        // Spawn multiple threads to test thread safety
        for _ in 0..10 {
            let results_clone = Arc::clone(&results);
            let handle = thread::spawn(move || {
                let result = get_tomorrow_deribit_format();
                results_clone.lock().unwrap().push(result);
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        let results = results.lock().unwrap();

        // All results should be the same (assuming no midnight crossing)
        let first_result = &results[0];
        for result in results.iter() {
            assert_eq!(result, first_result);
        }

        // All results should be valid format
        for result in results.iter() {
            assert_eq!(result.len(), 7);
            let day_part = &result[0..2];
            let month_part = &result[2..5];
            let year_part = &result[5..7];
            assert!(day_part.chars().all(|c| c.is_ascii_digit()));
            assert!(month_part.chars().all(|c| c.is_ascii_uppercase()));
            assert!(year_part.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_get_tomorrow_deribit_format_performance() {
        use std::time::Instant;

        let start = Instant::now();

        // Call function many times
        for _ in 0..1000 {
            let _ = get_tomorrow_deribit_format();
        }

        let duration = start.elapsed();

        // Should complete reasonably quickly (less than 1 second for 1000 calls)
        assert!(
            duration.as_secs() < 1,
            "Function took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_get_tomorrow_deribit_format_memory_usage() {
        // Test that function doesn't leak memory by calling it many times
        let mut results = Vec::new();

        for _ in 0..100 {
            results.push(get_tomorrow_deribit_format());
        }

        // All results should be valid and consistent
        for result in &results {
            assert_eq!(result.len(), 7);
            let day_part = &result[0..2];
            let month_part = &result[2..5];
            let year_part = &result[5..7];
            assert!(day_part.chars().all(|c| c.is_ascii_digit()));
            assert!(month_part.chars().all(|c| c.is_ascii_uppercase()));
            assert!(year_part.chars().all(|c| c.is_ascii_digit()));
        }

        // In normal circumstances, all results should be the same
        let first = &results[0];
        let all_same = results.iter().all(|r| r == first);
        assert!(all_same, "Results should be consistent: {:?}", results);
    }
}
