#[cfg(target_os = "macos")]
#[test]
fn default_data_path_on_macos_prefers_xdg_like_local_share() {
    use chute_kun::storage::default_state_path;
    use std::env;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    // Clear env that could override path resolution
    env::remove_var("CHUTE_KUN_STATE");
    env::remove_var("XDG_DATA_HOME");
    env::set_var("HOME", dir.path());

    let expected = dir.path().join(".local/share/chute_kun/snapshot.toml");
    let path = default_state_path().expect("resolved path");
    assert_eq!(path, expected);
}
