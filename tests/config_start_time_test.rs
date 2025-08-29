use chute_kun::{app::App, ui};
use std::fs;
use std::path::PathBuf;

fn write_test_config(dir: &PathBuf, contents: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("config.toml"), contents).unwrap();
}

#[test]
fn default_start_of_day_is_09_00_for_display() {
    // Without any config, display helpers should start at 09:00, not now.
    let mut app = App::new();
    app.add_task("A", 30);

    let lines = ui::format_task_lines(&app);
    assert!(lines[0].starts_with("09:00 "));
}

#[test]
fn start_of_day_can_be_overridden_by_config_toml() {
    // Point config dir to a temp path containing config.toml
    let tmp = tempfile::tempdir().unwrap();
    let cfg_dir = tmp.path().join("chute_kun");
    let cfg_path: PathBuf = cfg_dir.clone();
    std::env::set_var("CHUTE_KUN_CONFIG_DIR", &cfg_path);
    write_test_config(&cfg_dir, "start_of_day = \"08:15\"\n");

    let mut app = App::new();
    app.add_task("A", 10);
    let lines = ui::format_task_lines(&app);
    assert!(lines[0].starts_with("08:15 "));
}

