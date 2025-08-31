use chute_kun::{app::App, config::Config, task::Category};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// Red->Green: 設定したキーでカテゴリーが切り替わる
#[test]
fn pressing_configured_category_key_cycles_category() {
    let toml = r#"
[keys]
category_cycle = "z"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse config");
    let mut app = App::with_config(cfg);
    app.add_task("Task", 10);
    assert!(matches!(app.day.tasks[0].category, Category::General));

    // Press custom key 'z'
    app.handle_key_event(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));
    assert!(matches!(app.day.tasks[0].category, Category::Work));
}

