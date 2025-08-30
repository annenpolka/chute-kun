use chute_kun::{app::App, ui::format_header_line};
use crossterm::event::KeyCode;

// Red: 一時停止中でもヘッダーの Act に秒が切り捨てられず表示されること
#[test]
fn header_shows_seconds_while_paused() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start A

    // 59秒経過
    app.tick(59);

    // 一時停止
    app.handle_key(KeyCode::Char(' '));

    // 09:00 時点のヘッダーで 59秒が表示されていること（切り捨てない）
    let header = format_header_line(9 * 60, &app);
    assert!(header.contains("Act 0m 59s"), "header: {}", header);
}
