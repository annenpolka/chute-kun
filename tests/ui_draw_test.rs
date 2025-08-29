use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn draw_smoke_test() {
    let backend = TestBackend::new(20, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
}
