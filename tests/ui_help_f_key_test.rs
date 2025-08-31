use chute_kun::{app::App, ui::format_help_line_for};

// Red: ヘルプ文言に f: finish が含まれること（デフォルト）
#[test]
fn help_includes_f_key_for_finish() {
    let app = App::new();
    let s = format_help_line_for(&app);
    assert!(s.contains("f: finish"), "help should include 'f: finish', got: {}", s);
}
