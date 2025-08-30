//! Local date utilities for day-based logic.
//! - Provides `today_ymd()` returning `YYYYMMDD` as `u32`.
//! - Supports override via `CHUTE_KUN_TODAY` env for deterministic tests
//!   (formats: `YYYY-MM-DD` or `YYYYMMDD`).

use chrono::{Datelike, Local, NaiveDate};

/// Return today's local date as `YYYYMMDD`.
/// If `CHUTE_KUN_TODAY` is set, parse it instead (for tests).
pub fn today_ymd() -> u32 {
    if let Ok(s) = std::env::var("CHUTE_KUN_TODAY") {
        if let Some(v) = parse_ymd_override(&s) {
            return v;
        }
    }
    let d = Local::now().date_naive();
    ymd_to_u32(d.year(), d.month(), d.day())
}

fn parse_ymd_override(s: &str) -> Option<u32> {
    let s = s.trim();
    if s.len() == 8 && s.chars().all(|c| c.is_ascii_digit()) {
        let y: i32 = s[0..4].parse().ok()?;
        let m: u32 = s[4..6].parse().ok()?;
        let d: u32 = s[6..8].parse().ok()?;
        NaiveDate::from_ymd_opt(y, m, d)?;
        return Some(ymd_to_u32(y, m, d));
    }
    if let Some((y, m, d)) =
        s.split_once('-').and_then(|(y, rest)| rest.split_once('-').map(|(m, d)| (y, m, d)))
    {
        let y: i32 = y.parse().ok()?;
        let m: u32 = m.parse().ok()?;
        let d: u32 = d.parse().ok()?;
        NaiveDate::from_ymd_opt(y, m, d)?;
        return Some(ymd_to_u32(y, m, d));
    }
    None
}

#[inline]
fn ymd_to_u32(y: i32, m: u32, d: u32) -> u32 {
    (y as u32) * 10000 + m * 100 + d
}
