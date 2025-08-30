#[cfg(target_os = "macos")]
#[test]
fn default_path_on_macos_prefers_dot_config() {
    use chute_kun::config::{default_config_path, Config};
    use std::env;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    // Clear env that could override path resolution
    env::remove_var("CHUTE_KUN_CONFIG");
    env::remove_var("XDG_CONFIG_HOME");
    env::set_var("HOME", dir.path());

    let expected = dir.path().join(".config/chute_kun/config.toml");
    let path = default_config_path().expect("resolved path");
    assert_eq!(path, expected);

    // And write file via API
    let written = Config::write_default_file().expect("write default");
    assert_eq!(written, expected);
    let s = std::fs::read_to_string(&written).unwrap();
    assert!(s.contains("day_start = \"09:00\""));
}

