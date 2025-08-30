use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, Terminal};

fn line_at(buf: &ratatui::buffer::Buffer, y: u16) -> String {
    let mut s = String::new();
    // Read inner line across the full width (including borders is fine for substring match)
    for x in 0..buf.area.width {
        s.push_str(buf[(x, y)].symbol());
    }
    s
}

#[test]
fn shows_active_task_banner_on_top_when_running() {
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Focus Work", 30);
    // Start the first task (becomes Active)
    app.handle_key(KeyCode::Enter);

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    // y=0 is the top border line of the outer block, y=1 is tabs, y=2 should be the banner
    let banner = line_at(&buf, 2);
    assert!(
        banner.contains("Now:"),
        "expected 'Now:' banner on the line just under tabs, got: {}",
        banner
    );
    assert!(
        banner.contains("Focus Work"),
        "expected the running task title to appear, got: {}",
        banner
    );
}
