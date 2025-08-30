use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

#[test]
fn hover_row_renders_in_cyan_while_selection_stays_blue() {
    // Arrange terminal and app with two tasks; default selection is index 0
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // Compute list rect and move the mouse over the second row
    let area = Rect { x: 0, y: 0, width: 40, height: 10 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let row_y = list.y + 1; // index 1
    let col_x = list.x + 2;
    app.handle_mouse_move(col_x, row_y, area);

    // Draw and assert: row 0 (selected) is Blue; row 1 (hover) is Cyan
    use ratatui::style::Color;
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer();
    let list_y_top = 2u16; // tabs at y=1, list at y=2
    let sel_cell = &buf[(1, list_y_top)];
    let hover_cell = &buf[(1, list_y_top + 1)];
    assert_eq!(sel_cell.style().bg, Some(Color::Blue), "selected should be Blue");
    assert_eq!(hover_cell.style().bg, Some(Color::Cyan), "hover should be Cyan");
}
