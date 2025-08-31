use chute_kun::{app::App, date};
use crossterm::event::KeyCode;

#[test]
fn postpone_sets_planned_date_to_tomorrow() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let today = date::today_ymd();
    let mut app = App::new();
    app.add_task("A", 10);
    // Postpone selected task
    app.handle_key(KeyCode::Char('p'));

    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.tomorrow_tasks().len(), 1);
    let t = &app.tomorrow_tasks()[0];
    assert_eq!(t.title, "A");
    assert_eq!(t.planned_ymd, date::add_days_to_ymd(today, 1));
}
