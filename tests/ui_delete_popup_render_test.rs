use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::style::Color;
use ratatui::{backend::TestBackend, Terminal};

// Red: 削除確認はセンタリングされたポップアップで赤色テキストが表示される
#[test]
fn delete_confirmation_renders_centered_red_popup() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Danger", 15);
    app.add_task("Keep", 10);

    // Trigger delete confirm and draw
    app.handle_key(KeyCode::Char('x'));
    terminal.draw(|f| ui::draw(f, &app)).unwrap();

    let backend = terminal.backend();
    let buf = backend.buffer();

    // Use UI helper to compute the popup geometry so test tracks UI layout
    let area = ratatui::layout::Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_delete_popup_rect(&app, area).expect("popup rect");
    let text_x = popup.x + 1; // inner left
    let text_y = popup.y + 1; // inner top line

    let c = &buf[(text_x, text_y)];
    assert_eq!(c.symbol(), "D");
    assert_eq!(c.style().fg, Some(Color::Red));
}
