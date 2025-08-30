use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

// Red: ':'でコマンドパレットを開き、`est +15m` で選択中の見積を相対変更できる
#[test]
fn command_palette_est_relative() {
    let mut app = App::new();
    app.add_task("A", 20);

    // Open palette and type command
    app.handle_key(KeyCode::Char(':'));
    let lines = ui::format_task_lines(&app);
    assert!(lines.first().unwrap().starts_with("Command:"));
    for c in "est +15m".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks[0].estimate_min, 35);
}

// Red: `est 90m` で絶対値指定ができる
#[test]
fn command_palette_est_absolute() {
    let mut app = App::new();
    app.add_task("A", 20);

    app.handle_key(KeyCode::Char(':'));
    for c in "est 90m".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    assert_eq!(app.day.tasks[0].estimate_min, 90);
}
