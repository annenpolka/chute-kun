use chute_kun::{app::App, config::Config};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// Red: コンフィグで delete キーが変更でき、キーイベントも動作する
#[test]
fn config_overrides_delete_key_and_triggers_confirm() {
    let toml = r#"
day_start = "09:00"

[keys]
delete = "Ctrl+D"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse config");
    let mut app = App::with_config(cfg);
    app.add_task("A", 10);

    // Press configured delete key
    let ev = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL);
    app.handle_key_event(ev);

    assert!(app.is_confirm_delete(), "should open confirm delete on configured key");
}
