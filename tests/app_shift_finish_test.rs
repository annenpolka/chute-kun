use chute_kun::app::App;
use chute_kun::task::TaskState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn shift_enter_finishes_active() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start
    assert_eq!(app.day.tasks[0].state, TaskState::Active);

    let ev = KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT);
    app.handle_key_event(ev);
    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].state, TaskState::Done);
}
