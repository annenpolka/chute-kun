use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

fn cell_at(buf: &ratatui::buffer::Buffer, x: u16, y: u16) -> &ratatui::buffer::Cell {
    &buf[(x, y)]
}

// 固定時刻付きタスクのプラン時刻（左端の Plan 列）は色が変わる
#[test]
fn fixed_time_plan_cell_is_colored() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 20);

    // 固定: 09:30（色は Cyan を想定）
    use crossterm::event::KeyCode;
    app.handle_key(KeyCode::Char(':'));
    for c in "at 09:30".chars() {
        app.handle_key(KeyCode::Char(c));
    }
    app.handle_key(KeyCode::Enter);

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);

    // ヘッダ行の次が 1 行目。Plan 列は左端（枠の内側）
    let y = list.y + 1; // 1st data row
    let x = list.x; // first column cell start
    assert_eq!(cell_at(&buf, x, y).style().fg, Some(Color::Cyan));
}
