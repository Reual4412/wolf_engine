use log::{LevelFilter, Log, Metadata, Record};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub(crate) static ref LOGGER: Logger = Logger::new();
}

pub(crate) fn initialize_logging(level: LevelFilter) -> Result<&'static Logger, ()> {
    log::set_logger(&*LOGGER as &dyn Log)
        .map(|()| log::set_max_level(level));
    Ok(&LOGGER)
}

pub struct Logger {
    log_targets: Arc<Mutex<Vec<&'static dyn LogTarget>>>
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_log_target(&self, log_target: &dyn LogTarget) {}
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {}

    fn flush(&self) {}
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            log_targets: Arc::new(Mutex::new(vec![]))
        }
    }
}


pub trait LogTarget: 'static + Send + Sync {}

#[cfg(test)]
mod log_tests {
    use super::log_test_fixtures::*;
    use super::*;
    use log::info;

    #[test]
    fn should_log_to_connected_log_targets() {
        let logger =
            initialize_logging(LevelFilter::Trace).expect("Failed to initialize the logger");
        let log_target_a = TestLogTarget::new();
        let log_target_b = TestLogTarget::new();
        logger.add_log_target(&log_target_a);
        logger.add_log_target(&log_target_b);

        info!("Hello, World!");

        assert_eq!(
            log_target_a
                .latest_record()
                .expect("No message was sent")
                .args()
                .to_string(),
            "Hello, World!".to_string()
        );
        assert_eq!(
            log_target_b
                .latest_record()
                .expect("No message was sent")
                .args()
                .to_string(),
            "Hello, World!".to_string()
        );
    }
}

#[cfg(test)]
pub mod log_test_fixtures {
    use super::*;
    use log::Record;

    pub struct TestLogTarget;

    impl TestLogTarget {
        pub fn new() -> Self {
            Self
        }

        pub fn latest_record(&self) -> Option<Record> {
            None
        }
    }

    impl LogTarget for TestLogTarget {}
}
