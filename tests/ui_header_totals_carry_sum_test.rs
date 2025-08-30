use chute_kun::{app::App, ui::format_header_line};
use crossterm::event::KeyCode;

// Red: ヘッダーの Act は複数タスクの部分秒を合算して表示する
#[test]
fn header_totals_sum_carry_seconds_across_tasks() {
    let mut app = App::new();
    // A を開始して 59秒、一時停止
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter);
    app.tick(59);
    app.handle_key(KeyCode::Char(' '));

    // B を作成して開始し 30秒経過
    app.add_task("B", 30);
    app.handle_key(KeyCode::Down); // select B
    app.handle_key(KeyCode::Enter); // start B
    app.tick(30);

    // 合計は 59 + 30 = 89秒 => 1m 29s
    let header = format_header_line(9 * 60, &app);
    assert!(header.contains("Act 1m 29s"), "header: {}", header);
}
