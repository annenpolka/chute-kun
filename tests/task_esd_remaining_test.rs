use chute_kun::task::{DayPlan, Task};

#[test]
fn esd_uses_remaining_estimate_including_active_progress() {
    // now = 09:00, tasks: A(30 with 10 done), B(60), C(15)
    let mut day = DayPlan::new(vec![Task::new("A", 30), Task::new("B", 60), Task::new("C", 15)]);
    day.start(0);
    day.add_actual_to_active(10);

    let now = 9 * 60; // 540
    let esd = day.esd(now);
    // remaining = (30-10) + 60 + 15 = 95 => 540+95 = 635 = 10:35
    assert_eq!(esd, 10 * 60 + 35);
}

