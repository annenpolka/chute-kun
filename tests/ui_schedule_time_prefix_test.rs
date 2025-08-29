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
fn active_progress_shortens_next_task_scheduled_time() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);
    // Start A and add 10 minutes of progress
    app.handle_key(crossterm::event::KeyCode::Enter);
    app.day.add_actual_to_active(10);

    let lines = ui::format_task_lines_at(9 * 60, &app);
    // B should now start at 09:20 instead of 09:30
    assert!(lines[1].starts_with("09:20 "));
}

