use chute_kun::{app::App, task::Category, ui};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

fn buffer_to_strings(buf: &ratatui::buffer::Buffer) -> Vec<String> {
    let mut out = Vec::new();
    for y in 0..buf.area.height {
        let mut s = String::new();
        for x in 0..buf.area.width {
            s.push_str(buf[(x, y)].symbol());
        }
        out.push(s);
    }
    out
}

fn find_row_y_containing(buf: &ratatui::buffer::Buffer, needle: &str) -> Option<u16> {
    for y in 0..buf.area.height {
        let mut s = String::new();
        for x in 0..buf.area.width {
            s.push_str(buf[(x, y)].symbol());
        }
        if s.contains(needle) {
            return Some(y);
        }
    }
    None
}

fn find_char_x_in_row(buf: &ratatui::buffer::Buffer, y: u16, ch: char) -> Option<u16> {
    for x in 0..buf.area.width {
        if buf[(x, y)].symbol() == ch.to_string() {
            return Some(x);
        }
    }
    None
}

fn cell_at<'a>(buf: &'a ratatui::buffer::Buffer, x: u16, y: u16) -> &'a ratatui::buffer::Cell {
    &buf[(x, y)]
}

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

#[test]
fn calendar_plan_lane_uses_category_colors() {
    let backend = TestBackend::new(80, 18);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes;
    app.add_task("Work Block", 60);
    app.add_task("Home Block", 30);
    app.day.tasks[0].category = Category::Work; // Blue
    app.day.tasks[1].category = Category::Home; // Yellow

    // Switch to Calendar
    app.toggle_display_mode();

    // Draw at a fixed time well after both planned blocks to avoid the Now row interfering
    let clock = FixedClock(base + 180);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Derive content area and lane start positions mirroring UI layout
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);
    let gutter = 6u16; // "HH:00"
    let gaps = 2u16; // one space after gutter + one between lanes
    let lanes_w = list.width.saturating_sub(gutter + gaps);
    let lane_w = (lanes_w.saturating_sub(1)) / 2;
    let plan_x0 = list.x + gutter + 1; // after one space gap

    // Find the row containing each planned title and assert the color at its first letter
    let y_work = find_row_y_containing(&buf, "Work Block").expect("work row not found");
    let x_work = find_char_x_in_row(&buf, y_work, 'W').expect("W not found");
    assert!(x_work >= plan_x0 && x_work < plan_x0 + lane_w, "Work title should be in plan lane");
    assert_eq!(cell_at(&buf, x_work, y_work).style().fg, Some(Color::Blue));

    let y_home = find_row_y_containing(&buf, "Home Block").expect("home row not found");
    let x_home = find_char_x_in_row(&buf, y_home, 'H').expect("H not found");
    assert!(x_home >= plan_x0 && x_home < plan_x0 + lane_w, "Home title should be in plan lane");
    assert_eq!(cell_at(&buf, x_home, y_home).style().fg, Some(Color::Yellow));
}

#[test]
fn calendar_actual_lane_uses_category_colors() {
    let backend = TestBackend::new(80, 18);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes;
    app.add_task("Alpha", 60);
    app.add_task("Bravo", 60);
    app.day.tasks[1].category = Category::Hobby; // Magenta

    // Add one closed actual session for the second task
    if let Some(t) = app.day.tasks.get_mut(1) {
        t.sessions
            .push(chute_kun::task::Session { start_min: base + 10, end_min: Some(base + 40) });
    }

    app.toggle_display_mode();
    let clock = FixedClock(base + 120);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Compute act lane bounds
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);
    let gutter = 6u16;
    let gaps = 2u16;
    let lanes_w = list.width.saturating_sub(gutter + gaps);
    let lane_w = (lanes_w.saturating_sub(1)) / 2;
    let plan_x0 = list.x + gutter + 1;
    let act_x0 = plan_x0 + lane_w + 1; // +1 for inter-lane gap

    // Find any row in the act lane that contains a session block ('▓') and check color at act lane start
    let mut found_row: Option<u16> = None;
    for y in list.y..list.y + list.height {
        let row_str: String = (act_x0..act_x0 + lane_w)
            .map(|x| buf[(x, y)].symbol().chars().next().unwrap())
            .collect();
        if row_str.contains('▓') {
            found_row = Some(y);
            break;
        }
    }
    let y = found_row.expect("no actual session block row found");
    assert_eq!(cell_at(&buf, act_x0, y).style().fg, Some(Color::Magenta));
}
