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
fn estimate_column_is_second_and_before_title() {
    // Wide enough that columns render with labels and values clearly.
    let backend = TestBackend::new(100, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    app.add_task("Focus Work", 30);
    app.add_task("Email", 10);

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Header row is at y=2 without active banner; at y=3 with banner.
    // Probe a couple of lines to be robust.
    let header = [2u16, 3u16]
        .iter()
        .map(|y| line_at(&buf, *y))
        .find(|line| line.contains("Plan") || line.contains("Task"))
        .expect("should find a header line containing column labels");
    // Expect column order: Plan | Est | Task | Act | Actual
    let plan_idx = header.find("Plan").expect("header should contain Plan");
    let est_idx = header.find("Est").expect("header should contain Est");
    let task_idx = header.find("Task").expect("header should contain Task");
    assert!(
        plan_idx < est_idx && est_idx < task_idx,
        "expected Plan < Est < Task in header: {}",
        header
    );

    // First data row directly below header: try the next line we didn't pick for header
    let header_y = if header.contains("Plan") {
        // Find which of y=2 or y=3 matched by re-checking
        if line_at(&buf, 2).contains("Plan") || line_at(&buf, 2).contains("Task") {
            2
        } else {
            3
        }
    } else {
        2
    };
    let row = line_at(&buf, header_y + 1);
    // Estimate value like "30m" should appear before the task title
    let title_idx = row.find("Focus Work").expect("row should contain task title");
    let est_val_idx = row.find("30m").expect("row should show estimate minutes as Xm");
    assert!(est_val_idx < title_idx, "estimate column should be before title, row: {}", row);
    // Title cell should no longer include the "(est:" suffix inside it
    assert!(
        !row.contains("(est:"),
        "title cell should not contain (est:...) when Est column exists; row: {}",
        row
    );
}
