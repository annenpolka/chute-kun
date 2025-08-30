use chute_kun::task::{DayPlan, Session, Task, TaskState};

#[test]
fn esd_uses_latest_finished_time_plus_remaining_estimates() {
    // now = 09:00, estimates = A(30) + B(60) but B is already finished at 10:45
    // remaining non-done estimates = A(30)
    // ESD = max(now, 10:45) + 30 = 11:15
    let mut day = DayPlan::new(vec![Task::new("A", 30), Task::new("B", 60)]);
    day.tasks[1].state = TaskState::Done;
    day.tasks[1].finished_at_min = Some(10 * 60 + 45);

    let esd = day.esd(9 * 60);
    assert_eq!(esd, 11 * 60 + 15);
}

#[test]
fn esd_uses_latest_session_end_plus_remaining_estimates() {
    // now=09:00; A(30), B(20); latest session end = 10:10; remaining = 30+20=50
    // ESD = 10:10 + 50 = 11:00
    let mut day = DayPlan::new(vec![Task::new("A", 30), Task::new("B", 20)]);
    day.tasks[0].sessions.push(Session { start_min: 9 * 60, end_min: Some(10 * 60 + 10) });

    let esd = day.esd(9 * 60);
    assert_eq!(esd, 11 * 60);
}
