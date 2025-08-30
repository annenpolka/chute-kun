use chute_kun::{app::App, ui::format_help_line_for};
use crossterm::event::KeyCode;

// Red: Future ビューのヘルプに bring 操作が表示される
#[test]
fn future_help_includes_bring_only() {
    let mut app = App::new();
    // Future に移動
    app.handle_key(KeyCode::Tab);
    let s = format_help_line_for(&app);

    // bring が表示され、ナビ要素は維持
    assert!(s.contains("b: bring"));
    assert!(s.contains("q: quit"));
    assert!(s.contains("Tab: switch view"));

    // Today 専用の操作は含まれない
    assert!(!s.contains("Shift+Enter"));
    assert!(!s.contains("start/pause"));
    assert!(!s.contains("p: postpone"));
}
