use chute_kun::{app::App, ui::format_help_line_for};

#[test]
fn today_help_includes_category_picker_hint() {
    let app = App::new();
    let s = format_help_line_for(&app);
    assert!(s.contains("Shift+c: picker"), "help should include 'Shift+c: picker', got: {}", s);
}
