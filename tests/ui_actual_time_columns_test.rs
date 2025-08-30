use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

#[test]
fn table_shows_left_columns_for_plan_and_actual() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);

    let clock = FixedClock(9 * 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();

    let buf = terminal.backend().buffer().clone();
    // Row 2: table header should contain the column labels
    let mut header_line = String::new();
    for x in 0..buf.area.width {
        header_line.push_str(buf[(x, 2)].symbol());
    }
    assert!(header_line.contains("Plan"), "header: {}", header_line);
    assert!(header_line.contains("Actual"), "header: {}", header_line);

    // Buffer should contain planned time, actual placeholder, and title somewhere
    let mut all = String::new();
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            all.push_str(buf[(x, y)].symbol());
        }
        all.push('\n');
    }
    assert!(all.contains("09:00"), "buf: {}", all);
    assert!(all.contains("--:--"), "buf: {}", all);
    assert!(all.contains("A"), "buf: {}", all);
}

#[test]
fn table_actual_column_shows_last_finish_time_only() {
    let backend = TestBackend::new(80, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);

    // Craft full sessions deterministically
    {
        let t = &mut app.day.tasks[0];
        t.sessions.push(chute_kun::task::Session { start_min: 9 * 60 + 5, end_min: None });
    }
    {
        let t = &mut app.day.tasks[1];
        t.sessions
            .push(chute_kun::task::Session { start_min: 9 * 60 + 30, end_min: Some(9 * 60 + 50) });
        t.sessions
            .push(chute_kun::task::Session { start_min: 10 * 60 + 5, end_min: Some(10 * 60 + 10) });
        t.finished_at_min = Some(10 * 60 + 10);
    }

    let clock = FixedClock(9 * 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();

    let buf = terminal.backend().buffer().clone();
    let mut all = String::new();
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            all.push_str(buf[(x, y)].symbol());
        }
        all.push('\n');
    }
    // Row for A: no finished session yet -> placeholder
    assert!(all.contains("--:--"), "expected placeholder for unfinished task, buffer: {}", all);
    // Row for B: show only the last finished time (10:10), not the whole ranges
    assert!(all.contains("10:10"), "buffer: {}", all);
    assert!(!all.contains("09:30-09:50"), "should not list full session ranges: {}", all);
}
