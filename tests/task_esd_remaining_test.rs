use chute_kun::task::{DayPlan, Task};

#[test]
fn esd_ignores_actual_progress_and_uses_full_estimates() {
    // now = 09:00, tasks: A(30 with 10 done), B(60), C(15)
    let mut day = DayPlan::new(vec![Task::new("A", 30), Task::new("B", 60), Task::new("C", 15)]);
    day.start(0);
    day.add_actual_to_active(10);

    let now = 9 * 60; // 540
    let esd = day.esd(now);
    // ESD should not be shortened by actual progress.
    // sum(est) = 30+60+15 = 105 => 540+105 = 645 = 10:45
    assert_eq!(esd, 10 * 60 + 45);
}
