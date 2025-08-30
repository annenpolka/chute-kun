use chute_kun::app::App;
use crossterm::event::KeyCode;

#[test]
fn after_title_enter_prompts_for_estimate_and_accepts_digits() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    for ch in "Hi".chars() {
        app.handle_key(KeyCode::Char(ch));
    }
    // First Enter opens estimate input
    app.handle_key(KeyCode::Enter);
    assert!(app.is_new_task_estimate());
    assert_eq!(app.new_task_title(), Some("Hi"));
    // Adjust by slider keys to 40 (25 + 3*5)
    app.handle_key(KeyCode::Up);
    app.handle_key(KeyCode::Up);
    app.handle_key(KeyCode::Up);
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "Hi");
    assert_eq!(app.day.tasks[0].estimate_min, 40);
}

#[test]
fn interrupt_default_estimate_is_15_when_confirmed() {
    let mut app = App::new();
    app.handle_key(KeyCode::Char('I'));
    // No title typed -> default "Interrupt"
    app.handle_key(KeyCode::Enter);
    // Estimate step open; buffer may start empty but Enter should accept 15 by default
    assert!(app.is_new_task_estimate());
    // Accept
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "Interrupt");
    assert_eq!(app.day.tasks[0].estimate_min, 15);
}
