use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

#[test]
fn dragging_highlights_source_and_target_rows() {
    let backend = TestBackend::new(60, 16);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);
    app.add_task("D", 40);

    let area = Rect { x: 0, y: 0, width: 60, height: 16 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let col_x = list.x + 2;
    let row_of = |i: u16| list.y + 1 + i; // header at list.y

    // Begin dragging B (index 1)
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: col_x,
            row: row_of(1),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    // Drag over D (index 3) but don't release yet
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: col_x,
            row: row_of(3),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer();

    // Source row (B, index 1) should use one of the drag-source BG colors
    let mut source_ok = false;
    for x in list.x..list.x + list.width.min(buf.area.width) {
        let bg = buf[(x, row_of(1))].style().bg;
        if bg == Some(ui::DRAG_SOURCE_BG_A) || bg == Some(ui::DRAG_SOURCE_BG_B) {
            source_ok = true;
            break;
        }
    }
    assert!(source_ok, "drag source row should be highlighted with DRAG_SOURCE_BG");

    // Target row (D, index 3) should use one of the drag-target BG colors
    let mut target_ok = false;
    for x in list.x..list.x + list.width.min(buf.area.width) {
        let bg = buf[(x, row_of(3))].style().bg;
        if bg == Some(ui::DRAG_TARGET_BG_A) || bg == Some(ui::DRAG_TARGET_BG_B) {
            target_ok = true;
            break;
        }
    }
    assert!(target_ok, "drag target row should be highlighted with DRAG_TARGET_BG");
}
