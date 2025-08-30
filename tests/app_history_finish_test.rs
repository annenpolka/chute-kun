use chute_kun::app::{App, View};
use chute_kun::task::TaskState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn finish_moves_task_to_history_and_today_drops_it() {
    let mut app = App::new();
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-30");
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start A
    assert_eq!(app.day.active_index(), Some(0));

    // Shift+Enter to finish
    let ev = KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT);
    app.handle_key_event(ev);

    // Today keeps A as Done; history untouched until next day
    assert_eq!(app.day.tasks.len(), 1);
    assert!(matches!(app.day.tasks[0].state, TaskState::Done));
    assert_eq!(app.history_tasks().len(), 0);

    // Next day sweep
    app.sweep_done_before(20250831);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].title, "A");

    // Switching to Past shows the item in formatted lines
    app.handle_key(KeyCode::BackTab); // Today -> Past
    assert_eq!(app.view(), View::Past);
}
