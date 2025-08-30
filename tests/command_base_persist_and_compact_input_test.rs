use chute_kun::{app::App, config::Config, ui};
use crossterm::event::KeyCode;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

// Red: base が永続化され、UIにも反映される（HH:MM と HHMM の両方）
#[test]
fn base_persists_and_updates_ui() {
    let dir = tempdir().unwrap();
    let path: PathBuf = dir.path().join("config.toml");
    std::env::set_var("CHUTE_KUN_CONFIG", &path);

    let mut app = App::with_config(Config::load());
    app.add_task("A", 30);
    app.add_task("B", 20);

    app.handle_key(KeyCode::Char(':'));
    app.handle_paste("base 10:00");
    app.handle_key(KeyCode::Enter);

    // UI start time reflects 10:00
    let lines = ui::format_task_lines(&app);
    assert!(lines[0].starts_with("10:00 "));

    // Config file written with day_start = "10:00"
    let s = fs::read_to_string(&path).expect("config was written");
    assert!(s.contains("day_start = \"10:00\""), "missing updated day_start: {}", s);

    // Now verify compact input in the same (serial) test
    let dir2 = tempdir().unwrap();
    let path2: PathBuf = dir2.path().join("config.toml");
    std::env::set_var("CHUTE_KUN_CONFIG", &path2);

    let mut app2 = App::with_config(Config::load());
    app2.add_task("A", 15);
    app2.handle_key(KeyCode::Char(':'));
    app2.handle_paste("base 1000");
    app2.handle_key(KeyCode::Enter);

    let lines2 = ui::format_task_lines(&app2);
    assert!(lines2[0].starts_with("10:00 "));
    let s2 = fs::read_to_string(&path2).expect("config was written");
    assert!(s2.contains("day_start = \"10:00\""));
}
