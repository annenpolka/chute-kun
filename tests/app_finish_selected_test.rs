use chute_kun::app::App;
use chute_kun::config::Config;
use chute_kun::task::TaskState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// Red: 非アクティブでも選択行をfinishでき、Activeは維持される
#[test]
fn finish_selected_non_active_keeps_active_running() {
    let mut app = App::with_config(Config::default());
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

    // B should be removed from Today and moved to history as Done
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].title, "B");
    assert!(matches!(app.history_tasks()[0].state, TaskState::Done));

    // A should remain Active in Today
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.active_index(), Some(0));
    assert!(matches!(app.day.tasks[0].state, TaskState::Active));
}

// Red: 選択がActiveの場合は従来通りfinishして履歴へ移動
#[test]
fn finish_selected_active_behaves_like_finish_active() {
    let mut app = App::with_config(Config::default());
    app.add_task("A", 25);

    // Start A and keep selection at 0
    app.handle_key(KeyCode::Enter);
    assert_eq!(app.selected_index(), 0);
    assert_eq!(app.day.active_index(), Some(0));

    // Finish via 'f'
    let ev = KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty());
    app.handle_key_event(ev);

    assert_eq!(app.day.active_index(), None);
    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].title, "A");
    assert!(matches!(app.history_tasks()[0].state, TaskState::Done));
}

// Red: アクティブが存在しない場合でも選択行をfinishできる
#[test]
fn finish_selected_when_none_active() {
    let mut app = App::with_config(Config::default());
    app.add_task("P", 5); // Planned by default
    assert_eq!(app.day.active_index(), None);

    // Finish via Shift+Enter mapping as well
    let ev = KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT);
    app.handle_key_event(ev);

    assert_eq!(app.day.tasks.len(), 0);
    assert_eq!(app.history_tasks().len(), 1);
    assert_eq!(app.history_tasks()[0].title, "P");
    assert!(matches!(app.history_tasks()[0].state, TaskState::Done));
}

