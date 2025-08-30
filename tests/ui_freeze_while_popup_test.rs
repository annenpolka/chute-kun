use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, Terminal};

fn header_line(buf: &ratatui::buffer::Buffer) -> String {
    let width = buf.area.width as usize;
    let mut s = String::new();
    for x in 0..width as u16 {
        s.push_str(buf.get(x, 0).symbol());
    }
    s
}

// Red: ポップアップ表示中はヘッダ（Act 秒）が変化しない
#[test]
fn header_does_not_change_while_confirm_delete() {
    let backend = TestBackend::new(80, 6);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 30);
    // start active
    app.handle_key(KeyCode::Enter);

    // baseline draw
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let baseline = header_line(&buf);
    assert!(baseline.contains("Act 0m 0s"), "expected initial Act 0m 0s, got: {}", baseline);

    // open confirm and tick for 5s — should not reflect in header
    app.handle_key(KeyCode::Char('x'));
    app.tick(5);
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let frozen = header_line(&buf);
    assert!(
        frozen.contains("Act 0m 0s"),
        "header should remain frozen while confirming, got: {}",
        frozen
    );

    // cancel confirm, tick 5s — now header should update
    app.handle_key(KeyCode::Esc);
    app.tick(5);
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let after = header_line(&buf);
    assert!(
        after.contains("Act 0m 5s"),
        "header should reflect 5s after closing popup, got: {}",
        after
    );
}

