//! Cross-platform Mutex re-export for native and WASM targets
//!
//! This module provides a unified `Mutex` type that uses `tokio::sync::Mutex`
//! on native targets and `async_lock::Mutex` on WASM targets.

#[cfg(feature = "native")]
pub use tokio::sync::Mutex;

#[cfg(not(feature = "native"))]
pub use async_lock::Mutex;
