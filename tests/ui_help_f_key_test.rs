use chute_kun::ui::format_help_line;

// Red: ヘルプ文言に f: finish が含まれること（デフォルト）
#[test]
fn help_includes_f_key_for_finish() {
    let s = format_help_line();
    assert!(s.contains("f: finish"), "help should include 'f: finish', got: {}", s);
}
