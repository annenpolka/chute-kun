use chute_kun::task::{DayPlan, Task, TaskState};

#[test]
fn add_task_appends_and_returns_index() {
    let mut day = DayPlan::new(vec![Task::new("A", 30)]);
    let idx = day.add_task(Task::new("B", 20));
    assert_eq!(idx, 1);
    assert_eq!(day.tasks.len(), 2);
    assert_eq!(day.tasks[1].title, "B");
    assert_eq!(day.tasks[1].state, TaskState::Planned);
}

#[test]
fn add_task_does_not_change_active() {
    let mut day = DayPlan::new(vec![Task::new("A", 30)]);
    day.start(0); // make A active
    assert_eq!(day.active_index(), Some(0));
    let _ = day.add_task(Task::new("B", 20));
    assert_eq!(day.active_index(), Some(0));
    assert_eq!(day.tasks[1].state, TaskState::Planned);
}
