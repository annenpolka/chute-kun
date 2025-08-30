use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

// Red: 削除確認中はプロンプト行が表示される（タイトルを含む）
#[test]
fn shows_delete_confirmation_prompt_line() {
    let mut app = App::new();
    app.add_task("To Delete", 5);

    // Trigger delete confirm
    app.handle_key(KeyCode::Char('x'));

    // Use the same formatting helper used elsewhere to verify the prompt line text
    let lines = ui::format_task_lines(&app);
    let first = lines.first().unwrap().to_string();
    assert!(first.contains("Delete"), "expected a delete confirmation line, got: {}", first);
    assert!(first.contains("To Delete"), "should mention task title, got: {}", first);
    assert!(first.contains("Enter=Delete"), "should show Enter=Delete hint, got: {}", first);
}

