use chute_kun::{app::App, task::Session, ui};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

fn cell(buf: &ratatui::buffer::Buffer, x: u16, y: u16) -> &ratatui::buffer::Cell {
    &buf[(x, y)]
}

fn find_row_with_both<'a>(buf: &'a ratatui::buffer::Buffer, a: char, b: char) -> Option<u16> {
    for y in 0..buf.area.height {
        let mut has_a = false;
        let mut has_b = false;
        for x in 0..buf.area.width {
            let s = buf[(x, y)].symbol();
            if s == a.to_string() {
                has_a = true;
            }
            if s == b.to_string() {
                has_b = true;
            }
        }
        if has_a && has_b {
            return Some(y);
        }
    }
    None
}

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

#[test]
fn overlapping_actual_sessions_render_side_by_side_columns() {
    let backend = TestBackend::new(72, 18);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes; // typically 09:00

    // Two tasks with overlapping sessions starting at the same minute
    let a = "Alpha"; // starts at base+10 for 20m
    let b = "Bravo"; // starts at base+10 for 15m
    app.add_task(a, 40);
    app.add_task(b, 30);
    if let Some(t0) = app.day.tasks.get_mut(0) {
        t0.sessions.push(Session { start_min: base + 10, end_min: Some(base + 30) });
    }
    if let Some(t1) = app.day.tasks.get_mut(1) {
        t1.sessions.push(Session { start_min: base + 10, end_min: Some(base + 25) });
    }

    app.toggle_display_mode(); // Calendar
    let clock = FixedClock(base + 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Locate content area and act lane bounds
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);
    let gutter = 6u16; // HH:00
    let lanes_w = list.width.saturating_sub(gutter + 2); // gutter + 2 gaps
    let lane_w = (lanes_w.saturating_sub(1)) / 2;
    let plan_x0 = list.x + gutter + 1;
    let act_x0 = plan_x0 + lane_w + 1;

    // Find a row that shows both 'A' and 'B' (titles overlaid at start of each block)
    let y = find_row_with_both(&buf, 'A', 'B').expect("expected both titles on same row");
    let xa = (act_x0..act_x0 + lane_w)
        .find(|&x| cell(&buf, x, y).symbol() == "A")
        .expect("A not in act lane");
    let xb = (act_x0..act_x0 + lane_w)
        .find(|&x| cell(&buf, x, y).symbol() == "B")
        .expect("B not in act lane");

    // Expect that the two titles are placed in different horizontal regions (columns)
    assert!((xa as i32 - xb as i32).abs() as u16 >= 2, "titles should not overlap horizontally");
}
