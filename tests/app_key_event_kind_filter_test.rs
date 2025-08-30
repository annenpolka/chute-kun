use chute_kun::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent { code, modifiers: KeyModifiers::empty(), kind: KeyEventKind::Press, state: KeyEventState::NONE }
}

fn release(code: KeyCode) -> KeyEvent {
    KeyEvent { code, modifiers: KeyModifiers::empty(), kind: KeyEventKind::Release, state: KeyEventState::NONE }
}

#[test]
fn ignores_release_in_input_mode_for_ascii() {
    let mut app = App::new();
    // Enter input mode via mapped key 'i' (Press + Release)
    app.handle_key_event(press(KeyCode::Char('i')));
    app.handle_key_event(release(KeyCode::Char('i')));
    assert!(app.in_input_mode());

    // Type 'a' with Press + Release should append only once
    app.handle_key_event(press(KeyCode::Char('a')));
    app.handle_key_event(release(KeyCode::Char('a')));
    assert_eq!(app.input_buffer(), Some("a"));
}

#[test]
fn ignores_release_in_input_mode_for_japanese_char() {
    let mut app = App::new();
    app.handle_key_event(press(KeyCode::Char('i')));
    assert!(app.in_input_mode());

    // Simulate IME-committed char as normal Char key event
    let ch = '日';
    app.handle_key_event(press(KeyCode::Char(ch)));
    app.handle_key_event(release(KeyCode::Char(ch)));
    assert_eq!(app.input_buffer(), Some("日"));
}

#[test]
fn action_mapping_triggers_only_on_press() {
    let mut app = App::new();
    assert!(!app.in_input_mode());
    // 'i' maps to AddTask (enter input mode). Release should be ignored.
    app.handle_key_event(press(KeyCode::Char('i')));
    app.handle_key_event(release(KeyCode::Char('i')));
    assert!(app.in_input_mode());
}

