use chute_kun::app::App;
use crossterm::event::KeyCode;

#[test]
fn key_e_increases_selected_estimate_by_5() {
    let mut app = App::new();
    app.add_task("A", 20);
    app.handle_key(KeyCode::Char('e'));
    assert_eq!(app.day.tasks[0].estimate_min, 25);
}
