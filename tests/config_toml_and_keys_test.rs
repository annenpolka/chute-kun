use chute_kun::{app::App, config::Config, ui};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn parse_toml_day_start_and_custom_quit_key() {
    let toml = r#"
day_start = "08:15"

[keys]
quit = "x"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse config");
    let mut app = App::with_config(cfg);
    app.add_task("A", 30);

    // Display should start at 08:15
    let lines = ui::format_task_lines(&app);
    assert!(lines[0].starts_with("08:15 "), "got: {}", lines[0]);

    // Custom quit key should be recognized
    let ev = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
    app.handle_key_event(ev);
    assert!(app.should_quit);
}

