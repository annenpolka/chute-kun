use chute_kun::{app::App, ui};

#[test]
fn task_lines_prefix_with_scheduled_time_at_left() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);

    // Base time 09:00
    let lines = ui::format_task_lines_at(9 * 60, &app);
    assert!(lines[0].starts_with("09:00 "));
    assert!(lines[1].starts_with("09:30 "));
}

#[test]
fn active_progress_does_not_change_next_task_scheduled_time() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);
    // Start A and add 10 minutes of progress
    app.handle_key(crossterm::event::KeyCode::Enter);
    app.day.add_actual_to_active(10);

    let lines = ui::format_task_lines_at(9 * 60, &app);
    // Plan should not pull in by ACT time; B stays at 09:30
    assert!(lines[1].starts_with("09:30 "));
}
