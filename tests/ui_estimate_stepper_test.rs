use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

// Red: E で見積ステッパーに入り、↑/↓で5分刻み調整、Enterで確定して抜ける
#[test]
fn estimate_stepper_opens_updates_and_confirms() {
    let mut app = App::new();
    app.add_task("A", 20);

    // Enter estimate edit mode
    app.handle_key(KeyCode::Char('e'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().starts_with("Estimate:"), "expected stepper line, got: {:?}", lines);
    assert!(lines.first().unwrap().contains("20m"));

    // Up increases by 5m
    app.handle_key(KeyCode::Up);
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("25m"), "expected 25m after Up, got: {:?}", lines);

    // 'j' decreases by 5m
    app.handle_key(KeyCode::Char('j'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("20m"));

    // 'k' increases by 5m
    app.handle_key(KeyCode::Char('k'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("25m"));

    // Down decreases by 5m
    app.handle_key(KeyCode::Down);
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().contains("20m"));

    // Confirm and exit
    app.handle_key(KeyCode::Enter);
    assert!(!app.in_input_mode(), "should exit stepper on Enter");
}
