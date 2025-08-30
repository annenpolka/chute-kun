use chute_kun::{app::App, ui::format_help_line_for};
use crossterm::event::KeyCode;

// Red: ビューに応じてヘルプ文言が最適化されること
#[test]
fn today_help_includes_task_actions() {
    let app = App::new(); // default Today
    let s = format_help_line_for(&app);
    // task lifecycle present
    assert!(s.contains("Enter"));
    assert!(s.contains("Shift+Enter"));
    assert!(s.contains("start/pause"));
    assert!(s.contains("i: interrupt"));
    assert!(s.contains("e: edit"));
    assert!(s.contains("j/k"));
    assert!(s.contains("p: postpone"));
    // navigation and quit still present
    assert!(s.contains("Tab"));
    assert!(s.contains("q: quit"));
}

#[test]
fn future_and_past_help_hide_task_actions() {
    let mut app = App::new();
    // Move to Future
    app.handle_key(KeyCode::Tab);
    let s = format_help_line_for(&app);
    assert!(!s.contains("Shift+Enter"));
    assert!(!s.contains("start/pause"));
    assert!(!s.contains("p: postpone"));
    assert!(s.contains("Tab"));
    assert!(s.contains("q: quit"));

    // Move to Past
    app.handle_key(KeyCode::BackTab);
    app.handle_key(KeyCode::BackTab);
    let s = format_help_line_for(&app);
    assert!(!s.contains("Shift+Enter"));
    assert!(!s.contains("start/pause"));
    assert!(!s.contains("p: postpone"));
    assert!(s.contains("Tab"));
    assert!(s.contains("q: quit"));
}
