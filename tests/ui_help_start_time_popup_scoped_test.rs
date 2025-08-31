use chute_kun::{app::App, ui::format_help_line_for};
use crossterm::event::KeyCode;

// Start Time ポップアップ中は、その操作に関係するヘルプのみ表示される
#[test]
fn help_is_scoped_while_start_time_popup_open() {
    let mut app = App::new();
    app.add_task("T", 30);
    // Space で開始時刻スライダーを開く
    app.handle_key(KeyCode::Char(' '));

    let s = format_help_line_for(&app);
    // 一般操作は含まれない
    assert!(
        !s.contains("start/pause") && !s.contains("switch view"),
        "generic help leaked into start-time popup: {}",
        s
    );
    // 時刻スライダー関連のヒントのみ
    assert!(s.contains("OK") || s.contains("Enter"), "expected OK/Enter hint: {}", s);
    assert!(s.contains("cancel") || s.contains("Esc"), "expected cancel/Esc hint: {}", s);
    assert!(s.contains("5m"), "expected +/-5m hint: {}", s);
    assert!(s.contains("slider"), "expected slider hint: {}", s);
}
