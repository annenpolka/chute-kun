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
fn bottom_gauge_has_hour_ticks() {
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    let app = App::new();
    // Avoid placing 'now' on a tick position to prevent overlap
    let clock = FixedClock(12 * 60 + 30);

    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    let gauge_y = help.y;

    let map_x = |m: u16| -> u16 {
        let w = help.width as u32;
        let x = (m as u32) * w / 1440u32;
        (help.x as u32 + x).min((help.x + help.width - 1) as u32) as u16
    };

    // Expect thin tick at 00:00 and thick ticks at 06:00, 12:00, 18:00
    let x0 = map_x(0);
    assert_eq!(cell(&buf, x0, gauge_y).symbol(), "|");
    assert_eq!(cell(&buf, x0, gauge_y).style().fg, Some(Color::DarkGray));
    for h in [6u16, 12, 18] {
        let x = map_x(h * 60);
        assert_eq!(cell(&buf, x, gauge_y).symbol(), "│");
        assert_eq!(cell(&buf, x, gauge_y).style().fg, Some(Color::DarkGray));
    }
}
