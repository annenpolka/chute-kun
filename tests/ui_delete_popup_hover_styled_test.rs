use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

#[test]
fn hovering_button_turns_cyan() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Danger", 15);
    app.handle_key(KeyCode::Char('x'));
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_delete_popup_rect(&app, area).unwrap();
    let (_del, cancel) = ui::delete_popup_button_hitboxes(&app, popup);

    // Hover the Cancel button
    app.handle_mouse_move(cancel.x, cancel.y, area);

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer();
    let cell = &buf[(cancel.x, cancel.y)];
    assert_eq!(cell.style().bg, Some(Color::Cyan));
}
