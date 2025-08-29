use chute_kun::task::{DayPlan, Task, TaskState};

#[test]
fn accumulates_actual_minutes_across_sessions() {
    let mut day = DayPlan::new(vec![Task::new("A", 30)]);
    day.start(0);
    day.add_actual_to_active(10);
    day.pause_active();
    assert_eq!(day.tasks[0].actual_min, 10);

    day.start(0);
    day.add_actual_to_active(15);
    assert_eq!(day.tasks[0].actual_min, 25);
    assert_eq!(day.tasks[0].state, TaskState::Active);
}

#[test]
fn add_actual_without_active_is_noop() {
    let mut day = DayPlan::new(vec![Task::new("A", 30)]);
    day.add_actual_to_active(5);
    assert_eq!(day.tasks[0].actual_min, 0);
}
