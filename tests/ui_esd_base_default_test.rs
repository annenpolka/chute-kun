use chute_kun::app::App;
use chute_kun::ui::format_header_line;
use serial_test::serial;

fn write_config(dir: &std::path::Path, toml: &str) {
    let confdir = dir.join("chute_kun");
    std::fs::create_dir_all(&confdir).unwrap();
    std::fs::write(confdir.join("config.toml"), toml).unwrap();
}

#[test]
#[serial]
fn esd_defaults_to_start_of_day_when_defined() {
    let tmp = tempfile::tempdir().unwrap();
    // Note: esd_base is NOT set; only start_of_day is provided
    write_config(tmp.path(), "start_of_day = '07:15'\n");
    std::env::set_var("XDG_CONFIG_HOME", tmp.path());

    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 60);

    // With default 'start_of_day', ESD should be 07:15 + 90m = 08:45
    let s = format_header_line(9 * 60, &app);
    assert!(s.contains("ESD 08:45"), "header was: {}", s);
}

