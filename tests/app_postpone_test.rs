use crossterm::event::KeyCode;
use chute_kun::app::App;
use chute_kun::task::TaskState;

#[test]
fn postpone_moves_selected_to_tomorrow_and_updates_selection() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // select first (default)
    app.handle_key(KeyCode::Char('p'));

    // Today should now have only B
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "B");
    // Tomorrow outbox has A
    assert_eq!(app.tomorrow_tasks().len(), 1);
    assert_eq!(app.tomorrow_tasks()[0].title, "A");
    // Selection clamps to 0
    assert_eq!(app.selected_index(), 0);
}

#[test]
fn postpone_active_task_clears_active() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.handle_key(KeyCode::Enter); // start A
    assert_eq!(app.day.active_index(), Some(0));

    app.handle_key(KeyCode::Char('p'));
    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.tomorrow_tasks().len(), 1);
    assert_eq!(app.tomorrow_tasks()[0].state, TaskState::Planned); // stays planned
}

