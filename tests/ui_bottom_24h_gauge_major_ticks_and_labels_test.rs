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
fn bottom_gauge_has_thick_ticks_and_labels() {
    let backend = TestBackend::new(80, 22);
    let mut terminal = Terminal::new(backend).unwrap();

    let app = App::new();
    // Choose a time that doesn't collide with 6/12/18 tick columns
    let clock = FixedClock(13 * 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    assert!(help.height >= 2, "help area must have at least two lines");
    let gauge_y = help.y + help.height - 1;
    let label_y = gauge_y - 1;

    let map_x = |m: u16| -> u16 {
        let w = help.width as u32;
        let x = (m as u32) * w / 1440u32;
        (help.x as u32 + x).min((help.x + help.width - 1) as u32) as u16
    };

    for (h, expect) in [(6u16, '│'), (12u16, '│'), (18u16, '│')] {
        let x = map_x(h * 60);
        assert_eq!(cell(&buf, x, gauge_y).symbol(), expect.to_string());
        assert_eq!(cell(&buf, x, gauge_y).style().fg, Some(Color::DarkGray));
    }

    // Labels on the line above: 6, 12, 18
    let x6 = map_x(6 * 60);
    assert_eq!(cell(&buf, x6, label_y).symbol(), "6");
    let x12 = map_x(12 * 60);
    assert_eq!(cell(&buf, x12, label_y).symbol(), "1");
    assert_eq!(cell(&buf, x12 + 1, label_y).symbol(), "2");
    let x18 = map_x(18 * 60);
    assert_eq!(cell(&buf, x18, label_y).symbol(), "1");
    assert_eq!(cell(&buf, x18 + 1, label_y).symbol(), "8");
}
