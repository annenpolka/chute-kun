use chute_kun::app::{App, DisplayMode};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

fn press(ch: char) -> KeyEvent {
    KeyEvent {
        code: KeyCode::Char(ch),
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    }
}

#[test]
fn q_in_calendar_returns_to_list_instead_of_quitting() {
    let mut app = App::new();
    assert!(matches!(app.display_mode(), DisplayMode::List));
    // Enter Calendar mode
    app.toggle_display_mode();
    assert!(matches!(app.display_mode(), DisplayMode::Calendar));
    // Press 'q' via key event path (respects keymap Action::Quit)
    app.handle_key_event(press('q'));
    assert!(matches!(app.display_mode(), DisplayMode::List), "should return to List");
    assert!(!app.should_quit, "should not quit from Calendar on 'q'");
}

#[test]
fn q_in_list_quits_as_before() {
    let mut app = App::new();
    assert!(matches!(app.display_mode(), DisplayMode::List));
    app.handle_key_event(press('q'));
    assert!(app.should_quit, "should quit from List on 'q'");
}
