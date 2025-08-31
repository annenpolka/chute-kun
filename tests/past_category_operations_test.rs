use chute_kun::{
    app::App,
    task::{Category, Task, TaskState},
    ui,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

fn sample_done(title: &str) -> Task {
    let mut t = Task::new(title, 10);
    t.state = TaskState::Done;
    t.done_ymd = Some(chute_kun::date::today_ymd());
    t
}

// Red: Pastビューで 'c' によるカテゴリ切替と Shift+c ピッカー適用が動く
#[test]
fn past_cycle_and_picker_work() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    // Past に1件入ったスナップショットを適用
    app.apply_snapshot(vec![], vec![], vec![sample_done("P")]);
    app.handle_key(KeyCode::BackTab); // Today -> Past
    assert_eq!(app.history_tasks().len(), 1);
    assert!(matches!(app.history_tasks()[0].category, Category::General));

    // 1) 'c' でカテゴリ循環（General -> Work）
    app.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    assert!(matches!(app.history_tasks()[0].category, Category::Work));

    // 2) Shift+'c' でピッカーを開き、Down -> Down -> Enter で Hobby に設定
    app.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::SHIFT));
    assert!(app.is_category_picker(), "picker should open on Shift+c in Past");
    app.handle_key(KeyCode::Down); // Home
    app.handle_key(KeyCode::Down); // Hobby
    app.handle_key(KeyCode::Enter);
    assert!(matches!(app.history_tasks()[0].category, Category::Hobby));

    terminal.draw(|f| ui::draw(f, &app)).unwrap();
}

// Red: Pastビューでカテゴリドット右クリックによりピッカーが開く
#[test]
fn past_right_click_dot_opens_picker() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.apply_snapshot(vec![], vec![], vec![sample_done("Q")]);
    app.handle_key(KeyCode::BackTab); // Today -> Past

    // Render to find dot position in Past list
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let area = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let row_y = list.y + 1;
    let dot_x = (list.x..list.x + list.width)
        .find(|&x| buf[(x, row_y)].symbol() == "●")
        .expect("did not find category dot in Past");
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: dot_x,
            row: row_y,
            modifiers: KeyModifiers::empty(),
        },
        area,
    );
    assert!(app.is_category_picker(), "expected picker to open on right-click in Past");
}
