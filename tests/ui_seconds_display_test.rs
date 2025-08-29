use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

#[test]
fn header_and_task_lines_show_seconds() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start A

    // 経過5秒
    app.tick(5);

    // ヘッダーに "Act 0m 5s" を含む
    let header = ui::format_header_line(9 * 60, &app);
    assert!(header.contains("Act 0m 5s"));

    // タスクリスト行にも秒が表示される
    let lines = ui::format_task_lines(&app);
    assert!(lines.iter().any(|l| l.contains("act:0m 5s")));
}
