
use chrono::{Local, Duration}; // Add chrono import

pub fn get_tomorrow_deribit_format() -> String {
    let today = Local::now();
    let tomorrow = today + Duration::days(1);

    // Format: day (2 digits) + month (3 letters uppercase) + year (2 digits)
    // Example: 15SEP25 for September 15, 2025
    let day = tomorrow.format("%d").to_string(); // Two-digit day
    let month = tomorrow.format("%b").to_string().to_uppercase(); // Three-letter month
    let year = tomorrow.format("%y").to_string(); // Two-digit year

    format!("{}{}{}", day, month, year)
}