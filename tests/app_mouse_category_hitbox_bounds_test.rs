use chute_kun::{app::App, ui};
use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

// Regression: clicking below the last data row at the dot X should NOT toggle category
#[test]
fn bottom_area_click_does_not_toggle_category() {
    let backend = TestBackend::new(60, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("One", 10);

    // Render to get geometry and dot X
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let area = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Find dot X on the actual row
    let row_y = list.y + 1; // first data row
    // Derive actual dot X by reading the buffer instead of assuming widths
    let dot_x = (list.x..list.x + list.width)
        .find(|&x| buf[(x, row_y)].symbol() == "●")
        .expect("dot not found in row");

    // Click far below the data row but within the list rect height
    let click_y = list.y + list.height - 1; // bottom line of list area
    assert!(click_y > row_y);

    let ev = MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: dot_x, row: click_y, modifiers: KeyModifiers::empty() };
    app.handle_mouse_event(ev, area);

    // Category should remain General (not toggled to Work)
    assert_eq!(format!("{:?}", app.day.tasks[0].category), "General");
}
