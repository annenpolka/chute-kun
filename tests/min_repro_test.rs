use chute_kun::config::Binding;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn binding_char_matches_event() {
    let b = Binding::Char('x');
    let ev = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
    assert!(b.matches(&ev));
}
