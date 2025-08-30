use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

// Red: E で見積ポップアップに入り、↑/↓で5分刻み調整、Enterで確定して抜ける（メインは維持）
#[test]
fn estimate_popup_opens_updates_and_confirms_keeping_main() {
    let mut app = App::new();
    app.add_task("A", 20);

    // Enter estimate edit mode
    app.handle_key(KeyCode::Char('e'));
    let area = ratatui::layout::Rect::new(0, 0, 80, 24);
    assert!(ui::compute_estimate_popup_rect(&app, area).is_some());
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("est:20m"));

    // Up increases by 5m
    app.handle_key(KeyCode::Up);
    let lines = ui::format_task_lines(&app);
    assert!(
        lines.first().unwrap().contains("est:25m"),
        "expected est:25m after Up, got: {:?}",
        lines
    );

    // 'j' decreases by 5m
    app.handle_key(KeyCode::Char('j'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("est:20m"));

    // 'k' increases by 5m
    app.handle_key(KeyCode::Char('k'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("est:25m"));

    // Down decreases by 5m
    app.handle_key(KeyCode::Down);
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("est:20m"));

    // Confirm and exit
    app.handle_key(KeyCode::Enter);
    assert!(!app.in_input_mode(), "should exit stepper on Enter");
}
