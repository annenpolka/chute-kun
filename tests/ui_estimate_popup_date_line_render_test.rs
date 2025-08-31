use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, Terminal};

fn line_at(buf: &ratatui::buffer::Buffer, y: u16) -> String {
    let mut s = String::new();
    for x in 0..buf.area.width {
        s.push_str(buf[(x, y)].symbol());
    }
    s
}

#[test]
fn estimate_edit_popup_shows_date_line_without_hints() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let backend = TestBackend::new(80, 14);
    let mut term = Terminal::new(backend).unwrap();

    let mut app = App::new();
    app.add_task("A", 25);
    app.handle_key(KeyCode::Char('e'));

    term.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = term.backend().buffer().clone();

    // inner top starts y=1; tabs at y=1; popup will be centered roughly in middle
    // Instead of chasing y, scan lines for one that starts with " Date:" inside the popup
    let mut found = None;
    for y in 0..buf.area.height {
        let s = line_at(&buf, y);
        if s.contains("Date:") {
            found = Some(s);
            break;
        }
    }
    let line = found.expect("date line not found");
    assert!(line.contains("Date:"));
    // 2025-08-31 is Sunday; weekday should be shown in parentheses
    assert!(line.contains("(Sun)"), "weekday should be shown: {}", line);
}

#[test]
fn new_task_estimate_popup_shows_date_line_without_hints() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let backend = TestBackend::new(80, 14);
    let mut term = Terminal::new(backend).unwrap();

    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Char('B'));
    app.handle_key(KeyCode::Enter);
    assert!(app.is_new_task_estimate());

    term.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = term.backend().buffer().clone();
    let mut found = None;
    for y in 0..buf.area.height {
        let s = line_at(&buf, y);
        if s.contains("Date:") {
            found = Some(s);
            break;
        }
    }
    let line = found.expect("date line not found");
    assert!(line.contains("Date:"));
    assert!(line.contains("(Sun)"), "weekday should be shown: {}", line);
}
