//! Clock abstraction to obtain local wall-clock time in minutes since midnight.
//!
//! - `Clock` trait enables injection for tests.
//! - `SystemClock` uses the OS local time zone (chrono::Local).

use chrono::{Local, Timelike};

pub trait Clock: Send + Sync {
    /// Minutes since local midnight (00:00..=23:59).
    fn now_minutes(&self) -> u16;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_minutes(&self) -> u16 {
        let now = Local::now();
        (now.hour() as u16) * 60 + (now.minute() as u16)
    }
}

/// Convenience function for callers that don't need dependency injection.
pub fn system_now_minutes() -> u16 {
    SystemClock.now_minutes()
}

