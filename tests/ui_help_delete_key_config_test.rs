use chute_kun::{app::App, config::Config, ui::format_help_line_for};

// Red: ヘルプの delete 表記がコンフィグのキーに追従する
#[test]
fn help_shows_configured_delete_key() {
    let toml = r#"
day_start = "09:00"

[keys]
delete = "d"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse config");
    let app = App::with_config(cfg);
    let s = format_help_line_for(&app);
    assert!(s.contains("d: delete"), "help should reflect configured delete key, got: {}", s);
}
