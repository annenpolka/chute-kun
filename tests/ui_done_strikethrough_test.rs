use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, layout::Rect, style::Modifier, Terminal};

#[test]
fn done_task_row_shows_strikethrough_on_title() {
    // Arrange: three tasks; make the third one Done
    let backend = TestBackend::new(80, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 25);
    app.add_task("B", 15);
    app.add_task("C", 10);

    // Move selection to C and finish it
    app.handle_key(KeyCode::Char('j'));
    app.handle_key(KeyCode::Char('j'));
    app.handle_key_event(crossterm::event::KeyEvent::new(
        KeyCode::Char('f'),
        crossterm::event::KeyModifiers::NONE,
    ));

    // Act: render
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Compute list rect and locate C's row (3rd data row)
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);
    let row_c_y = list.y + 3; // header at list.y; rows start at +1

    // Assert: some cells in the row should have the strikethrough modifier
    let mut found = false;
    for x in list.x..list.x + list.width {
        if buf[(x, row_c_y)].style().add_modifier.contains(Modifier::CROSSED_OUT) {
            found = true;
            break;
        }
    }
    assert!(found, "expected strikethrough on the Done task row");
}
