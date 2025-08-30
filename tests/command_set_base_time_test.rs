use chute_kun::{app::App, config::Config, ui};
use crossterm::event::KeyCode;

// :base HH:MM で予定の基準時刻を変更できる
#[test]
fn command_base_updates_display_start_time() {
    let mut app = App::with_config(Config::default());
    app.add_task("A", 30);
    app.add_task("B", 20);

    // Open command palette and enter command
    app.handle_key(KeyCode::Char(':'));
    assert!(app.is_command_mode());
    app.handle_paste("base 10:30");
    app.handle_key(KeyCode::Enter);

    // Config should be updated in-memory
    assert_eq!(app.config.day_start_minutes, 10 * 60 + 30);

    // And UI should reflect the new base time
    let lines = ui::format_task_lines(&app);
    assert!(lines[0].starts_with("10:30 "), "expected 10:30 start, got: {}", lines[0]);
    assert!(lines[1].starts_with("11:00 "), "expected 11:00 second, got: {}", lines[1]);
}
