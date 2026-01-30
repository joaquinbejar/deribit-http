//! Logger setup for the Deribit HTTP client
//!
//! This module provides cross-platform logging setup using tracing.
//! - Native: Uses `tracing_subscriber::FmtSubscriber` with env var configuration
//! - WASM: Uses `tracing-web` to route logs to browser/Worker console

use std::sync::Once;

#[cfg(not(target_arch = "wasm32"))]
use std::env;
#[cfg(not(target_arch = "wasm32"))]
use tracing::Level;
#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::FmtSubscriber;

#[cfg(target_arch = "wasm32")]
use tracing_subscriber::Layer;
#[cfg(target_arch = "wasm32")]
use tracing_subscriber::layer::SubscriberExt;
#[cfg(target_arch = "wasm32")]
use tracing_subscriber::util::SubscriberInitExt;
#[cfg(target_arch = "wasm32")]
use tracing_web::MakeWebConsoleWriter;

static INIT: Once = Once::new();

/// Sets up the logger for the application.
///
/// - **Native**: Log level is determined by `DERIBIT_LOG_LEVEL` env var (defaults to INFO)
/// - **WASM**: Logs to browser/Worker console at INFO level
#[cfg(not(target_arch = "wasm32"))]
pub fn setup_logger() {
    INIT.call_once(|| {
        let log_level = env::var("DERIBIT_LOG_LEVEL")
            .unwrap_or_else(|_| "INFO".to_string())
            .to_uppercase();

        let level = match log_level.as_str() {
            "DEBUG" => Level::DEBUG,
            "ERROR" => Level::ERROR,
            "WARN" => Level::WARN,
            "TRACE" => Level::TRACE,
            _ => Level::INFO,
        };

        let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Error setting default subscriber");

        tracing::debug!("Log level set to: {}", level);
    });
}

/// Sets up the logger for the application.
///
/// - **Native**: Log level is determined by `DERIBIT_LOG_LEVEL` env var (defaults to INFO)
/// - **WASM**: Logs to browser/Worker console at INFO level
#[cfg(target_arch = "wasm32")]
pub fn setup_logger() {
    INIT.call_once(|| {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .without_time()
            .with_writer(MakeWebConsoleWriter::new())
            .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

        tracing_subscriber::registry().with(fmt_layer).init();

        tracing::debug!("WASM logger initialized");
    });
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests_setup_logger {
    use super::setup_logger;
    use std::env;
    use tracing::subscriber::set_global_default;
    use tracing_subscriber::FmtSubscriber;

    #[test]
    fn test_logger_initialization_info() {
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "INFO");
        }
        setup_logger();

        // After setting up the logger, you would typically assert that the logger is working
        // However, due to the nature of logging, it's difficult to directly assert on log output.
        // You can, however, check that set_global_default has been called successfully without panic.
        assert!(
            set_global_default(FmtSubscriber::builder().finish()).is_err(),
            "Logger should already be set"
        );
    }

    #[test]
    fn test_logger_initialization_debug() {
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "DEBUG");
        }
        setup_logger();

        // Similar to the previous test, check that the global logger has been set
        assert!(
            set_global_default(FmtSubscriber::builder().finish()).is_err(),
            "Logger should already be set"
        );
    }

    #[test]
    fn test_logger_initialization_default() {
        unsafe {
            env::remove_var("DERIBIT_LOG_LEVEL");
        }
        setup_logger();

        // Check that the global logger has been set
        assert!(
            set_global_default(FmtSubscriber::builder().finish()).is_err(),
            "Logger should already be set"
        );
    }

    #[test]
    fn test_logger_called_once() {
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "INFO");
        }

        setup_logger(); // First call should set up the logger
        setup_logger(); // Second call should not re-initialize

        // Check that the global logger has been set only once
        assert!(
            set_global_default(FmtSubscriber::builder().finish()).is_err(),
            "Logger should already be set and should not be reset"
        );
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests_setup_logger_bis {
    use super::*;
    use std::sync::Mutex;
    use tracing::info;
    use tracing::subscriber::with_default;
    use tracing_subscriber::Layer;
    use tracing_subscriber::layer::SubscriberExt;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[derive(Clone)]
    struct TestLayer {
        level: std::sync::Arc<Mutex<Option<Level>>>,
    }

    impl<S> Layer<S> for TestLayer
    where
        S: tracing::Subscriber,
    {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _ctx: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let mut level = self.level.lock().unwrap();
            *level = Some(*event.metadata().level());
        }
    }

    fn create_test_layer() -> (TestLayer, std::sync::Arc<Mutex<Option<Level>>>) {
        let level = std::sync::Arc::new(Mutex::new(None));
        (
            TestLayer {
                level: level.clone(),
            },
            level,
        )
    }

    #[test]
    fn test_default_log_level() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("DERIBIT_LOG_LEVEL");
        }

        let (layer, level) = create_test_layer();
        let subscriber = tracing_subscriber::registry().with(layer);

        with_default(subscriber, || {
            setup_logger();
            info!("Test log");
        });

        assert_eq!(*level.lock().unwrap(), Some(Level::INFO));
    }

    #[test]
    fn test_debug_log_level() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "DEBUG");
        }

        let (layer, level) = create_test_layer();
        let subscriber = tracing_subscriber::registry().with(layer);

        with_default(subscriber, || {
            setup_logger();
            tracing::debug!("Test log");
        });

        assert_eq!(*level.lock().unwrap(), Some(Level::DEBUG));

        unsafe {
            env::remove_var("DERIBIT_LOG_LEVEL");
        }
    }

    #[test]
    fn test_error_log_level() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "ERROR");
        }

        let (layer, level) = create_test_layer();
        let subscriber = tracing_subscriber::registry().with(layer);

        with_default(subscriber, || {
            setup_logger();
            tracing::error!("Test log");
        });

        assert_eq!(*level.lock().unwrap(), Some(Level::ERROR));
        unsafe {
            env::remove_var("DERIBIT_LOG_LEVEL");
        }
    }

    #[test]
    fn test_warn_log_level() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "WARN");
        }

        let (layer, level) = create_test_layer();
        let subscriber = tracing_subscriber::registry().with(layer);

        with_default(subscriber, || {
            setup_logger();
            tracing::warn!("Test log");
        });

        assert_eq!(*level.lock().unwrap(), Some(Level::WARN));
        unsafe {
            env::remove_var("DERIBIT_LOG_LEVEL");
        }
    }

    #[test]
    fn test_trace_log_level() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "TRACE");
        }

        let (layer, level) = create_test_layer();
        let subscriber = tracing_subscriber::registry().with(layer);

        with_default(subscriber, || {
            setup_logger();
            tracing::trace!("Test log");
        });

        assert_eq!(*level.lock().unwrap(), Some(Level::TRACE));
        unsafe {
            env::remove_var("DERIBIT_LOG_LEVEL");
        }
    }

    #[test]
    fn test_invalid_log_level() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("DERIBIT_LOG_LEVEL", "INVALID");
        }

        let (layer, level) = create_test_layer();
        let subscriber = tracing_subscriber::registry().with(layer);

        with_default(subscriber, || {
            setup_logger();
            info!("Test log");
        });

        assert_eq!(*level.lock().unwrap(), Some(Level::INFO));
        unsafe {
            env::remove_var("DERIBIT_LOG_LEVEL");
        }
    }
}
