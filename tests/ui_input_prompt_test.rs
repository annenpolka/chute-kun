use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

#[test]
fn input_mode_shows_prompt_line() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Char('T'));
    app.handle_key(KeyCode::Char('e'));
    app.handle_key(KeyCode::Char('s'));
    app.handle_key(KeyCode::Char('t'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.get(0).unwrap().starts_with("Input: Test"));
}

