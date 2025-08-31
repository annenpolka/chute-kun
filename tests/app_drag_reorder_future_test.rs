use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

// Red: Futureビューでのドラッグ並び替え
#[test]
fn future_drag_reorders_tomorrow_tasks() {
    let mut app = App::new();
    // Todayに4件作成
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);
    app.add_task("D", 40);
    // A, B, C を Future に送る（明日リスト作成）
    app.postpone_selected(); // A
    app.postpone_selected(); // B (A削除後に先頭がB)
    app.postpone_selected(); // C
    assert_eq!(
        app.tomorrow_tasks().iter().map(|t| t.title.as_str()).collect::<Vec<_>>(),
        vec!["A", "B", "C"]
    );

    // Futureビューへ
    app.handle_key(crossterm::event::KeyCode::Tab); // Today -> Future
    let area = Rect { x: 0, y: 0, width: 60, height: 16 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let col_x = list.x + 2;
    let row_of = |i: u16| list.y + 1 + i; // header at list.y

    // B(index1)をC(index2)の後ろへドロップ（最下段へ）
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: col_x,
            row: row_of(1),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: col_x,
            row: row_of(2),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: col_x,
            row: row_of(2),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    let titles: Vec<_> = app.tomorrow_tasks().iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["A", "C", "B"], "Bが末尾に移動する");
    assert_eq!(app.selected_index(), 2, "選択は移動先に追随");
}
