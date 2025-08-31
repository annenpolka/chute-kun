use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn pressing_c_opens_category_picker() {
    let backend = TestBackend::new(60, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("T", 10);
    // Use keymap-aware path (Shift+c)
    app.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::SHIFT));
    assert!(app.is_category_picker(), "expected category picker to open on 'C'");
    // Render to ensure UI path does not panic
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
}
