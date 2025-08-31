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

/// Add `days` to a `YYYYMMDD` date, returning `YYYYMMDD`.
/// Negative `days` subtracts. Panics if input is invalid.
pub fn add_days_to_ymd(ymd: u32, days: i32) -> u32 {
    let y = (ymd / 10000) as i32;
    let m = ymd / 100 % 100;
    let d = ymd % 100;
    let date = NaiveDate::from_ymd_opt(y, m, d).expect("valid ymd");
    let new = date
        .checked_add_signed(chrono::Duration::days(days as i64))
        .expect("date add within range");
    ymd_to_u32(new.year(), new.month(), new.day())
}

/// Format `YYYYMMDD` to `YYYY-MM-DD`.
pub fn format_ymd(ymd: u32) -> String {
    let y = (ymd / 10000) as i32;
    let m = ymd / 100 % 100;
    let d = ymd % 100;
    format!("{:04}-{:02}-{:02}", y, m, d)
}

/// Return English short weekday for a given `YYYYMMDD` (e.g., "Mon").
pub fn weekday_short_en(ymd: u32) -> &'static str {
    let y = (ymd / 10000) as i32;
    let m = ymd / 100 % 100;
    let d = ymd % 100;
    let date = NaiveDate::from_ymd_opt(y, m, d).expect("valid ymd");
    match date.weekday() {
        chrono::Weekday::Mon => "Mon",
        chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed",
        chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri",
        chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    }
}
