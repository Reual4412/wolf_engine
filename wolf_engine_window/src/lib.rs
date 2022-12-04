//! Provides a high-level, back-end agnostic window API for [Wolf
//! Engine](https://docs.rs/wolf_engine/latest).
//!
//! # Examples
//!
//! ```
//! # use wolf_engine_window::prelude::*;
//! # 
//! # let mut window_backend = TestWindowBackend;
//! #
//! let window_settings = WindowSettings::new()
//!     .with_title("Example Window")
//!     .with_size((800, 600));
//! let window = window_backend.create_window(window_settings)
//!     .unwrap();
//! ```

mod window_settings;
pub use window_settings::*;

#[cfg(test)]
use mockall::automock;

pub mod prelude {
    pub use super::*;
}

#[cfg_attr(test, automock(type Window = MockWindow;))]
pub trait WindowBackend {
    type Window: Window;

    fn create_window(&mut self, settings: WindowSettings) -> Result<Self::Window, String>;
}

#[cfg_attr(test, automock)]
pub trait Window {}

#[doc(hidden)]
pub struct TestWindowBackend;

impl WindowBackend for TestWindowBackend {
    type Window = TestWindow;

    fn create_window(&mut self, _settings: WindowSettings) -> Result<Self::Window, String> {
        Ok(TestWindow)
    }
}

#[doc(hidden)]
pub struct TestWindow;

impl Window for TestWindow {}

#[cfg(test)]
pub mod window_api_tests {
    use super::*;

    #[test]
    fn should_have_title_setter_and_accessor() {
        let (window, _backend) = mock_window(WindowSettings::default());
        window.expect_title()
            .once()
            .returning(|| "Test".to_string());
        window.expect_set_title()
            .once();
    }

    fn mock_window(settings: WindowSettings) -> (MockWindow, MockWindowBackend) {
        let mut backend = MockWindowBackend::new();
        backend.expect_create_window()
            .once()
            .returning(|_| Ok(MockWindow::new()));
        let window = backend.create_window(settings).unwrap();
        (window, backend) 
    }
}
