use chute_kun::{app::App, config::Config, ui::format_help_line_for};

// Red→Green: Ctrl+LETTER の大文字小文字を設定で許容し、表示は Ctrl+小文字 で出す
#[test]
fn help_normalizes_ctrl_letter_to_lowercase() {
    let toml = r#"
day_start = "09:00"

[keys]
delete = "Ctrl+D"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse config");
    let app = App::with_config(cfg);
    let s = format_help_line_for(&app);
    assert!(s.contains("Ctrl+d: delete"), "expected normalized Ctrl+d in help, got: {}", s);
}
