use chute_kun::{app::App, ui::format_help_line_for};

// Today のヘルプに Space の開始時刻操作が含まれる
#[test]
fn today_help_includes_space_time() {
    let app = App::new();
    let s = format_help_line_for(&app);
    assert!(s.contains("Space") && s.contains("time"), "help lacks Space: time: {}", s);
}
