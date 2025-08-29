use chute_kun::task::{DayPlan, Task, TaskState};
use chute_kun::task::esd_from;

// 単体: ESD計算（now=09:00, 残=30+60+15 => 10:45）
#[test]
fn esd_calc_simple_sum() {
    let now_min = 9 * 60; // 09:00
    let remaining = [30u16, 60u16, 15u16];
    let esd = esd_from(now_min, &remaining);
    assert_eq!(esd, 10 * 60 + 45);
}

// 単体: アクティブ単一性
#[test]
fn only_one_active_task_at_a_time() {
    let mut day = DayPlan::new(vec![
        Task::new("A", 30),
        Task::new("B", 60),
    ]);

    // start A
    day.start(0);
    assert_eq!(day.tasks[0].state, TaskState::Active);
    assert_eq!(day.active_index(), Some(0));

    // then start B -> A becomes paused, B becomes active
    day.start(1);
    assert_eq!(day.tasks[0].state, TaskState::Paused);
    assert_eq!(day.tasks[1].state, TaskState::Active);
    assert_eq!(day.active_index(), Some(1));
}

