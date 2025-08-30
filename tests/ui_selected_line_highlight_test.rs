use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn selected_line_has_background_highlight() {
    // Arrange a small terminal and an app with a couple of tasks
    // Height is set so that list keeps at least 2 rows after help wraps
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);

    // Act: initial draw (selected index = 0)
    terminal.draw(|f| ui::draw(f, &app)).unwrap();

    // Assert: the first list row (inside the bordered block) has Blue BG
    {
        let backend = terminal.backend();
        let buf = backend.buffer();
        // Inner area starts at (1,1). Tabs are at y=1, list starts at y=2, help at bottom.
        let list_y_top = 2u16;
        let first_cell = &buf[(1, list_y_top)]; // leftmost cell of first list row
        assert!(
            first_cell.style().bg.is_some(),
            "expected selected row to have a background, got {:?}",
            first_cell.style().bg
        );
    }

    // Move selection down and redraw; highlight should move one row down
    app.select_down();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    {
        let backend = terminal.backend();
        let buf = backend.buffer();
        let list_y_top = 2u16;
        let second_row_cell = &buf[(1, list_y_top + 1)];
        assert!(
            second_row_cell.style().bg.is_some(),
            "expected second row to be highlighted after moving down"
        );
    }
}
