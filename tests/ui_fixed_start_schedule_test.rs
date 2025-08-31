use chute_kun::{app::App, ui};

// 固定開始時刻（per-task）を反映してスケジュールを組む
// - 指定タスク自身は固定時刻から始まる（基準09:00より後ろにできる）
// - それ以降のタスクはその時刻以降から見積で押し出される
#[test]
fn fixed_start_time_pushes_following_tasks() {
    let mut app = App::new();
    app.add_task("A", 30); // 09:00-09:30 想定
    app.add_task("B", 20); // 09:30-09:50 想定だが固定で後ろへ
    app.add_task("C", 10);

    // B を選択し、10:00 固定にする → B は 10:00, C は 10:20 になる
    // Command パレット経由で設定（"at HH:MM"）
    use crossterm::event::KeyCode;
    app.handle_key(KeyCode::Down); // select B
    app.handle_key(KeyCode::Char(':'));
    for c in "at 10:00".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    let lines = ui::format_task_lines_at(9 * 60, &app);
    assert!(lines[0].starts_with("09:00 ")); // A
    assert!(lines[1].starts_with("10:00 ")); // B 固定
    assert!(lines[2].starts_with("10:20 ")); // C は B 以降で押し出し
}

// 固定を解除できる（at -）
#[test]
fn clear_fixed_start_time_via_command() {
    let mut app = App::new();
    app.add_task("Task", 30);

    use crossterm::event::KeyCode;
    app.handle_key(KeyCode::Char(':'));
    for c in "at 09:30".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    // 解除
    app.handle_key(KeyCode::Char(':'));
    for c in "at -".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    // 固定解除後は通常通り 09:00 始まり
    let lines = ui::format_task_lines_at(9 * 60, &app);
    assert!(lines[0].starts_with("09:00 "));
}
