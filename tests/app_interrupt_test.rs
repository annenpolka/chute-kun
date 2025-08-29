use crossterm::event::KeyCode;
use chute_kun::app::App;
use chute_kun::task::TaskState;

#[test]
fn press_i_creates_active_interrupt_when_empty() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.active_index(), Some(0));
    assert_eq!(app.day.tasks[0].state, TaskState::Active);
    assert_eq!(app.day.tasks[0].title, "Interrupt");
}

#[test]
fn press_i_pauses_current_and_starts_new_interrupt() {
    let mut app = App::new();
    let a = app.add_task("A", 30);
    app.day.start(a);
    app.handle_key(KeyCode::Char('i'));

    // A is paused, Interrupt is active at the end
    let last = app.day.tasks.len() - 1;
    assert_eq!(app.day.tasks[a].state, TaskState::Paused);
    assert_eq!(app.day.tasks[last].title, "Interrupt");
    assert_eq!(app.day.tasks[last].state, TaskState::Active);
    assert_eq!(app.day.active_index(), Some(last));
}

