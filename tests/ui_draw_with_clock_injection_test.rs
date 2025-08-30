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
fn draw_runs_with_injected_clock() {
    let backend = TestBackend::new(20, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();
    let clock = FixedClock(9 * 60 + 12);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
}
