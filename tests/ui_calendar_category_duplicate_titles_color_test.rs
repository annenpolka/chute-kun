use chute_kun::{app::App, task::Category, ui};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

fn find_all_rows_with(buf: &ratatui::buffer::Buffer, needle: &str) -> Vec<u16> {
    let mut rows = Vec::new();
    for y in 0..buf.area.height {
        let mut s = String::new();
        for x in 0..buf.area.width { s.push_str(buf[(x, y)].symbol()); }
        if s.contains(needle) { rows.push(y); }
    }
    rows
}

#[test]
fn calendar_plan_lane_uses_per_range_category_even_with_duplicate_titles() {
    let backend = TestBackend::new(80, 18);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes;
    // Two tasks with the same title but different categories
    app.add_task("Same", 40);
    app.add_task("Same", 30);
    app.day.tasks[0].category = Category::Work; // Blue
    app.day.tasks[1].category = Category::Home; // Yellow

    // Switch to Calendar and draw later so Now row does not interfere
    app.toggle_display_mode();
    struct Fixed(u16);
    impl chute_kun::clock::Clock for Fixed { fn now_minutes(&self) -> u16 { self.0 } }
    let clock = Fixed(base + 200);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Compute plan lane start X
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);
    let gutter = 6u16; let gaps = 2u16;
    let lanes_w = list.width.saturating_sub(gutter + gaps);
    let lane_w = (lanes_w.saturating_sub(1)) / 2;
    let plan_x0 = list.x + gutter + 1;

    // Expect two rows containing the title 'Same'
    let mut ys = find_all_rows_with(&buf, "Same");
    ys.sort();
    assert!(ys.len() >= 2, "expected at least two rows with title");
    let y_top = ys[0];
    let y_bottom = ys[1];

    // Color at the first character of each title in the plan lane
    let x_top = (plan_x0..plan_x0 + lane_w).find(|&x| buf[(x, y_top)].symbol() == "S").unwrap();
    assert_eq!(buf[(x_top, y_top)].style().fg, Some(Color::Blue));

    let x_bot = (plan_x0..plan_x0 + lane_w).find(|&x| buf[(x, y_bottom)].symbol() == "S").unwrap();
    assert_eq!(buf[(x_bot, y_bottom)].style().fg, Some(Color::Yellow));
}

