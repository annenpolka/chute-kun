use chute_kun::{app::App, task::Session, ui};
use ratatui::{backend::TestBackend, Terminal};

fn buf_to_string(buf: &ratatui::buffer::Buffer) -> String {
    let mut s = String::new();
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            s.push_str(buf[(x, y)].symbol());
        }
        s.push('\n');
    }
    s
}

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

#[test]
fn longer_session_title_wins_on_overlap() {
    let backend = TestBackend::new(64, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes; // 09:00 by default

    // Two tasks with sessions mapping to the same calendar row when height is small
    let longer = "Longer";
    let shorter = "Short";
    app.add_task(longer, 15);
    app.add_task(shorter, 10);
    if let Some(t) = app.day.tasks.get_mut(0) {
        t.sessions.push(Session { start_min: base + 5, end_min: Some(base + 25) });
        // 20m
    }
    if let Some(t) = app.day.tasks.get_mut(1) {
        // Start at the same minute to force the same row after quantization
        t.sessions.push(Session { start_min: base + 5, end_min: Some(base + 12) });
        // 7m
    }

    app.toggle_display_mode(); // Calendar
    let clock = FixedClock(base + 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = buf_to_string(&buf);
    // Title must appear on a line that represents Actual lane (contains block char '▓')
    let has_longer_on_actual = full.lines().any(|line| match (line.find(longer), line.find('▓')) {
        (Some(ti), Some(bi)) => bi > ti,
        _ => false,
    });
    assert!(has_longer_on_actual, "expected longer session title to be printed on actual lane");
}
