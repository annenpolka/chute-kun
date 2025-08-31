use chute_kun::config::Config;
use std::env;

// Red: ${VAR} and ~ expansion in state_path
#[test]
fn state_path_expands_env_and_tilde() {
    // Prepare sandboxed env
    let tmp = tempfile::tempdir().expect("tempdir");
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).ok();
    env::set_var("HOME", &home);
    env::set_var("XDG_DATA_HOME", &home);

    // ${XDG_DATA_HOME} expansion (whitelisted)
    let cfg1 = Config::from_toml_str(
        r#"
day_start = "09:00"
state_path = "${XDG_DATA_HOME}/chute_kun/s1.toml"
"#,
    )
    .expect("parse config with var");
    assert_eq!(
        cfg1
            .state_path
            .as_ref()
            .expect("state_path present"),
        &home.join("chute_kun/s1.toml")
    );

    // ~ expansion
    let cfg2 = Config::from_toml_str(
        r#"
day_start = "09:00"
state_path = "~/.local/share/chute_kun/s2.toml"
"#,
    )
    .expect("parse config with tilde");
    assert_eq!(
        cfg2
            .state_path
            .as_ref()
            .expect("state_path present"),
        &home.join(".local/share/chute_kun/s2.toml")
    );
}

// Unknown env var should disable state_path (ignored), not expand to empty or error.
#[test]
fn unknown_env_var_disables_state_path() {
    std::env::remove_var("NOT_DEFINED_VAR_12345");
    let cfg = Config::from_toml_str(
        r#"
day_start = "09:00"
state_path = "${NOT_DEFINED_VAR_12345}/foo.toml"
"#,
    )
    .expect("parse config");
    assert!(cfg.state_path.is_none());
}
