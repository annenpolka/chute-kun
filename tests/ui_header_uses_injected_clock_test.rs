use chute_kun::clock::Clock;
use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

struct FixedClock(u16);
impl Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

#[test]
fn header_contains_esd_and_totals_from_injected_local_minutes() {
    // Arrange
    let backend = TestBackend::new(80, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 60);
    let clock = FixedClock(9 * 60); // 09:00 local

    // Act
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();

    // Assert: read first line and check substrings
    let backend = terminal.backend();
    let buf = backend.buffer();
    let width = buf.area.width as usize;
    let mut first_line = String::new();
    for x in 0..width as u16 {
        let cell = &buf[(x, 0)];
        first_line.push_str(cell.symbol());
    }
    // ESD 09:00 + (30+60)=90 => 10:30
    assert!(first_line.contains("ESD 10:30"), "header should show ESD 10:30, got: {}", first_line);
    assert!(
        first_line.contains("Est 90m 0s"),
        "header should include Est 90m 0s, got: {}",
        first_line
    );
    assert!(
        first_line.contains("Act 0m 0s"),
        "header should include Act 0m 0s, got: {}",
        first_line
    );
}
