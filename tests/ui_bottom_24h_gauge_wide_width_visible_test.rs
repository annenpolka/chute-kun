use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

fn row(buf: &ratatui::buffer::Buffer, y: u16) -> String {
    (0..buf.area.width).map(|x| buf[(x, y)].symbol()).collect::<String>()
}

#[test]
fn gauge_visible_on_very_wide_terminal() {
    // very wide width to simulate wrapping reduction
    let backend = TestBackend::new(240, 20);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = ratatui::layout::Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    assert!(help.height >= 2, "help height should be at least 2 on wide view");
    let gauge_line = row(&buf, help.y);
    // gauge line must contain at least baseline/tick/now marker characters
    assert!(
        gauge_line.contains('·')
            || gauge_line.contains('│')
            || gauge_line.contains('|')
            || gauge_line.contains('^'),
        "expected gauge characters on top help line: {}",
        gauge_line
    );
}
