use chute_kun::{app::App, ui};

#[test]
fn selected_row_shows_marker_and_moves_down() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // default selection at 0
    let lines = ui::format_task_lines_at(9 * 60, &app);
    assert!(lines[0].starts_with("09:00 "));
    assert!(lines[0][6..].starts_with("▶ "));
    assert!(lines[1].starts_with("09:10 "));
    assert!(!lines[1].contains("▶ "));

    // move down and verify marker moved
    app.select_down();
    let lines = ui::format_task_lines_at(9 * 60, &app);
    assert!(lines[0].starts_with("09:00 "));
    assert!(!lines[0].contains("▶ "));
    assert!(lines[1].starts_with("09:10 "));
    assert!(lines[1][6..].starts_with("▶ "));
}
