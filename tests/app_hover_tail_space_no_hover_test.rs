use chute_kun::{app::App, ui};
use ratatui::layout::Rect;

// Hover should only activate on actual task rows.
// Moving the mouse into the empty tail space below the last task must clear hover.
#[test]
fn hover_does_not_activate_in_tail_space() {
    let mut app = App::new();
    // Two tasks; default selection at index 0
    app.add_task("A", 10);
    app.add_task("B", 20);

    // Use a reasonably tall area to ensure list has trailing empty space
    let area = Rect { x: 0, y: 0, width: 80, height: 20 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Compute Y just below the last data row but still inside the list rect
    // Header at list.y; first data row at list.y + 1; last data row at list.y + len
    let len = 2u16;
    let last_row_y = list.y + 1 + (len - 1);
    // Choose a row further down within the list area (if possible)
    let tail_y = (last_row_y + 2).min(list.y + list.height - 1);
    assert!(tail_y > last_row_y, "precondition: need a tail row inside list area");

    let col_x = list.x + 2;
    // Move over the last real row first → hover index 1
    app.handle_mouse_move(col_x, last_row_y, area);
    assert_eq!(app.hovered_index(), Some(1));

    // Move into the tail space below → hover should clear to None
    app.handle_mouse_move(col_x, tail_y, area);
    assert_eq!(app.hovered_index(), None);
}
