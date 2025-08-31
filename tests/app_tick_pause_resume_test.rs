use chute_kun::app::App;
use crossterm::event::KeyCode;

// 部分秒は一時停止→再開後も保持されること
#[test]
fn partial_seconds_persist_on_resume() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start

    // 59秒では分は増えない
    app.tick(59);
    assert_eq!(app.day.tasks[0].actual_min, 0);

    // 一時停止→再開（Enter のトグル）
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.active_index(), None);
    app.handle_key(KeyCode::Enter);

    // 1秒で 60秒に到達し 1分へ繰上がる（保持されていた59sが有効）
    app.tick(1);
    assert_eq!(app.day.tasks[0].actual_min, 1);
}
