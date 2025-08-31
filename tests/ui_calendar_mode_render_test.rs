use chute_kun::{app::App, clock::Clock, task::Session, ui};
use ratatui::{backend::TestBackend, Terminal};

fn buf_contains(buf: &ratatui::buffer::Buffer, needle: &str) -> bool {
    for y in 0..buf.area.height {
        let mut s = String::new();
        for x in 0..buf.area.width {
            s.push_str(buf[(x, y)].symbol());
        }
        if s.contains(needle) {
            return true;
        }
    }
    false
}

#[test]
fn calendar_mode_shows_hours_and_blocks() {
    let backend = TestBackend::new(60, 18);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    app.add_task("Deep Work", 90);
    app.add_task("Email", 30);

    // Add one actual session crossing into the next hour so we can see blocks
    let base = app.config.day_start_minutes;
    if let Some(t0) = app.day.tasks.get_mut(0) {
        t0.sessions.push(Session { start_min: base + 30, end_min: Some(base + 85) });
    }

    // t once: List -> Calendar (Blocks view removed)
    app.toggle_display_mode();

    // Inject a stable "now" that won't overwrite the first plan row title
    struct TestClock(u16);
    impl Clock for TestClock {
        fn now_minutes(&self) -> u16 {
            self.0
        }
    }
    let now = app.config.day_start_minutes + 30; // 30m after base
    terminal.draw(|f| ui::draw_with_clock(f, &app, &TestClock(now))).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Expect hour labels derived from configured day start
    let base_h = (base / 60) % 24;
    let next_h = (base_h + 1) % 24;
    let h0 = format!("{:02}:00", base_h);
    let h1 = format!("{:02}:00", next_h);
    assert!(buf_contains(&buf, &h0), "should render hour tick label {}", h0);
    assert!(buf_contains(&buf, &h1), "should render next hour label {}", h1);
    assert!(buf_contains(&buf, "█"), "planned blocks should be present");
    assert!(buf_contains(&buf, "▓"), "actual session blocks should be present");
    // Task title should be shown inside the planned block region
    assert!(buf_contains(&buf, "Deep Work"), "should show task title in calendar lane");
}
