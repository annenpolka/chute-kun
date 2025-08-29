use crossterm::event::KeyCode;

use chute_kun::app::App;

#[test]
fn app_initial_state() {
    let app = App::new();
    assert_eq!(app.title, "Chute_kun");
    assert!(!app.should_quit);
}

#[test]
fn quits_on_q_key() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('q'));
    assert!(app.should_quit);
}
