use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

// Red: clicking on a list row should select that task.
#[test]
fn mouse_click_selects_list_row() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);

    // Simulate a 40x10 terminal. Compute list area using UI layout helper.
    let area = Rect { x: 0, y: 0, width: 40, height: 10 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Click roughly on the second row inside the list area.
    let row_y = list.y + 2; // index 1 => second task (header at list.y)
    let col_x = list.x + 2; // a safe in-bounds x
    let ev = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: col_x,
        row: row_y,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    // Act
    app.handle_mouse_event(ev, area);

    // Assert
    assert_eq!(app.selected_index(), 1, "clicking second row should select index 1");
}
