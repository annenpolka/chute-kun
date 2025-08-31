use chute_kun::{app::App, task::{Category, Session}, ui};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

#[test]
fn columns_use_each_task_category_color() {
    let backend = TestBackend::new(80, 18);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    let base = app.config.day_start_minutes;

    app.add_task("Alpha", 30);
    app.add_task("Bravo", 30);
    app.day.tasks[0].category = Category::Work;  // Blue
    app.day.tasks[1].category = Category::Home;  // Yellow
    // Overlap
    if let Some(t) = app.day.tasks.get_mut(0) {
        t.sessions.push(Session { start_min: base + 10, end_min: Some(base + 25) });
    }
    if let Some(t) = app.day.tasks.get_mut(1) {
        t.sessions.push(Session { start_min: base + 10, end_min: Some(base + 20) });
    }
    app.toggle_display_mode();

    // Draw later so titles overlay on the start row
    struct Cl(u16); impl chute_kun::clock::Clock for Cl { fn now_minutes(&self)->u16{self.0} }
    let clock = Cl(base + 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Locate act lane bounds
    let area = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let gutter = 6u16; let lanes_w = list.width.saturating_sub(gutter + 2);
    let lane_w = (lanes_w.saturating_sub(1)) / 2; let plan_x0 = list.x + gutter + 1; let act_x0 = plan_x0 + lane_w + 1;

    // Find the start row (contains both titles)
    let mut y_row = None;
    for y in list.y..list.y + list.height {
        let mut s = String::new(); for x in 0..buf.area.width { s.push_str(buf[(x, y)].symbol()); }
        if s.contains('A') && s.contains('B') { y_row = Some(y); break; }
    }
    let y = y_row.expect("no row with both titles");

    // Inspect colors at each title cell
    let xa = (act_x0..act_x0 + lane_w).find(|&x| buf[(x, y)].symbol() == "A").unwrap();
    assert_eq!(buf[(xa, y)].style().fg, Some(Color::Blue));
    let xb = (act_x0..act_x0 + lane_w).find(|&x| buf[(x, y)].symbol() == "B").unwrap();
    assert_eq!(buf[(xb, y)].style().fg, Some(Color::Yellow));
}

