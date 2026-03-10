//! Cross-platform time utilities for native and WASM targets
//!
//! This module provides unified `Instant`, `SystemTime`, and `UNIX_EPOCH`
//! types that work across both native (std::time) and WASM (web_time) targets.

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;
#[cfg(target_arch = "wasm32")]
pub use web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
pub use web_time::{SystemTime, UNIX_EPOCH};
