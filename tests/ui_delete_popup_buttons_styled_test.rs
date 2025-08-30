use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, style::Color, Terminal};

#[test]
fn delete_popup_buttons_have_colored_backgrounds() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Danger", 15);
    app.handle_key(KeyCode::Char('x'));

    // Draw and fetch popup + button boxes
    let area = ratatui::layout::Rect { x: 0, y: 0, width: 60, height: 12 };
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let popup = ui::compute_delete_popup_rect(&app, area).unwrap();
    let (del, cancel) = ui::delete_popup_button_hitboxes(&app, popup);

    let buf = terminal.backend().buffer();
    // First cell of each button should have a non-default background color
    let del_cell = &buf[(del.x, del.y)];
    let cancel_cell = &buf[(cancel.x, cancel.y)];
    assert_eq!(del_cell.style().bg, Some(Color::Red));
    assert_eq!(cancel_cell.style().bg, Some(Color::Gray));
}
