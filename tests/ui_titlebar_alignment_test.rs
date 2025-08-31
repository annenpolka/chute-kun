use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

fn line_at(buf: &ratatui::buffer::Buffer, y: u16) -> String {
    let mut s = String::new();
    for x in 0..buf.area.width {
        s.push_str(buf[(x, y)].symbol());
    }
    s
}

#[test]
fn titlebar_left_aligns_stats_and_right_aligns_buttons() {
    // Wide enough that right alignment is obvious
    let backend = TestBackend::new(80, 5);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    // Add some tasks so header values are non-trivial
    app.add_task("A", 25);
    app.add_task("B", 10);

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let first = line_at(&buf, 0);

    // Sanity: contains both left stats and rightmost Delete button label
    assert!(first.contains("ESD"), "header should contain ESD stats: {}", first);
    assert!(first.contains("Delete"), "header should render Delete button: {}", first);

    let width = buf.area.width as usize;
    let pos_esd = first.find("ESD").unwrap();
    let pos_delete = first.find("Delete").unwrap();

    // Expect the stats to start near the left border (after the top-left corner)
    assert!(
        pos_esd <= 3,
        "expected left-aligned stats (ESD) near start, got index {} in: {}",
        pos_esd,
        first
    );

    // Expect buttons group to be right-aligned: Delete should sit near the right edge
    // Allow a small slack of 3 cells before the top-right corner.
    assert!(
        pos_delete >= width.saturating_sub(2 + "Delete".len() + 3),
        "expected right-aligned buttons; Delete too far left at {} in: {}",
        pos_delete,
        first
    );
}
