use chute_kun::task::{tc_log_line, Task, TaskState};

#[test]
fn tc_log_line_has_minimal_fields() {
    let mut t = Task::new("Write report", 30);
    t.actual_min = 25;
    t.state = TaskState::Done;
    let s = tc_log_line(&t);
    assert!(s.contains("tc-log"));
    assert!(s.contains("Write report"));
    assert!(s.contains("act:25m"));
    assert!(s.contains("est:30m"));
    assert!(s.contains("state:Done"));
}
