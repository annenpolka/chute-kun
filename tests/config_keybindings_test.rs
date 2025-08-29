use chute_kun::app::App;
use crossterm::event::KeyCode;
use std::fs;

#[test]
fn quit_key_can_be_customized_via_config() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg_dir = tmp.path().join("chute_kun");
    std::env::set_var("CHUTE_KUN_CONFIG_DIR", &cfg_dir);
    fs::create_dir_all(&cfg_dir).unwrap();
    fs::write(
        cfg_dir.join("config.toml"),
        r#"
start_of_day = "09:00"
[keybindings]
quit = "x"
"#,
    )
    .unwrap();

    let mut app = App::new();
    assert!(!app.should_quit);
    // Press custom quit key
    app.handle_key(KeyCode::Char('x'));
    assert!(app.should_quit);
}
