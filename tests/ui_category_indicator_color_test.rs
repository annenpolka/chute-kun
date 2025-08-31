use chute_kun::{app::App, task::Category, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

fn cell_at(buf: &ratatui::buffer::Buffer, x: u16, y: u16) -> &ratatui::buffer::Cell {
    &buf[(x, y)]
}

fn find_char_x_in_row(buf: &ratatui::buffer::Buffer, rect: Rect, y: u16, ch: char) -> Option<u16> {
    let y = y.min(rect.y + rect.height.saturating_sub(1));
    (rect.x..rect.x + rect.width).find(|&x| buf[(x, y)].symbol() == ch.to_string())
}

#[test]
fn category_indicator_renders_expected_colors() {
    let backend = TestBackend::new(80, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 25);
    app.add_task("B", 15);
    app.add_task("C", 10);
    // Assign categories directly for the test
    app.day.tasks[0].category = Category::Work;
    app.day.tasks[1].category = Category::Home;
    app.day.tasks[2].category = Category::Hobby;

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);

    // Data rows start at list.y + 1
    let row_a_y = list.y + 1;
    let row_b_y = list.y + 2;
    let row_c_y = list.y + 3;

    // Find the category dot '●' in each row and assert its color
    let xa = find_char_x_in_row(&buf, list, row_a_y, '●').expect("dot not found in row A");
    assert_eq!(cell_at(&buf, xa, row_a_y).style().fg, Some(Color::Blue));

    let xb = find_char_x_in_row(&buf, list, row_b_y, '●').expect("dot not found in row B");
    assert_eq!(cell_at(&buf, xb, row_b_y).style().fg, Some(Color::Yellow));

    let xc = find_char_x_in_row(&buf, list, row_c_y, '●').expect("dot not found in row C");
    assert_eq!(cell_at(&buf, xc, row_c_y).style().fg, Some(Color::Magenta));
}

#[test]
fn pressing_c_cycles_category_colors() {
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Focus", 30);

    // Default should be General (white dot)
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);
    let y = list.y + 1;
    let x0 = find_char_x_in_row(&buf, list, y, '●').expect("dot not found initially");
    assert_eq!(cell_at(&buf, x0, y).style().fg, Some(Color::White));

    // Cycle: General -> Work
    app.handle_key(KeyCode::Char('c'));
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let x1 = find_char_x_in_row(&buf, list, y, '●').expect("dot not found after 1st cycle");
    assert_eq!(cell_at(&buf, x1, y).style().fg, Some(Color::Blue));

    // Work -> Home
    app.handle_key(KeyCode::Char('c'));
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let x2 = find_char_x_in_row(&buf, list, y, '●').expect("dot not found after 2nd cycle");
    assert_eq!(cell_at(&buf, x2, y).style().fg, Some(Color::Yellow));

    // Home -> Hobby
    app.handle_key(KeyCode::Char('c'));
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let x3 = find_char_x_in_row(&buf, list, y, '●').expect("dot not found after 3rd cycle");
    assert_eq!(cell_at(&buf, x3, y).style().fg, Some(Color::Magenta));
}
