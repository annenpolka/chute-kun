use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

#[test]
fn dragging_over_tail_space_highlights_last_row_as_target() {
    let mut app = App::new();
    // Three tasks to make behavior obvious
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);

    let area = Rect { x: 0, y: 0, width: 60, height: 14 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Start dragging from the first row (index 0)
    let start_col = list.x + 2;
    let start_row = list.y + 1; // first data row
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: start_col,
            row: start_row,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    // Drag into the tail space (still inside list rect but below the last data row)
    let len = 3u16;
    let last_row_y = list.y + 1 + (len - 1);
    let tail_y = (last_row_y + 3).min(list.y + list.height - 1);
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: start_col,
            row: tail_y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    // Hover should snap to the last row while dragging
    assert_eq!(app.hovered_index(), Some(2));

    // And the last row should render as a drag target (greenish BG)
    let backend = TestBackend::new(area.width, area.height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer();
    let target_y = last_row_y; // last real row
    let mut target_bg_seen = false;
    for x in list.x..list.x + list.width.min(buf.area.width) {
        let bg = buf[(x, target_y)].style().bg;
        if bg == Some(ui::DRAG_TARGET_BG_A) || bg == Some(ui::DRAG_TARGET_BG_B) {
            target_bg_seen = true;
            break;
        }
    }
    assert!(target_bg_seen, "last row should render as drag target when dragging over tail space");
}
