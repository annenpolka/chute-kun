use chute_kun::ui::format_help_line;

// Red: キーヘルプの文言に主要なキーが含まれること
#[test]
fn help_line_includes_primary_keys() {
    let s = format_help_line();
    // quitting / navigation
    assert!(s.contains("q: quit"));
    assert!(s.contains("Tab"));
    // task lifecycle
    assert!(s.contains("Enter"));
    assert!(s.contains("Shift+Enter"));
    assert!(s.contains("start/pause"));
    // task operations
    assert!(s.contains("i: interrupt"));
    assert!(s.contains("p: postpone"));
    assert!(s.contains("b: bring"));
    assert!(s.contains("[: up"));
    assert!(s.contains("]: down"));
    assert!(s.contains("e: +5m"));
}
