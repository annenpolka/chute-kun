use chute_kun::{app::App, task::Category, task::Session, ui};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

// Fixed clock for deterministic rendering
struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

fn cell_at(buf: &ratatui::buffer::Buffer, x: u16, y: u16) -> &ratatui::buffer::Cell {
    &buf[(x, y)]
}

#[test]
fn bottom_gauge_shows_category_segments_over_24h() {
    // Wide enough to get a multi-line help area
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    // Add two tasks with actual sessions in different categories and far apart times
    let s1 = 9 * 60; // 09:00
    let e1 = 9 * 60 + 30; // 09:30
    let s2 = 21 * 60; // 21:00
    let e2 = 21 * 60 + 15; // 21:15
    app.add_task("Morning Work", 60);
    app.add_task("Late Home", 30);
    app.day.tasks[0].category = Category::Work; // Blue
    app.day.tasks[1].category = Category::Home; // Yellow
    if let Some(t) = app.day.tasks.get_mut(0) {
        t.sessions.push(Session { start_min: s1, end_min: Some(e1) });
    }
    if let Some(t) = app.day.tasks.get_mut(1) {
        t.sessions.push(Session { start_min: s2, end_min: Some(e2) });
    }

    // Draw with a fixed clock late at night so no active-now overlay interferes
    let clock = FixedClock(23 * 60 + 50);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();

    let buf = terminal.backend().buffer().clone();
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, _list, help) = ui::compute_layout(&app, full);
    assert!(help.height >= 1, "help area should exist to host the gauge");
    let gauge_y = help.y; // first line of help hosts the gauge

    // Map minute -> X coordinate in the gauge line, mirroring ui logic (24h across width)
    let map_x = |m: u16| -> u16 {
        let w = help.width as u32;
        let x = (m as u32) * w / 1440u32;
        (help.x as u32 + x).min((help.x + help.width - 1) as u32) as u16
    };

    let x_work = map_x(s1 + 5); // inside first session
    let x_home = map_x(s2 + 5); // inside second session
    assert_eq!(cell_at(&buf, x_work, gauge_y).style().fg, Some(Color::Blue));
    assert_eq!(cell_at(&buf, x_home, gauge_y).style().fg, Some(Color::Yellow));
}
