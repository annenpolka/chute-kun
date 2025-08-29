use chute_kun::config;
use std::fs;

#[test]
fn writes_default_config_when_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg_dir = tmp.path().join("chute_kun");
    std::env::set_var("CHUTE_KUN_CONFIG_DIR", &cfg_dir);

    let (path, created) = config::write_default_config().expect("write default config");
    assert!(created, "should create when missing");
    let s = fs::read_to_string(&path).unwrap();
    assert!(s.contains("start_of_day = \"09:00\""));
    assert!(s.contains("[keybindings]"));
    assert!(s.contains("quit = \"q\""));
}

#[test]
fn does_not_overwrite_existing_config() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg_dir = tmp.path().join("chute_kun");
    std::env::set_var("CHUTE_KUN_CONFIG_DIR", &cfg_dir);
    fs::create_dir_all(&cfg_dir).unwrap();
    let p = cfg_dir.join("config.toml");
    fs::write(&p, "start_of_day = \"08:00\"\n").unwrap();

    let (path, created) = config::write_default_config().expect("write default config");
    assert!(!created, "should not overwrite existing file");
    assert_eq!(path, p);
    let s = fs::read_to_string(&path).unwrap();
    assert!(s.contains("08:00"), "existing content preserved");
}
