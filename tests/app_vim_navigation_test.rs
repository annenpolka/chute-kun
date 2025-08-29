use chute_kun::app::App;
use crossterm::event::KeyCode;

// Red: j/k should navigate selection like Down/Up
#[test]
fn vim_keys_move_selection_up_down() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);

    // initial
    assert_eq!(app.selected_index(), 0);

    // 'j' moves down
    app.handle_key(KeyCode::Char('j'));
    assert_eq!(app.selected_index(), 1);
    app.handle_key(KeyCode::Char('j'));
    assert_eq!(app.selected_index(), 2);
    // clamp at bottom
    app.handle_key(KeyCode::Char('j'));
    assert_eq!(app.selected_index(), 2);

    // 'k' moves up
    app.handle_key(KeyCode::Char('k'));
    assert_eq!(app.selected_index(), 1);
    app.handle_key(KeyCode::Char('k'));
    assert_eq!(app.selected_index(), 0);
    // clamp at top
    app.handle_key(KeyCode::Char('k'));
    assert_eq!(app.selected_index(), 0);
}
