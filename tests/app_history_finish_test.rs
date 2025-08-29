use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use chute_kun::app::{App, View};
use chute_kun::task::TaskState;

#[test]
fn finish_moves_task_to_history_and_today_drops_it() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start A
    assert_eq!(app.day.active_index(), Some(0));

    // Shift+Enter to finish
    let ev = KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT);
    app.handle_key_event(ev);

    // Today becomes empty
    assert_eq!(app.day.tasks.len(), 0);
    // History has A with Done
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].title, "A");
    assert_eq!(app.history_tasks()[0].state, TaskState::Done);

    // Switching to Past shows the item in formatted lines
    app.handle_key(KeyCode::BackTab); // Today -> Past
    assert_eq!(app.view(), View::Past);
}

