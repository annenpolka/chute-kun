use chute_kun::{app::App, config::Config, storage};

fn build_sample_app() -> App {
    let mut app = App::new();
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-30");
    // Today: A (will be Done -> Past), D (Planned -> Active)
    let _a = app.add_task("A", 30);
    let _b = app.add_task("B", 15);
    // Move B to Future via postpone
    app.select_down(); // select B
    app.postpone_selected(); // B -> Future (Planned)

    // Start and finish A (stays in Today as Done for the same day)
    // After postponing, only A remains at index 0
    app.day.start(0);
    app.finish_active(); // A -> Done (Today)

    // Simulate next day: sweep moves Done items to Past
    app.sweep_done_before(20250831);

    // Add D and start it (Active in Today)
    let _d = app.add_task("D", 20);
    app.day.start(0); // start D
    app
}

#[test]
fn snapshot_roundtrip_preserves_lists_and_states() {
    let app = build_sample_app();

    // Serialize to TOML string
    let toml_str = storage::save_to_string(&app).expect("serialize to toml");

    // Load back into a new App using default config
    let cfg = Config::default();
    let loaded = storage::load_from_str(&toml_str, cfg).expect("deserialize from toml");

    // Today: one task (D) and it should be Active
    assert_eq!(loaded.day.tasks.len(), 1);
    assert_eq!(loaded.day.tasks[0].title, "D");
    assert!(matches!(loaded.day.tasks[0].state, chute_kun::task::TaskState::Active));

    // Future: one task (B) Planned
    assert_eq!(loaded.tomorrow_tasks().len(), 1);
    assert_eq!(loaded.tomorrow_tasks()[0].title, "B");
    assert!(matches!(loaded.tomorrow_tasks()[0].state, chute_kun::task::TaskState::Planned));

    // Past: one task (A) Done
    assert_eq!(loaded.history_tasks().len(), 1);
    assert_eq!(loaded.history_tasks()[0].title, "A");
    assert!(matches!(loaded.history_tasks()[0].state, chute_kun::task::TaskState::Done));
}

#[test]
fn toml_layout_is_git_friendly_and_ordered() {
    let app = build_sample_app();
    let toml_str = storage::save_to_string(&app).expect("serialize to toml");

    // top-level version present
    assert!(toml_str.contains("version = 1"));

    // one array-of-tables header per task, preserving order
    let today_count = toml_str.match_indices("[[today]]").count();
    let future_count = toml_str.match_indices("[[future]]").count();
    let past_count = toml_str.match_indices("[[past]]").count();
    assert_eq!(today_count, 1, "today should have 1 task block");
    assert_eq!(future_count, 1, "future should have 1 task block");
    assert_eq!(past_count, 1, "past should have 1 task block");

    // Fields appear as simple key = value pairs (diff-friendly)
    assert!(toml_str.contains("title = \"D\""));
    assert!(toml_str.contains("estimate_min = 20"));
    assert!(toml_str.contains("state = \"Active\""));
}
