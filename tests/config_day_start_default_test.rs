use chute_kun::{app::App, ui};

// Default display should start from a fixed 09:00 base, not system now.
#[test]
fn default_display_uses_fixed_9am_base() {
    // Ensure external config is ignored for this test
    std::env::set_var("CHUTE_KUN_DISABLE_CONFIG", "1");
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);

    let lines = ui::format_task_lines(&app);
    assert!(lines[0].starts_with("09:00 "), "expected 09:00 start, got: {}", lines[0]);
    assert!(lines[1].starts_with("09:30 "), "expected 09:30 second, got: {}", lines[1]);
}
