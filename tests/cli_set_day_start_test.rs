use assert_cmd::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn set_day_start_updates_config_toml() {
    let dir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("config.toml");

    // Run the binary with CHUTE_KUN_CONFIG pointing to a temp file and set day start
    let mut cmd = Command::cargo_bin("chute").unwrap();
    cmd.env("CHUTE_KUN_CONFIG", &path);
    cmd.args(["--set-day-start", "10:30"]);
    cmd.assert().success();

    let s = fs::read_to_string(&path).expect("config file created/updated");
    assert!(
        s.contains("day_start = \"10:30\""),
        "config should contain updated day_start, got: {}",
        s
    );
}

#[test]
fn set_day_start_rejects_invalid_format() {
    let dir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("config.toml");

    let mut cmd = Command::cargo_bin("chute").unwrap();
    cmd.env("CHUTE_KUN_CONFIG", &path);
    cmd.args(["--set-day-start", "25-99"]); // invalid format (missing colon)
    cmd.assert().failure();
}

#[test]
fn set_day_start_accepts_compact_hhmm() {
    let dir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("config.toml");

    let mut cmd = Command::cargo_bin("chute").unwrap();
    cmd.env("CHUTE_KUN_CONFIG", &path);
    cmd.args(["--set-day-start", "1000"]);
    cmd.assert().success();

    let s = fs::read_to_string(&path).expect("config file created/updated");
    assert!(s.contains("day_start = \"10:00\""), "expected 10:00, got: {}", s);
}
