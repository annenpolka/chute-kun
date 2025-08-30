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
    let row_y = list.y + 2; // index 1 (first data row is list.y + 1)
    let col_x = list.x + 2;
    app.handle_mouse_move(col_x, row_y, area);
    assert_eq!(app.hovered_index(), Some(1), "hovered index should be 1 after mouse move");

    // Draw and assert: row 0 (selected) has selected highlight BG; row 1 (hover) has hover BG
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer();
    let list_y_top = list.y; // table header at list.y; first data row at list.y + 1
                             // Selected row (index 0) should have background somewhere across the row
    let mut blue_found = false;
    for x in list.x..list.x + list.width.min(buf.area.width) {
        if buf[(x, list_y_top + 1)].style().bg == Some(ui::SELECTED_ROW_BG) {
            blue_found = true;
            break;
        }
    }
    assert!(blue_found, "selected row should use SELECTED_ROW_BG across at least one cell");

    // Hover row (index 1) should have Cyan background somewhere across the row
    let mut cyan_found = false;
    for x in list.x..list.x + list.width.min(buf.area.width) {
        let cell = &buf[(x, list_y_top + 2)];
        if cell.style().bg == Some(ui::HOVER_ROW_BG) {
            cyan_found = true;
            break;
        }
    }
    if !cyan_found {
        // dump the hovered row for debugging
        let mut row_chars = String::new();
        let mut row_styles = String::new();
        for x in 0..buf.area.width {
            row_chars.push_str(buf[(x, list_y_top + 2)].symbol());
            row_styles.push_str(match buf[(x, list_y_top + 2)].style().bg {
                Some(c) if c == ui::SELECTED_ROW_BG => "B",
                Some(c) if c == ui::HOVER_ROW_BG => "C",
                Some(_) => "*",
                None => ".",
            });
        }
        panic!(
            "hover row not HOVER_ROW_BG. chars='{}' styles='{}' list=({},{},{},{})",
            row_chars, row_styles, list.x, list.y, list.width, list.height
        );
    }
    assert!(cyan_found, "hover row should use HOVER_ROW_BG across at least one cell");
}
