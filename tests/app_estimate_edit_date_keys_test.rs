use chute_kun::{app::App, date};
use crossterm::event::KeyCode;

#[test]
fn estimate_edit_allows_date_change_with_keys() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let today = date::today_ymd();
    let mut app = App::new();
    app.add_task("A", 30);

    // Open estimate edit (legacy 'e' is mapped in code default)
    app.handle_key(KeyCode::Char('e'));
    assert!(app.is_estimate_editing());
    let before = app.day.tasks[0].planned_ymd;
    assert_eq!(before, today);

    // Increase date by 1 day using '.' key
    app.handle_key(KeyCode::Char('.'));
    // Close popup
    app.handle_key(KeyCode::Esc);

    let after = app.day.tasks[0].planned_ymd;
    assert_eq!(after, date::add_days_to_ymd(today, 1));
}
