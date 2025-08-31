use chute_kun::{app::App, ui::format_help_line_for};
use crossterm::event::KeyCode;

// Red: ポップアップ中は、その操作に関係するヘルプのみ表示する（削減される）
#[test]
fn help_is_scoped_while_confirm_delete_popup_open() {
    let mut app = App::new();
    // 前提: タスクが1つある状態で削除確認ポップアップを開く
    app.add_task("Sample", 25);
    app.handle_key(KeyCode::Char('x'));
    let s = format_help_line_for(&app);
    // 一般操作は含まれない
    assert!(
        !s.contains("start/pause") && !s.contains("switch view"),
        "generic help leaked into delete popup: {}",
        s
    );
    // 削除ポップアップに関係する操作のみ
    assert!(s.contains("delete"), "expected delete hint in popup help: {}", s);
    assert!(s.contains("cancel"), "expected cancel hint in popup help: {}", s);
}

#[test]
fn help_is_scoped_while_estimate_edit_popup_open() {
    let mut app = App::new();
    app.add_task("Edit me", 30);
    // 見積編集ポップアップを開く（'e'）
    app.handle_key(KeyCode::Char('e'));
    let s = format_help_line_for(&app);
    // 一般操作は含まれない
    assert!(
        !s.contains("start/pause") && !s.contains("switch view"),
        "generic help leaked into estimate popup: {}",
        s
    );
    // 見積編集に関係する操作のみ（5分刻み、OK/Cancel、日付操作）
    assert!(s.contains("5m"), "expected +/-5m hint in popup help: {}", s);
    assert!(s.contains("OK") || s.contains("Enter"), "expected OK/Enter hint: {}", s);
    assert!(s.contains("cancel") || s.contains("Esc"), "expected cancel/Esc hint: {}", s);
}
