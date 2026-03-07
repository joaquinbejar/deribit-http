//! Cross-platform async sleep for native and WASM targets
//!
//! This module provides a unified `sleep` function that uses `tokio::time::sleep`
//! on native targets and `futures_timer::Delay` on WASM targets.

use std::time::Duration;

/// Asynchronously sleeps for the specified duration.
///
/// This function provides cross-platform async sleep functionality:
/// - On native targets (with `native` feature): uses `tokio::time::sleep`
/// - On WASM targets: uses `futures_timer::Delay`
///
/// # Arguments
///
/// * `duration` - The duration to sleep for
#[cfg(feature = "native")]
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

/// Asynchronously sleeps for the specified duration.
///
/// This function provides cross-platform async sleep functionality:
/// - On native targets (with `native` feature): uses `tokio::time::sleep`
/// - On WASM targets: uses `futures_timer::Delay`
///
/// # Arguments
///
/// * `duration` - The duration to sleep for
#[cfg(not(feature = "native"))]
pub async fn sleep(duration: Duration) {
    futures_timer::Delay::new(duration).await;
}
