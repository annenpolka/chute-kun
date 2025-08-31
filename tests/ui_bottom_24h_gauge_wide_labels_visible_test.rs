use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

fn cell(buf: &ratatui::buffer::Buffer, x: u16, y: u16) -> &ratatui::buffer::Cell {
    &buf[(x, y)]
}

#[test]
fn labels_visible_on_wide_terminal() {
    let backend = TestBackend::new(240, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = ratatui::layout::Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    // labels live on the line just below gauge
    let label_y = help.y + 1;
    // Map minute -> x like UI does
    let map_x = |m: u16| -> u16 {
        let w = help.width as u32;
        let x = (m as u32) * w / 1440u32;
        (help.x as u32 + x).min((help.x + help.width - 1) as u32) as u16
    };
    for (m, s0, s1) in [(6u16 * 60, '6', ' '), (12u16 * 60, '1', '2'), (18u16 * 60, '1', '8')] {
        let x = map_x(m);
        assert_eq!(cell(&buf, x, label_y).symbol(), s0.to_string());
        if s1 != ' ' {
            assert_eq!(cell(&buf, x + 1, label_y).symbol(), s1.to_string());
        }
    }
}
