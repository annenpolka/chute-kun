use chute_kun::{app::App, ui::format_help_line_for};

// Red: Todayヘルプにカテゴリー切り替えのヒントが表示される
#[test]
fn today_help_includes_category_hint() {
    let app = App::new(); // default Today
    let s = format_help_line_for(&app);
    assert!(s.contains("c: category"), "help should include 'c: category', got: {}", s);
}
