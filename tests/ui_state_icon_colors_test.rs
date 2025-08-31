use chute_kun::{app::App, task::TaskState, ui};
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
fn table_status_icons_are_colored() {
    let backend = TestBackend::new(80, 14);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    // Three tasks: we'll make A = Paused, B = Active, C = Done
    app.add_task("A", 25);
    app.add_task("B", 15);
    app.add_task("C", 10);

    // A: start then pause -> '='
    app.handle_key(KeyCode::Enter);
    app.handle_key(KeyCode::Enter);
    // B: select and start -> '>'
    app.handle_key(KeyCode::Char('j')); // move selection to B
    app.handle_key(KeyCode::Enter);
    // C: select and finish -> 'x'
    app.handle_key(KeyCode::Char('j')); // move selection to C
    app.handle_key_event(crossterm::event::KeyEvent::new(
        KeyCode::Char('f'),
        crossterm::event::KeyModifiers::NONE,
    ));

    // Sanity check states before rendering
    assert!(matches!(app.day.tasks[0].state, TaskState::Paused));
    assert!(matches!(app.day.tasks[1].state, TaskState::Active));
    assert!(matches!(app.day.tasks[2].state, TaskState::Done));

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Compute list rect to focus search within the task table area
    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, full);

    // Header is at list.y, data rows start at list.y + 1
    let row_a_y = list.y + 1; // A (Paused '=')
    let row_b_y = list.y + 2; // B (Active '>')
    let row_c_y = list.y + 3; // C (Done 'x')

    // Debug output of the three visible rows (for --nocapture runs)
    let mut row_a = String::new();
    let mut row_b = String::new();
    let mut row_c = String::new();
    for x in list.x..list.x + list.width {
        row_a.push_str(buf[(x, row_a_y)].symbol());
        row_b.push_str(buf[(x, row_b_y)].symbol());
        row_c.push_str(buf[(x, row_c_y)].symbol());
    }
    println!("ROW A: {}", row_a);
    println!("ROW B: {}", row_b);
    println!("ROW C: {}", row_c);

    // Find and assert colors
    let xa = find_char_x_in_row(&buf, list, row_a_y, '=').expect("'=' not found in row A");
    assert_eq!(cell_at(&buf, xa, row_a_y).style().fg, Some(Color::Yellow));

    let xb = find_char_x_in_row(&buf, list, row_b_y, '>').expect("'>' not found in row B");
    assert_eq!(cell_at(&buf, xb, row_b_y).style().fg, Some(Color::Green));

    let xc = find_char_x_in_row(&buf, list, row_c_y, 'x').expect("'x' not found in row C");
    assert_eq!(cell_at(&buf, xc, row_c_y).style().fg, Some(Color::DarkGray));
}

#[test]
fn active_banner_icon_is_colored() {
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("Focus", 30);
    app.handle_key(KeyCode::Enter); // start -> Active

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let full = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, banner_opt, _list, _help) = ui::compute_layout(&app, full);
    let banner = banner_opt.expect("expected active banner when a task is running");

    // Scan the banner line for '>' and assert it's green
    let y = banner.y;
    let x = (banner.x..banner.x + banner.width)
        .find(|&x| buf[(x, y)].symbol() == ">")
        .expect("'>' not found in active banner line");
    assert_eq!(cell_at(&buf, x, y).style().fg, Some(Color::Green));
}
