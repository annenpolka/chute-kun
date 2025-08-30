use chute_kun::{app::App, ui};

// Done(完了)したタスクも見積時間で次以降のPlanを押し出す
#[test]
fn done_task_estimate_shifts_next_task_schedule() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);

    // 選択はデフォルトで先頭(0)。選択タスクの完了操作で A を Done にする。
    app.finish_selected();

    // 予定の基準時刻 09:00 から、A(30m) が完了済みでも見積 30m を加味して
    // B の開始は 09:30 になること。
    let lines = ui::format_task_lines_at(9 * 60, &app);
    assert!(
        lines.get(1).is_some_and(|s| s.starts_with("09:30 ")),
        "expected second line to start with 09:30, got: {:?}",
        lines.get(1)
    );
}
