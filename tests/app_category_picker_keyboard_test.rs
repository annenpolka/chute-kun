use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

#[test]
fn right_click_dot_opens_picker_and_enter_applies() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Focus", 20);

    // Render to locate dot
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let area = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let row_y = list.y + 1;
    // Find the actual dot position in the rendered row
    let dot_x = (list.x..list.x + list.width)
        .find(|&x| buf[(x, row_y)].symbol() == "●")
        .expect("did not find category dot");

    // Open picker via right click on the dot
    app.handle_mouse_event(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: dot_x, row: row_y, modifiers: crossterm::event::KeyModifiers::empty() },
        area,
    );
    assert!(app.is_category_picker(), "expected category picker to be open");

    // Down once -> select Work (index 1), Enter to apply
    app.handle_key(KeyCode::Down);
    app.handle_key(KeyCode::Enter);
    assert!(format!("{:?}", app.day.tasks[0].category).contains("Work"));
}
