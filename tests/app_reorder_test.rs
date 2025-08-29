use chute_kun::app::App;
use crossterm::event::KeyCode;

#[test]
fn reorder_down_and_up_updates_order_and_selection() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);

    // select middle (B)
    app.select_down();
    assert_eq!(app.selected_index(), 1);

    // move down with ']'
    app.handle_key(KeyCode::Char(']'));
    let titles: Vec<_> = app.day.tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["A", "C", "B"]);
    assert_eq!(app.selected_index(), 2);

    // move up with '['
    app.handle_key(KeyCode::Char('['));
    let titles: Vec<_> = app.day.tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["A", "B", "C"]);
    assert_eq!(app.selected_index(), 1);
}
