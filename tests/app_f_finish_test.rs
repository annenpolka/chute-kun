use chute_kun::app::App;
use chute_kun::config::Config;
use chute_kun::task::TaskState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// Red: デフォルトで f キーで終了できること
#[test]
fn f_key_finishes_active_by_default() {
    // Use default config directly to avoid picking host config files
    let mut app = App::with_config(Config::default());
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-30");
    app.add_task("A", 30);

    // Start with Enter
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(0));

    // Press 'f' (no modifiers) -> should finish active
    let ev = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty());
    app.handle_key_event(ev);

    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks.len(), 1);
    assert!(matches!(app.day.tasks[0].state, TaskState::Done));
    assert!(app.day.tasks[0].done_ymd.is_some());
}
