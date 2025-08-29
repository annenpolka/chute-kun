use chute_kun::{app::App, config::Config};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serial_test::serial;

fn write_config(dir: &std::path::Path, toml: &str) {
    let confdir = dir.join("chute_kun");
    std::fs::create_dir_all(&confdir).unwrap();
    std::fs::write(confdir.join("config.toml"), toml).unwrap();
}

#[test]
#[serial]
fn loads_start_of_day_from_toml() {
    let tmp = tempfile::tempdir().unwrap();
    write_config(tmp.path(), "start_of_day = '05:30'\n");
    std::env::set_var("XDG_CONFIG_HOME", tmp.path());

    let cfg = Config::load().expect("config loads");
    assert_eq!(cfg.start_of_day_min, Some(5 * 60 + 30));
}

#[test]
#[serial]
fn app_uses_custom_quit_key() {
    let tmp = tempfile::tempdir().unwrap();
    write_config(
        tmp.path(),
        r#"
start_of_day = '09:00'
[keys]
quit = 'x'
"#,
    );
    std::env::set_var("XDG_CONFIG_HOME", tmp.path());

    let mut app = App::new();
    assert!(!app.should_quit);
    let ev = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
    app.handle_key_event(ev);
    assert!(app.should_quit);
}

#[test]
#[serial]
fn app_schedule_base_uses_start_of_day_when_set() {
    let tmp = tempfile::tempdir().unwrap();
    write_config(tmp.path(), "start_of_day = '07:15'\n");
    std::env::set_var("XDG_CONFIG_HOME", tmp.path());

    let app = App::new();
    // Even if 'now' is 09:00, schedule base should honor 07:15 from config
    assert_eq!(app.schedule_start_minute_from(9 * 60), 7 * 60 + 15);
}
