use chute_kun::{config::Config, storage};
use std::env;
use std::path::PathBuf;

// Red: config.toml's state_path should take precedence over --state (CLI override).
#[test]
fn config_state_path_overrides_cli_override() {
    // Ensure env does not influence resolution
    env::remove_var("CHUTE_KUN_STATE");
    env::remove_var("XDG_DATA_HOME");

    let cfg = Config::from_toml_str(
        r#"
day_start = "09:00"
state_path = "/tmp/chute_kun_from_config/snapshot.toml"
"#,
    )
    .expect("parse config with state_path");

    let cli_override = Some(PathBuf::from("/tmp/cli_override/snap.toml"));
    let chosen = storage::resolve_state_path(&cfg, cli_override)
        .expect("some path chosen");

    assert_eq!(
        chosen,
        PathBuf::from("/tmp/chute_kun_from_config/snapshot.toml")
    );
}

