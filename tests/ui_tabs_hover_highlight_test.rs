use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

#[test]
fn hovering_tab_shows_cyan_foreground() {
    let backend = TestBackend::new(60, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    let area = Rect { x: 0, y: 0, width: 60, height: 10 };
    let (tabs, _b, _l, _h) = ui::compute_layout(&app, area);
    let boxes = ui::tab_hitboxes(&app, tabs);
    // Hover the third tab (Future)
    let r = boxes[2];
    app.handle_mouse_move(r.x, r.y, area);
    terminal.draw(|f| ui::draw(f, &app)).unwrap();

    let buf = terminal.backend().buffer();
    // Expect the first cell of the hovered tab to be Cyan fg (unless it's also selected)
    let c = &buf[(r.x, r.y)];
    // Selected is Today (middle), hovered is Future → Cyan
    assert_eq!(c.style().fg, Some(Color::Cyan));
}
