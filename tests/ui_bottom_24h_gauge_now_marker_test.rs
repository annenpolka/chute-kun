use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

fn cell(buf: &ratatui::buffer::Buffer, x: u16, y: u16) -> &ratatui::buffer::Cell {
    &buf[(x, y)]
}

#[test]
fn bottom_gauge_shows_now_marker() {
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    let app = App::new();
    let now = 12 * 60 + 34; // 12:34
    let clock = FixedClock(now);

    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    let gauge_y = help.y;

    // Map minutes to X like the gauge does
    let map_x = |m: u16| -> u16 {
        let w = help.width as u32;
        let x = (m as u32) * w / 1440u32;
        (help.x as u32 + x).min((help.x + help.width - 1) as u32) as u16
    };
    let x_now = map_x(now);

    assert_eq!(cell(&buf, x_now, gauge_y).symbol(), "^");
    assert_eq!(cell(&buf, x_now, gauge_y).style().fg, Some(Color::Red));
}
