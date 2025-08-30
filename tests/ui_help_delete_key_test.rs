use chute_kun::ui::format_help_line_for;
use chute_kun::app::App;

// Red: ヘルプに x: delete が表示される（Today ビュー）
#[test]
fn help_includes_delete_key_on_today() {
    let app = App::new(); // default Today view
    let s = format_help_line_for(&app);
    assert!(s.contains("x: delete"), "help should include 'x: delete', got: {}", s);
}

