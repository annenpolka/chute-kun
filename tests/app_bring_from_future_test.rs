use chute_kun::app::App;
use chute_kun::task::TaskState;
use crossterm::event::KeyCode;

// Red: Future から Today に持ってくる（bring）操作
#[test]
fn bring_moves_selected_from_future_to_today_and_updates_selection() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // A を Future に送る（postpone）
    app.handle_key(KeyCode::Char('p'));
    assert_eq!(app.tomorrow_tasks().len(), 1);
    assert_eq!(app.tomorrow_tasks()[0].title, "A");

    // Future ビューへ移動し、bring 実行
    app.handle_key(KeyCode::Tab); // Today -> Future
    app.handle_key(KeyCode::Char('b'));

    // Future は空になり、Today に B, A の順で並ぶ（末尾に追加）
    assert_eq!(app.tomorrow_tasks().len(), 0);
    assert_eq!(app.day.tasks.len(), 2);
    assert_eq!(app.day.tasks[0].title, "B");
    assert_eq!(app.day.tasks[1].title, "A");
    assert_eq!(app.day.tasks[1].state, TaskState::Planned);

    // 選択は Future 側でクランプされる（空なら 0）
    assert_eq!(app.selected_index(), 0);
}
