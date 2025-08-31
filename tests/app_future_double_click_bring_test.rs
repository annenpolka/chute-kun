use chute_kun::task::TaskState;
use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

// Red: Futureビューでのダブルクリックは誤ってTodayの別タスクを開始しない。
// ここでは「bringするが開始はしない」挙動に見直す。
#[test]
fn future_double_click_brings_task_without_starting_any() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // AをFutureへ
    app.postpone_selected();
    assert_eq!(app.tomorrow_tasks().len(), 1);

    // Futureビューへ
    app.handle_key(crossterm::event::KeyCode::Tab);
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let col_x = list.x + 2;
    let row_y = list.y + 1; // first data row

    // ダブルクリック（Downを短時間で2回送る）
    let down = |row| MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: col_x,
        row,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    app.handle_mouse_event(down(row_y), area);
    app.handle_mouse_event(down(row_y), area);

    // A が Today に戻り末尾に追加、かつどのタスクも Active になっていない
    assert_eq!(app.tomorrow_tasks().len(), 0, "Future は空になる");
    assert_eq!(app.day.tasks.len(), 2);
    assert_eq!(app.day.tasks[0].title, "B");
    assert_eq!(app.day.tasks[1].title, "A");
    assert!(app.day.active_index().is_none(), "どのタスクも開始されない");
    assert_eq!(app.day.tasks[0].state, TaskState::Planned);
    assert_eq!(app.day.tasks[1].state, TaskState::Planned);
}
