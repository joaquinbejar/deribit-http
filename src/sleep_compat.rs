//! Cross-platform async sleep for native and WASM targets
//!
//! This module provides a unified `sleep` function that uses `tokio::time::sleep`
//! on native targets and `futures_timer::Delay` on WASM targets.

use std::time::Duration;

#[cfg(feature = "native")]
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

#[cfg(not(feature = "native"))]
pub async fn sleep(duration: Duration) {
    futures_timer::Delay::new(duration).await;
}
