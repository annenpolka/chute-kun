use chute_kun::app::App;
use chute_kun::config::Config;
use chute_kun::task::TaskState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// Red: 非アクティブでも選択行をfinishでき、Activeは維持される
#[test]
fn finish_selected_non_active_keeps_active_running() {
    let mut app = App::with_config(Config::default());
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-30");
    // Today: A, B
    app.add_task("A", 30);
    app.add_task("B", 10);

    // Start A (index 0 becomes Active)
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.day.active_index(), Some(0));
    assert!(matches!(app.day.tasks[0].state, TaskState::Active));

    // Move selection to B (index 1)
    app.select_down();
    assert_eq!(app.selected_index(), 1);

    // Press 'f' to finish the selected (B), even though it's not Active
    let ev = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty());
    app.handle_key_event(ev);

    // B should be Done but still in Today
    assert_eq!(app.history_tasks().len(), 0);
    assert_eq!(app.day.tasks.len(), 2);
    assert!(matches!(app.day.tasks[1].state, TaskState::Done));
    assert!(app.day.tasks[1].done_ymd.is_some());

    // A should remain Active in Today
    assert_eq!(app.day.active_index(), Some(0));
    assert!(matches!(app.day.tasks[0].state, TaskState::Active));

    // Next day: sweep moves B to history
    app.sweep_done_before(20250831);
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].title, "B");
}

// Red: 選択がActiveの場合は従来通りfinishして履歴へ移動
#[test]
fn finish_selected_active_behaves_like_finish_active() {
    let mut app = App::with_config(Config::default());
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-30");
    app.add_task("A", 25);

    // Start A and keep selection at 0
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.selected_index(), 0);
    assert_eq!(app.day.active_index(), Some(0));

    // Finish via 'f'
    let ev = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty());
    app.handle_key_event(ev);

    // Done but remains in Today; active cleared
    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].title, "A");
    assert!(matches!(app.day.tasks[0].state, TaskState::Done));
    assert!(app.day.tasks[0].done_ymd.is_some());

    // Next day sweep moves it to history
    app.sweep_done_before(20250831);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
}

// Red: アクティブが存在しない場合でも選択行をfinishできる
#[test]
fn finish_selected_when_none_active() {
    let mut app = App::with_config(Config::default());
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-30");
    app.add_task("P", 5); // Planned by default
    assert_eq!(app.day.active_index(), None);

    // Finish via Shift+Enter mapping as well
    let ev = KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT);
    app.handle_key_event(ev);

    // Done but remains in Today
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.history_tasks().len(), 0);
    assert_eq!(app.day.tasks[0].title, "P");
    assert!(matches!(app.day.tasks[0].state, TaskState::Done));

    app.sweep_done_before(20250831);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
}
