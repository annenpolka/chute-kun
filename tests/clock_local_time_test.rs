use chrono::{Local, Timelike};

// Red: Expect a clock abstraction that reports local minutes since midnight.
// API under test (to be implemented): chute_kun::clock::{Clock, SystemClock, system_now_minutes}
use chute_kun::clock::{system_now_minutes, Clock, SystemClock};

#[test]
fn system_clock_uses_local_time_minutes() {
    // Expected: minutes since LOCAL midnight
    let now = Local::now();
    let expected = (now.hour() as u16) * 60 + (now.minute() as u16);

    let got = system_now_minutes();
    assert_eq!(got, expected, "SystemClock should return local minutes since midnight");
}

#[test]
fn clock_trait_can_be_implemented_for_fixed_clock_in_tests() {
    // Local injector for tests: prove trait-based injection works.
    struct FixedClock(u16);
    impl Clock for FixedClock {
        fn now_minutes(&self) -> u16 { self.0 }
    }

    let c = FixedClock(9 * 60 + 30);
    assert_eq!(c.now_minutes(), 9 * 60 + 30);

    // Also ensure SystemClock type exists and compiles
    let _ = SystemClock::default();
}

