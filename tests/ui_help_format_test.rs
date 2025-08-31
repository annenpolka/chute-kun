use chute_kun::{app::App, ui::format_help_line_for};

// Red: キーヘルプの文言に主要なキーが含まれること
#[test]
fn help_line_includes_primary_keys() {
    let app = App::new();
    let s = format_help_line_for(&app);
    // quitting / navigation
    assert!(s.contains("q: quit"));
    assert!(s.contains("Tab"));
    // task lifecycle
    assert!(s.contains("Enter"));
    assert!(s.contains("Shift+Enter"));
    assert!(s.contains("start/pause"));
    // task operations
    assert!(s.contains("Shift+i: interrupt"));
    assert!(s.contains("p: postpone"));
    assert!(s.contains("[: up"));
    assert!(s.contains("]: down"));
    assert!(s.contains("e: edit"));
    assert!(s.contains("j/k"));
}
