use chute_kun::app::App;

#[test]
fn app_can_add_task() {
    let mut app = App::new();
    assert_eq!(app.day.tasks.len(), 0);
    let idx = app.add_task("New Task", 25);
    assert_eq!(idx, 0);
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "New Task");
}

