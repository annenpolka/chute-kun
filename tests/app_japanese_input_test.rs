use chute_kun::app::App;
use crossterm::event::KeyCode;

// Red/Green: Input mode should accept and keep Japanese (non-ASCII) characters.
#[test]
fn input_accepts_japanese_chars() {
    let mut app = App::new();
    // Enter input mode
    app.handle_key(KeyCode::Char('i'));
    // Type "日本語"
    for ch in "日本語".chars() {
        app.handle_key(KeyCode::Char(ch));
    }
    assert!(app.in_input_mode());
    assert_eq!(app.input_buffer(), Some("日本語"));

    // First Enter: move to estimate input; second Enter: finalize
    app.handle_key(KeyCode::Enter);
    assert!(app.in_input_mode());
    app.handle_key(KeyCode::Enter);
    assert!(!app.in_input_mode());
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "日本語");
}

// Red/Green: Backspace should delete exactly one Unicode scalar (e.g., one Japanese character).
#[test]
fn backspace_removes_one_japanese_char() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    for ch in "あいう".chars() {
        app.handle_key(KeyCode::Char(ch));
    }
    assert_eq!(app.input_buffer(), Some("あいう"));
    app.handle_key(KeyCode::Backspace);
    assert_eq!(app.input_buffer(), Some("あい"));
}

// Paste support should append multi-byte strings to the buffer in input mode.
#[test]
fn paste_appends_japanese_string_in_input_mode() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_paste("予定");
    assert_eq!(app.input_buffer(), Some("予定"));
    // Regular typing continues to work
    app.handle_key(KeyCode::Char('!'));
    assert_eq!(app.input_buffer(), Some("予定!"));
}
