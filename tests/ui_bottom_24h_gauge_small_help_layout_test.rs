use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

fn row(buf: &ratatui::buffer::Buffer, y: u16) -> String {
    (0..buf.area.width).map(|x| buf[(x, y)].symbol()).collect::<String>()
}

#[test]
fn when_help_height_is_two_render_gauge_then_help() {
    // Height 8 → inner height ~6 → reserved 4 → help height = 2
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = ratatui::layout::Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    assert_eq!(help.height, 2, "expected help height 2 for this viewport");
    let top = row(&buf, help.y);
    assert!(top.contains('·') || top.contains('^') || top.contains('│') || top.contains('|'));
    let bottom = row(&buf, help.y + 1);
    assert!(bottom.contains("quit"), "expected help on second line: {}", bottom);
}

#[test]
fn when_help_height_is_one_render_help_only() {
    // Height 7 → inner ~5 → reserved 4 → help height = 1
    let backend = TestBackend::new(80, 7);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = ratatui::layout::Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    assert_eq!(help.height, 1, "expected help height 1 for this viewport");
    let line = row(&buf, help.y);
    assert!(line.contains("quit"), "expected help text when only one line: {}", line);
    // Ensure no gauge glyphs: disallow gauge dot and caret; thin '|' may appear as separators in help
    assert!(!line.contains('·') && !line.contains('^'));
}
