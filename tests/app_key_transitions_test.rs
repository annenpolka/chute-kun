use chute_kun::app::App;
use chute_kun::task::TaskState;
use crossterm::event::KeyCode;

#[test]
fn enter_space_shiftenter_transitions() {
    let mut app = App::new();
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-30");
    app.add_task("A", 30);

    // Enter: Start
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(0));

    // Enter again: Pause (Space no longer pauses)
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Paused);
    assert_eq!(app.day.active_index(), None);

    // Enter: Resume
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(0));

    // Finish (today): stays in Today as Done
    app.finish_active();
    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks.len(), 1);
    assert!(matches!(app.day.tasks[0].state, TaskState::Done));

    // Next day sweep moves it to history
    app.sweep_done_before(20250831);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
    assert!(matches!(app.history_tasks()[0].state, TaskState::Done));
}
