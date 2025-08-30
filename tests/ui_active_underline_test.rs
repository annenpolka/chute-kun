use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, layout::Rect, style::Modifier, Terminal};

#[test]
fn active_row_title_is_not_underlined() {
    // Arrange: terminal + two tasks; start the first (Active)
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Focus Work", 30);
    app.add_task("Email", 10);
    app.handle_key(KeyCode::Enter); // start index 0

    // Compute list rect to locate the first data row (index 0)
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Act: draw
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer();

    // Assert: on the active row (list.y + 1), no cell is underlined
    let y = list.y + 1; // header at list.y; first row at list.y + 1
    let mut underlined_found = false;
    for x in list.x..(list.x + list.width).min(buf.area.width) {
        if buf[(x, y)].style().add_modifier.contains(Modifier::UNDERLINED) {
            underlined_found = true;
            break;
        }
    }
    assert!(!underlined_found, "active row should not be underlined");
}

#[test]
fn active_banner_text_is_fully_underlined() {
    // Arrange
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Focus Work", 30);
    app.handle_key(KeyCode::Enter); // make active

    // Act
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer();

    // Compute banner rect and banner text width
    let area = Rect { x: 0, y: 0, width: 80, height: 8 };
    let (_tabs, banner_opt, _list, _help) = ui::compute_layout(&app, area);
    let banner = banner_opt.expect("banner rect should exist when a task is active");
    let text_w = ui::format_active_banner(&app).unwrap().width() as u16;

    // y=2 should be banner line; verify that all cells covering the banner text are underlined
    let y = banner.y;
    for x in banner.x..(banner.x + text_w).min(buf.area.width) {
        assert!(
            buf[(x, y)].style().add_modifier.contains(Modifier::UNDERLINED),
            "expected underline at x={} on banner",
            x
        );
    }
}
