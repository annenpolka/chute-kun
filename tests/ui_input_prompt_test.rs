use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

#[test]
fn input_mode_keeps_main_and_shows_popup() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Char('T'));
    app.handle_key(KeyCode::Char('e'));
    app.handle_key(KeyCode::Char('s'));
    app.handle_key(KeyCode::Char('t'));
    // Popup should be visible, but main content stays (no tasks yet -> default hint)
    let area = ratatui::layout::Rect::new(0, 0, 80, 24);
    assert!(ui::compute_input_popup_rect(&app, area).is_some());
    let lines = ui::format_task_lines(&app);
    assert!(
        lines.first().unwrap().contains("No tasks"),
        "main list hint should remain, got: {:?}",
        lines
    );
}
