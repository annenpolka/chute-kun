use chute_kun::storage::default_state_path;
use std::env;
use tempfile::tempdir;

#[test]
fn xdg_data_home_env_is_used_when_set() {
    let dir = tempdir().unwrap();
    // Ensure override variables are clear
    env::remove_var("CHUTE_KUN_STATE");
    env::set_var("XDG_DATA_HOME", dir.path());
    // HOME should not affect when XDG_DATA_HOME is present
    env::set_var("HOME", "/nonexistent-home-placeholder");

    let expected = dir.path().join("chute_kun/snapshot.toml");
    let path = default_state_path().expect("resolved path");
    assert_eq!(path, expected);
}

