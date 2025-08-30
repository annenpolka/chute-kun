use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;

// 新仕様: 削除確認中でもメインリストは維持される（プロンプト行で置換しない）
#[test]
fn main_list_stays_while_confirm_delete() {
    let mut app = App::new();
    app.add_task("To Delete", 5);

    // Before confirm, first line should include the task title
    let before = ui::format_task_lines(&app);
    assert!(before[0].contains("To Delete"));

    // Trigger delete confirm; underlying list should remain the same in formatting output
    app.handle_key(KeyCode::Char('x'));
    let during = ui::format_task_lines(&app);
    assert!(during[0].contains("To Delete"), "list should remain while confirming");
    assert!(!during[0].contains("Delete?"), "main content should not be replaced by delete prompt");
}
