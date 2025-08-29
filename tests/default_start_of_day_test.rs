use chute_kun::app::App;
use chute_kun::ui::format_header_line;
use serial_test::serial;

#[test]
#[serial]
fn schedule_base_defaults_to_09_00_without_config() {
    // Ensure no XDG_CONFIG_HOME is set to a path with a config
    std::env::remove_var("XDG_CONFIG_HOME");

    let app = App::new();
    // Passing any 'now' should not change the base; expect 09:00
    assert_eq!(app.schedule_start_minute_from(13 * 60), 9 * 60);
}

#[test]
#[serial]
fn esd_defaults_to_09_00_without_config() {
    std::env::remove_var("XDG_CONFIG_HOME");

    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 60);

    // Even if now=06:00, ESD should be 09:00 + 90m = 10:30
    let s = format_header_line(6 * 60, &app);
    assert!(s.contains("ESD 10:30"), "header was: {}", s);
}

