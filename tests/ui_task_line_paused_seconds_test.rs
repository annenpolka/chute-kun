use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

// Red: タスクリスト行でも一時停止中のタスクに秒が表示される
#[test]
fn task_line_shows_seconds_while_paused() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start A

    app.tick(59); // 59sec elapsed
    app.handle_key(KeyCode::Enter); // pause (toggle)

    let lines = ui::format_task_lines(&app);
    let row = lines.iter().find(|l| l.contains("A")).expect("row for A");
    assert!(row.contains("act:0m 59s"), "row: {}", row);
}
