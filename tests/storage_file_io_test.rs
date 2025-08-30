use chute_kun::{app::App, config::Config, storage};
use tempfile::tempdir;

#[test]
fn save_and_load_via_files_roundtrip() {
    let dir = tempdir().expect("tempdir");
    let p = dir.path().join("snapshot.toml");

    // build simple app
    let mut app = App::new();
    app.add_task("A", 10);

    storage::save_to_path(&app, &p).expect("save file");
    let loaded =
        storage::load_from_path(&p, Config::default()).expect("load ok").expect("file exists");

    assert_eq!(loaded.day.tasks.len(), 1);
    assert_eq!(loaded.day.tasks[0].title, "A");
}
