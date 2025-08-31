use chute_kun::app::App;
use chute_kun::date::today_ymd;
use crossterm::event::KeyCode;

#[test]
fn new_task_defaults_to_today_date() {
    // Fix today for determinism
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let mut app = App::new();
    // Create a normal task with default estimate (Enter twice)
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks.len(), 1);
    let t = &app.day.tasks[0];
    assert_eq!(t.planned_ymd, today_ymd());
}
