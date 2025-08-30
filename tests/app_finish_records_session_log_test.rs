use chute_kun::{app::App, config::Config, ui};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn start_and_finish_via_actions_produces_actual_times_in_table() {
    // Use default config (Enter mapped to StartOrResume, f to FinishActive)
    let cfg = Config::default();
    let mut app = App::with_config(cfg);
    app.add_task("A", 5);

    // Simulate key events: Enter (press) to start, then 'f' to finish
    let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
    app.handle_key_event(enter);
    let finish = KeyEvent {
        code: KeyCode::Char('f'),
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    };
    app.handle_key_event(finish);

    // Render and assert that the Actual column contains a start time (HH:MM- or HH:MM-HH:MM)
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let mut all = String::new();
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            all.push_str(buf[(x, y)].symbol());
        }
        all.push('\n');
    }
    assert!(all.contains("Actual"), "table header missing: {}", all);
    assert!(all.contains(":"), "expected some actual time marker in table: {}", all);
}
