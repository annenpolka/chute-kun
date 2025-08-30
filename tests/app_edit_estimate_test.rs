use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

#[test]
fn key_e_opens_stepper_and_updates_by_5_on_up() {
    let mut app = App::new();
    app.add_task("A", 20);
    app.handle_key(KeyCode::Char('e'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().starts_with("Estimate:"));
    // Up to +5
    app.handle_key(KeyCode::Up);
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("25m"));
    // Enter to confirm
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].estimate_min, 25);
}
