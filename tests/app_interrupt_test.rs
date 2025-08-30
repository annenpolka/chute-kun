use chute_kun::app::App;
use chute_kun::task::TaskState;
use crossterm::event::KeyCode;

#[test]
fn press_i_then_enter_creates_planned_normal_task_when_empty() {
    let mut app = App::new();
    // open input, then commit without typing: uses default title
    app.handle_key(KeyCode::Char('i'));
    // Title empty -> default; first Enter opens estimate; second Enter accepts default
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks.len(), 1);
    // creating a normal task should NOT auto-start
    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks[0].state, TaskState::Planned);
    assert_ne!(app.day.tasks[0].title, "Interrupt");
}

#[test]
fn press_i_does_not_pause_current_and_creates_planned_interrupt() {
    let mut app = App::new();
    let a = app.add_task("A", 30);
    app.day.start(a);
    app.handle_key(KeyCode::Char('I'));
    // For interrupt, Enter twice to create with default estimate
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);

    // A remains active, new Interrupt is planned at the end
    let last = app.day.tasks.len() - 1;
    assert_eq!(app.day.tasks[a].state, TaskState::Active);
    assert_eq!(app.day.tasks[last].title, "Interrupt");
    assert_eq!(app.day.tasks[last].state, TaskState::Planned);
    assert_eq!(app.day.active_index(), Some(a));
}

#[test]
fn press_i_creates_planned_interrupt_when_empty() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('I'));
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks.len(), 1);
    // creating an interrupt should NOT auto-start
    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks[0].state, TaskState::Planned);
    assert_eq!(app.day.tasks[0].title, "Interrupt");
}
