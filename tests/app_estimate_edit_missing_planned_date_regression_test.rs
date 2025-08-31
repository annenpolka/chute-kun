use chute_kun::app::App;
use chute_kun::date;
use crossterm::event::KeyCode;

#[test]
fn estimate_edit_handles_missing_planned_date_without_panic() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let mut app = App::new();
    app.add_task("Legacy", 10);
    // simulate legacy snapshot where planned_ymd was not set
    app.day.tasks[0].planned_ymd = 0;

    // Open estimate editor and adjust date forward/backward
    app.handle_key(KeyCode::Char('e'));
    assert!(app.is_estimate_editing());
    app.handle_key(KeyCode::Char('.'));
    app.handle_key(KeyCode::Char(','));
    // After operations, planned_ymd must be valid and >= today
    let ymd = app.day.tasks[0].planned_ymd;
    assert!(date::is_valid_ymd(ymd));
    assert!(ymd >= date::today_ymd());
}
