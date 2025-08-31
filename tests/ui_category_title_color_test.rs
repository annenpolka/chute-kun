use chute_kun::{app::App, task::Category, ui};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

fn cell_at(buf: &ratatui::buffer::Buffer, x: u16, y: u16) -> &ratatui::buffer::Cell {
    &buf[(x, y)]
}

fn find_char_x_in_row(buf: &ratatui::buffer::Buffer, rect: Rect, y: u16, ch: char) -> Option<u16> {
    let y = y.min(rect.y + rect.height.saturating_sub(1));
    (rect.x..rect.x + rect.width).find(|&x| buf[(x, y)].symbol() == ch.to_string())
}

#[test]
fn title_is_colored_by_category_and_done_is_gray() {
    let backend = TestBackend::new(80, 14);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    // Three tasks; set categories Work, Home, Hobby; mark the third as Done
    app.add_task("Alpha", 20);
    app.add_task("Bravo", 15);
    app.add_task("Charlie", 10);
    app.day.tasks[0].category = Category::Work;
    app.day.tasks[1].category = Category::Home;
    app.day.tasks[2].category = Category::Hobby;

    // Mark the third task Done via key flow to stay realistic
    // Move selection to 3rd row and finish (f)
    app.handle_key(KeyCode::Char('j'));
    app.handle_key(KeyCode::Char('j'));
    app.handle_key_event(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);

    let row_a_y = list.y + 1; // Alpha (Work)
    let row_b_y = list.y + 2; // Bravo (Home)
    let row_c_y = list.y + 3; // Charlie (Hobby, Done)

    // After the category dot '●' there is a space then the title's first character.
    let xa_dot = find_char_x_in_row(&buf, list, row_a_y, '●').expect("dot A not found");
    let xa = xa_dot + 2; // skip dot and following space
    assert_eq!(cell_at(&buf, xa, row_a_y).style().fg, Some(Color::Blue)); // Work -> Blue

    let xb_dot = find_char_x_in_row(&buf, list, row_b_y, '●').expect("dot B not found");
    let xb = xb_dot + 2;
    assert_eq!(cell_at(&buf, xb, row_b_y).style().fg, Some(Color::Yellow)); // Home -> Yellow

    let xc_dot = find_char_x_in_row(&buf, list, row_c_y, '●').expect("dot C not found");
    let xc = xc_dot + 2;
    assert_eq!(cell_at(&buf, xc, row_c_y).style().fg, Some(Color::DarkGray)); // Done overrides -> Gray
}
