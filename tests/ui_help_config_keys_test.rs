use chute_kun::{app::App, config::Config, ui::format_help_line_for};

// Red: ヘルプの表示がコンフィグのキーアサインに追従する
#[test]
fn help_uses_configured_keys() {
    // Customize a few bindings to clearly differ from defaults
    let toml = r#"
day_start = "07:30"

[keys]
quit = "Ctrl+C"
finish_active = "g"
view_next = "Ctrl+N"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse custom config");
    let app = App::with_config(cfg);

    let s = format_help_line_for(&app);
    // Quit displays Ctrl+c (normalized lower-case letter)
    assert!(s.contains("Ctrl+c: quit"), "help should show Ctrl+c for quit, got: {}", s);
    // Finish displays single custom key 'g' (no Shift+Enter/f)
    assert!(s.contains("g: finish"), "help should show custom 'g: finish', got: {}", s);
    assert!(!s.contains("Shift+Enter/f: finish"), "should not show default finish keys: {}", s);
    // View switch uses configured Ctrl+N (normalized)
    assert!(
        s.contains("Ctrl+n: switch view"),
        "help should use Ctrl+n for view switch, got: {}",
        s
    );
}
