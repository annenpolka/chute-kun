use chute_kun::app::App;
use chute_kun::task::TaskState;
use crossterm::event::KeyCode;

// Enter キーで開始/一時停止をトグルできること
#[test]
fn enter_toggles_start_and_pause() {
    let mut app = App::new();
    app.add_task("A", 30);

    // 1回目の Enter で開始
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(0));

    // 2回目の Enter で一時停止（従来は Space）
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Paused);
    assert_eq!(app.day.active_index(), None);

    // 3回目の Enter で再開
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(0));
}

