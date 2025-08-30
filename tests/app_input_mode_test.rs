use chute_kun::app::App;
use chute_kun::task::TaskState;
use crossterm::event::KeyCode;

#[test]
fn i_enters_input_and_enter_adds_typed_title() {
    let mut app = App::new();
    // Enter input mode for normal task
    app.handle_key(KeyCode::Char('i'));
    // Type a title: "Hi"
    app.handle_key(KeyCode::Char('H'));
    app.handle_key(KeyCode::Char('i'));
    // Commit
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "Hi");
    assert_eq!(app.day.tasks[0].state, TaskState::Planned);
}

#[test]
fn i_enters_interrupt_input_and_uses_interrupt_defaults() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('I'));
    app.handle_key(KeyCode::Char('U'));
    app.handle_key(KeyCode::Char('r'));
    app.handle_key(KeyCode::Char('g'));
    app.handle_key(KeyCode::Char('e'));
    app.handle_key(KeyCode::Char('n'));
    app.handle_key(KeyCode::Char('t'));
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "Urgent");
    // Interrupt default estimate is 15m
    assert_eq!(app.day.tasks[0].estimate_min, 15);
}

#[test]
fn esc_cancels_input_without_adding() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Char('A'));
    app.handle_key(KeyCode::Esc);
    assert_eq!(app.day.tasks.len(), 0);
}

#[test]
fn backspace_edits_buffer_before_commit() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Char('A'));
    app.handle_key(KeyCode::Char('b'));
    app.handle_key(KeyCode::Backspace);
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "A");
}
