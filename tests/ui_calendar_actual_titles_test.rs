use chute_kun::{app::App, task::Session, ui};
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

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

#[test]
fn calendar_displays_titles_for_past_actual_sessions() {
    let backend = TestBackend::new(72, 18);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes; // expected 09:00 in defaults

    let title = "Write Doc";
    app.add_task(title, 30);
    // Closed session from base+5 to base+25
    if let Some(t) = app.day.tasks.get_mut(0) {
        t.sessions.push(Session { start_min: base + 5, end_min: Some(base + 25) });
    }

    // Switch to Calendar and draw at a later time
    app.toggle_display_mode();
    let clock = FixedClock(base + 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    assert!(buf_contains(&buf, title), "calendar actual lane should show past session title");
}
