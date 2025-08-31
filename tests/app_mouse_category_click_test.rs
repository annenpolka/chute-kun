use chute_kun::{app::App, task::Category, ui};
use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

fn find_char_x_in_row(
    buf: &ratatui::buffer::Buffer,
    rect: Rect,
    y: u16,
    ch: char,
) -> Option<u16> {
    let y = y.min(rect.y + rect.height.saturating_sub(1));
    (rect.x..rect.x + rect.width).find(|&x| buf[(x, y)].symbol() == ch.to_string())
}

#[test]
fn left_click_on_category_dot_cycles_category() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Alpha", 20);

    // Before: default is General
    assert!(matches!(app.day.tasks[0].category, Category::General));

    // Render once to discover dot coordinates
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);
    let row_y = list.y + 1; // first data row
    let x = find_char_x_in_row(&buf, list, row_y, '●').expect("did not find category dot");

    // Simulate mouse left click on the dot
    let ev = MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: x, row: row_y, modifiers: KeyModifiers::empty() };
    app.handle_mouse_event(ev, full);

    // After: category should cycle to Work
    assert!(matches!(app.day.tasks[0].category, Category::Work));
}

