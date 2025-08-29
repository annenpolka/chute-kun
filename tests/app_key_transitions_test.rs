use crossterm::event::{KeyCode, KeyModifiers};
use chute_kun::app::App;
use chute_kun::task::TaskState;

#[test]
fn enter_space_shiftenter_transitions() {
    let mut app = App::new();
    app.add_task("A", 30);

    // Enter: Start
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(0));

    // Space: Pause
    app.handle_key(KeyCode::Char(' '));
    assert_eq!(app.day.tasks[0].state, TaskState::Paused);
    assert_eq!(app.day.active_index(), None);

    // Enter: Resume
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(0));

    // Shift+Enter: Finish
    // crossterm provides modifiers in KeyEvent; our `handle_key` takes only KeyCode for now,
    // so we simulate Finish via a dedicated method later; for now we call a special code path by sending `KeyCode::Null` is not ideal.
    // To keep the test executable, we will add a helper on App to finish_active() and call it here.
    app.finish_active();
    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].state, TaskState::Done);
}
