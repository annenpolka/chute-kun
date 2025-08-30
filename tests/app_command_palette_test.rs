use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

// Red: ':'でコマンドパレットをポップアップで開き、`est +15m` で選択中の見積を相対変更できる（メインは維持）
#[test]
fn command_palette_est_relative() {
    let mut app = App::new();
    app.add_task("A", 20);

    // Open palette and type command
    app.handle_key(KeyCode::Char(':'));
    let area = ratatui::layout::Rect::new(0, 0, 80, 24);
    let _popup = ui::compute_command_popup_rect(&app, area).expect("command popup");
    // Main list should still show the task line, not a Command: prompt
    let lines = ui::format_task_lines(&app);
    assert!(
        lines[0].contains("A"),
        "main list should remain while command popup open, got: {:?}",
        lines
    );
    for c in "est +15m".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks[0].estimate_min, 35);
}

// Red: `est 90m` で絶対値指定ができる（ポップアップ中）
#[test]
fn command_palette_est_absolute() {
    let mut app = App::new();
    app.add_task("A", 20);

    app.handle_key(KeyCode::Char(':'));
    let area = ratatui::layout::Rect::new(0, 0, 80, 24);
    assert!(ui::compute_command_popup_rect(&app, area).is_some());
    for c in "est 90m".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks[0].estimate_min, 90);
}
