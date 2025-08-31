use chute_kun::{app::App, config::Config, ui::format_help_line_for};

// Red->Green: カテゴリー切替キーは設定に従ってヘルプに表示される
#[test]
fn help_uses_configured_category_key() {
    let toml = r#"
day_start = "09:00"

[keys]
category_cycle = "z"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse config");
    let app = App::with_config(cfg);
    let s = format_help_line_for(&app);
    assert!(s.contains("z: category"), "expected custom category key in help, got: {}", s);
    assert!(!s.contains("c: category"), "should not show default when overridden: {}", s);
}

