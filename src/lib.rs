/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

pub mod model;

#[cfg(not(feature = "notifications"))]
pub mod notification {
    pub struct NotificationManager {}

    impl NotificationManager {
        pub fn new() -> Self {
            Self {}
        }

        pub fn notify(&mut self, _title: &str, _body: &str) {}
    }
}

#[cfg(feature = "notifications")]
pub mod notification;
