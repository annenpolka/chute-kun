use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn double_click_starts_then_stops_task() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    let area = Rect { x: 0, y: 0, width: 60, height: 10 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Target row 1 (title B). Header is at list.y; first data row at list.y+1
    let row_y = list.y + 2;
    let col_x = list.x + 2;

    // Two quick left clicks -> start task at index 1
    let ev = |kind| MouseEvent {
        kind,
        column: col_x,
        row: row_y,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    app.handle_mouse_event(ev(MouseEventKind::Down(MouseButton::Left)), area);
    app.handle_mouse_event(ev(MouseEventKind::Down(MouseButton::Left)), area);
    assert_eq!(app.day.active_index(), Some(1));

    // Layout shifts because an active banner appears; recompute list and row_y
    let (_t2, _b2, list2, _h2) = ui::compute_layout(&app, area);
    let row_y2 = list2.y + 2; // same index (1)
    let ev2 = |kind| MouseEvent {
        kind,
        column: col_x,
        row: row_y2,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    // Another double click on same task -> pause
    app.handle_mouse_event(ev2(MouseEventKind::Down(MouseButton::Left)), area);
    app.handle_mouse_event(ev2(MouseEventKind::Down(MouseButton::Left)), area);
    assert_eq!(app.day.active_index(), None);
}
