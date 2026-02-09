//! Cross-platform time utilities for native and WASM targets
//!
//! This module provides a unified `Instant` type that works across
//! both native (std::time::Instant) and WASM (web_time::Instant) targets.

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub use web_time::Instant;
