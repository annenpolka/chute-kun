use chute_kun::{app::App, task::Session, ui};
use ratatui::{backend::TestBackend, Terminal};

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

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

#[test]
fn calendar_shows_now_line_and_active_session() {
    let backend = TestBackend::new(70, 18);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes; // usually 09:00

    // Two tasks totalling 2h
    app.add_task("Focus", 60);
    app.add_task("Email", 60);

    // Simulate an active session: start at base+10, open-ended
    if let Some(t0) = app.day.tasks.get_mut(0) {
        t0.sessions.push(Session { start_min: base + 10, end_min: None });
    }
    // Mark task 0 as Active so the UI knows the active title
    app.day.start(0);

    // Switch to Calendar
    app.toggle_display_mode();

    // Draw with injected clock at base+30
    let clock = FixedClock(base + 30);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let lines = buffer_to_strings(terminal.backend().buffer());

    // Expect a now line (─) somewhere
    assert!(lines.iter().any(|l| l.contains("─")), "should draw a red now line using '─'");
    // Expect Now label with HH:MM on the same line
    let hh = (clock.0 / 60) % 24;
    let mm = clock.0 % 60;
    let now_label = format!("Now {:02}:{:02}", hh, mm);
    let has_now_label = lines.iter().any(|l| l.contains(&now_label));
    assert!(has_now_label, "should show now label: {}", now_label);
    // Expect the active task title to appear on the now line (Actual側)
    assert!(
        lines.iter().any(|l| l.contains("─") && l.contains("Focus")),
        "now line should include active task title on Actual lane"
    );
}
