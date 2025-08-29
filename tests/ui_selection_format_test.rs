use chute_kun::{app::App, ui::format_task_lines};

#[test]
fn selected_row_shows_marker_and_moves_down() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // default selection at 0
    let lines = format_task_lines(&app);
    assert!(lines[0].starts_with("▶ "));
    assert!(lines[1].starts_with("  "));

    // move down and verify marker moved
    app.select_down();
    let lines = format_task_lines(&app);
    assert!(lines[0].starts_with("  "));
    assert!(lines[1].starts_with("▶ "));
}
