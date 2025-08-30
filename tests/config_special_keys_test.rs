use chute_kun::app::{App, View};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

fn with_config(contents: &str) -> TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let cfg_dir = tmp.path().join("chute_kun");
    std::env::set_var("CHUTE_KUN_CONFIG_DIR", &cfg_dir);
    fs::create_dir_all(&cfg_dir).unwrap();
    fs::write(cfg_dir.join("config.toml"), contents).unwrap();
    tmp
}

#[test]
#[serial]
fn finish_can_be_bound_to_char() {
    let _guard = with_config(
        r#"
start_of_day = "09:00"
[keybindings]
finish = "x"
"#,
    );

    let mut app = App::new();
    app.add_task("A", 10);
    app.handle_key(KeyCode::Enter); // start
    assert!(app.day.active_index().is_some());

    // Press custom finish key
    let ev = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
    app.handle_key_event(ev);
    assert!(app.day.active_index().is_none());
    assert_eq!(app.history_tasks().len(), 1);
}

#[test]
#[serial]
fn start_can_be_bound_to_char() {
    let _guard = with_config(
        r#"
start_of_day = "09:00"
[keybindings]
start = "s"
"#,
    );
    let mut app = App::new();
    app.add_task("A", 10);
    assert!(app.day.active_index().is_none());

    let ev = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
    app.handle_key_event(ev);
    assert!(app.day.active_index().is_some());
}

#[test]
#[serial]
fn views_can_be_switched_without_tab() {
    let _guard = with_config(
        r#"
start_of_day = "09:00"
[keybindings]
view_next = "n"
view_prev = "p"
"#,
    );
    let mut app = App::new();
    assert_eq!(app.view(), View::Today);

    // Go to Future via custom next
    let ev_n = KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE);
    app.handle_key_event(ev_n);
    assert_eq!(app.view(), View::Future);

    // Back to Today via custom prev (Future -> Today)
    let ev_p = KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE);
    app.handle_key_event(ev_p);
    assert_eq!(app.view(), View::Today);
}
