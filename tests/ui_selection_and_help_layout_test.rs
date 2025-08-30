use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, Terminal};

fn line_at(buf: &ratatui::buffer::Buffer, y: u16) -> String {
    let mut s = String::new();
    for x in 0..buf.area.width {
        s.push_str(buf[(x, y)].symbol());
    }
    s
}

#[test]
fn help_should_render_at_bottom_when_active_banner_present() {
    // Height large enough to host: border, tabs, banner, list line(s), and one-line help
    // Wide enough so help fits a single line
    let backend = TestBackend::new(120, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    app.add_task("Focus Work", 30);
    app.add_task("Email", 10);

    // Start the first task to trigger the active banner line under tabs
    app.handle_key(KeyCode::Enter);

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Outer border occupies y=0 and y=height-1; inner area starts at y=1
    // Expected layout with banner:
    // y=1: tabs, y=2: active banner, y>=3: list content, bottom inner line: help
    let banner_line = line_at(&buf, 2);
    assert!(banner_line.contains("Now:"), "expected active banner on y=2, got: {}", banner_line);

    // The first content rows should be a table header then task rows, not help
    let header_row = line_at(&buf, 3);
    assert!(
        header_row.contains("Plan") && header_row.contains("Actual"),
        "expected table header at y=3, got: {}",
        header_row
    );
    let list_first = line_at(&buf, 4);
    assert!(
        list_first.contains("Focus Work") || list_first.contains("Email"),
        "expected task content at y=4, got: {}",
        list_first
    );

    // Sanity: ensure no help line leaked into the list area.
    assert!(!list_first.contains("q: quit"), "help text should not overlap the list area");
}
