use chute_kun::app::App;
use crossterm::event::KeyCode;

// Red: x で確認ダイアログ → Enter で削除、Esc でキャンセル
#[test]
fn delete_confirmation_flow() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 10);
    assert_eq!(app.day.tasks.len(), 2);

    // Open confirm for delete
    app.handle_key(KeyCode::Char('x'));
    // While confirming, Enter should delete selected (index 0 by default)
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "B");

    // Open confirm again, then Esc cancels (no deletion)
    app.handle_key(KeyCode::Char('x'));
    app.handle_key(KeyCode::Esc);
    assert_eq!(app.day.tasks.len(), 1);
}

