use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

#[test]
fn key_e_opens_estimate_popup_and_updates_main_list() {
    let mut app = App::new();
    app.add_task("A", 20);
    app.handle_key(KeyCode::Char('e'));
    // Popup should be active, but main list stays rendered
    let area = ratatui::layout::Rect::new(0, 0, 80, 24);
    assert!(ui::compute_estimate_popup_rect(&app, area).is_some());
    let lines = ui::format_task_lines(&app);
    assert!(
        lines[0].contains("A") && lines[0].contains("est:20m"),
        "main list should remain with est:20m, got: {:?}",
        lines
    );
    // Up to +5
    app.handle_key(KeyCode::Up);
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("est:25m"));
    // Enter to confirm
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks[0].estimate_min, 25);
}
