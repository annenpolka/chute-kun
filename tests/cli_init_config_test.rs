use assert_cmd::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn init_config_writes_default_toml_to_env_path() {
    let dir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("config.toml");

    let mut cmd = Command::cargo_bin("chute-kun").unwrap();
    cmd.env("CHUTE_KUN_CONFIG", &path);
    cmd.arg("--init-config");
    cmd.assert().success();

    let s = fs::read_to_string(&path).expect("config file created");
    assert!(s.contains("day_start = \"09:00\""), "default day_start missing: {}", s);
}
