use chrono::{DateTime, Duration, Local, Utc}; // Add chrono import

/// Returns tomorrow's date in Deribit format (DDMMMYY)
///
/// # Returns
/// A string representing tomorrow's date in the format used by Deribit:
/// - DD: Two-digit day
/// - MMM: Three-letter month (uppercase)
/// - YY: Two-digit year
///
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

/// Converts a date string from Deribit format to UTC DateTime
///
/// Parses a date string in Deribit's DDMMMYY format and converts it to a UTC DateTime.
///
/// # Arguments
///
/// * `date` - A date string in DDMMMYY format (e.g., "15SEP25")
///
/// # Returns
///
/// Returns a `Result` containing either:
/// - `Ok(DateTime<Utc>)` - The parsed date in UTC timezone
/// - `Err(chrono::ParseError)` - If the date string cannot be parsed
///
/// # Examples
///
/// ```
/// use deribit_http::utils::from_deribit_format_date;
///
/// let date = from_deribit_format_date("15SEP25").unwrap();
/// ```
pub fn from_deribit_format_date(date: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    Ok(DateTime::parse_from_str(date, "%d%b%y")?.with_timezone(&Utc))
}
