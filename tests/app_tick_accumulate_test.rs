use chute_kun::app::App;

#[test]
fn tick_adds_minutes_after_60_seconds_active() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(crossterm::event::KeyCode::Enter); // start

    app.tick(30);
    assert_eq!(app.day.tasks[0].actual_min, 0);
    app.tick(30);
    assert_eq!(app.day.tasks[0].actual_min, 1);

    app.tick(120);
    assert_eq!(app.day.tasks[0].actual_min, 3);
}
