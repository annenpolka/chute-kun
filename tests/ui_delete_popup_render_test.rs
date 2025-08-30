use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::style::Color;
use ratatui::{backend::TestBackend, Terminal};
use unicode_width::UnicodeWidthStr;

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

    // Outer area 60x12 -> inner is (x=1,y=1,w=58,h=10)
    let inner_x = 1u16;
    let inner_y = 1u16;
    let inner_w = 58u16;
    let inner_h = 10u16;

    // Derive popup geometry like the UI code
    let msg = format!("Delete? — {}  (Enter=Delete Esc=Cancel)", "Danger");
    let content_w = UnicodeWidthStr::width(msg.as_str()) as u16;
    let popup_w = (content_w + 4).min(inner_w).max(20).min(inner_w);
    let popup_h = 3u16;
    let px = inner_x + (inner_w - popup_w) / 2;
    let py = inner_y + (inner_h - popup_h) / 2;
    let text_x = px + 1;
    let text_y = py + 1;

    let c = &buf[(text_x, text_y)];
    assert_eq!(c.symbol(), "D");
    assert_eq!(c.style().fg, Some(Color::Red));
}
