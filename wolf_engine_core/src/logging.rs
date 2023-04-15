//! Provides a default logging implementation using [`SimpleLogger`].

use simple_logger::SimpleLogger;

pub enum LogLevel {
    Debug,
}

impl Into<log::LevelFilter> for LogLevel {
    fn into(self) -> log::LevelFilter {
        match self {
            Self::Debug => log::LevelFilter::Debug,
        }
    }
}

/// Initializes the logging system with a pre-configured [SimpleLogger] instance.
///
/// This function is provided for those who don't need a complicated logging setup.  Messages will
/// be logged to the terminal.
///
/// # Examples
///
/// To use the default logger, just initialize it by calling this function and providing it with
/// the desired [LogLevel].
///
/// ```
/// # use wolf_engine_core::logging:LogLevel;
/// wolf_engine_core::logging::initialize_logging(LogLevel::Debug);
/// ```
///
/// Messages are logged using [log] macros.
///
/// ```
/// # use log::info;
/// #
/// info!("Hello, world!");
/// ```
pub fn initialize_logging(log_level: LogLevel) {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log_level.into())
        .with_utc_timestamps()
        .init()
        .expect("Failed to initialize the logger");
}
