use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

fn row_string(buf: &ratatui::buffer::Buffer, y: u16) -> String {
    (0..buf.area.width).map(|x| buf[(x, y)].symbol()).collect::<String>()
}

#[test]
fn help_text_renders_below_gauge() {
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let full = ratatui::layout::Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);

    // With the new layout, the gauge must occupy the first line of help
    let top = row_string(&buf, help.y);
    assert!(top.contains('·') || top.contains('^') || top.contains('│') || top.contains('|'));

    // And the bottom-most help line should contain help text like "quit"
    let bottom = row_string(&buf, help.y + help.height - 1);
    assert!(bottom.contains("quit"), "expected help text at the bottom: {}", bottom);
}
