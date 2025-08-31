use chute_kun::{app::App, date};
use crossterm::event::KeyCode;

#[test]
fn date_picker_can_set_tomorrow_and_place_in_future() {
    // Fix today for determinism
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let today = date::today_ymd();
    let mut app = App::new();

    // Start creating a task titled "A"
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Char('A'));
    // Move to estimate step (date picker visible there)
    app.handle_key(KeyCode::Enter);
    assert!(app.is_new_task_estimate());
    // Increase planned date by one day via picker key
    app.handle_key(KeyCode::Char('.'));
    // Accept (Enter)
    app.handle_key(KeyCode::Enter);

    // Should be placed into Future list, not Today
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.tomorrow_tasks().len(), 1);
    let t = &app.tomorrow_tasks()[0];
    assert_eq!(t.title, "A");
    assert_eq!(t.planned_ymd, date::add_days_to_ymd(today, 1));
}
