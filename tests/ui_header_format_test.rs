use chute_kun::{app::App, ui::format_header_line};

#[test]
fn header_shows_esd_and_totals() {
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 60);
    let now = 9 * 60; // 09:00
    let s = format_header_line(now, &app);
    assert!(s.contains("ESD 10:30"));
    assert!(s.contains("Est 90m"));
    assert!(s.contains("Act 0m"));
    // The header should not include the view label; tabs show it now.
    assert!(!s.contains("View:"));
}
