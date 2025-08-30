use chute_kun::config::{Action, Config};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn default_keymap_maps_f_to_finish() {
    let cfg = Config::default();
    let ev = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty());
    let act = cfg.keys.action_for(&ev);
    assert!(matches!(act, Some(Action::FinishActive)), "expected FinishActive, got: {:?}", act);
}
