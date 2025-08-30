use chute_kun::config::{self, Binding};
use serial_test::serial;
use std::fs;

#[test]
#[serial]
fn reads_finish_binding_from_config() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg_dir = tmp.path().join("chute_kun");
    std::env::set_var("CHUTE_KUN_CONFIG_DIR", &cfg_dir);
    fs::create_dir_all(&cfg_dir).unwrap();
    fs::write(cfg_dir.join("config.toml"), "[keybindings]\nfinish = \"x\"\n").unwrap();

    let cfg = config::load();
    match cfg.keys.finish {
        Binding::Char('x') => {}
        other => panic!("unexpected finish binding: {:?}", other),
    }
}
