use chute_kun::{app::App, ui::format_task_lines};
use crossterm::event::KeyCode;

#[test]
fn switch_today_future_past_views_render_expected_lists() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // Postpone selected (A) to tomorrow
    app.handle_key(KeyCode::Char('p'));

    // Today view (default) should show only B
    let lines = format_task_lines(&app);
    assert!(lines.iter().any(|l| l.contains("B")));
    assert!(!lines.iter().any(|l| l.contains("A")));

    // Go to Future (Tab)
    app.handle_key(KeyCode::Tab);
    let lines = format_task_lines(&app);
    assert!(lines.iter().any(|l| l.contains("A")));
    assert!(!lines.iter().any(|l| l.contains("B")));

    // Go to Past (BackTab)
    app.handle_key(KeyCode::BackTab);
    app.handle_key(KeyCode::BackTab);
    let lines = format_task_lines(&app);
    assert!(lines.iter().any(|l| l.contains("No tasks")));
}
