use chute_kun::{app::App, task::Task};
use chute_kun::ui::format_task_lines;

#[test]
fn shows_hint_when_no_tasks_then_title_after_add() {
    let mut app = App::new();
    let lines = format_task_lines(&app);
    assert!(lines.iter().any(|l| l.contains("No tasks")));

    app.add_task("Hello", 10);
    let lines = format_task_lines(&app);
    assert!(lines.iter().any(|l| l.contains("Hello")));
}

