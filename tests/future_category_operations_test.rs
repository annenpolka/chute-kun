use chute_kun::{app::App, task::Category, ui};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

// Red: Futureビューで 'c' によるカテゴリ切替と Shift+c ピッカー適用が動く
#[test]
fn future_cycle_and_picker_work() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 10);
    // A を Future へ
    app.handle_key(KeyCode::Char('p')); // postpone to tomorrow
                                        // Future ビューへ
    app.handle_key(KeyCode::Tab);
    assert_eq!(app.tomorrow_tasks().len(), 1);
    assert!(matches!(app.tomorrow_tasks()[0].category, Category::General));

    // 1) 'c' でカテゴリが循環（General -> Work）
    app.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    assert!(matches!(app.tomorrow_tasks()[0].category, Category::Work));

    // 2) Shift+'c' でピッカーを開き、Down -> Enter で Home に設定
    app.handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::SHIFT));
    assert!(app.is_category_picker(), "picker should open on Shift+c in Future");
    app.handle_key(KeyCode::Down); // select Home (index 2) from Work's index(1)
    app.handle_key(KeyCode::Enter);
    assert!(matches!(app.tomorrow_tasks()[0].category, Category::Home));

    // 描画パスも問題ないことを確認
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
}

// Red: Futureビューでカテゴリドット右クリックによりピッカーが開く
#[test]
fn future_right_click_dot_opens_picker() {
    let backend = TestBackend::new(60, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("B", 10);
    app.handle_key(KeyCode::Char('p')); // postpone to tomorrow
    app.handle_key(KeyCode::Tab); // Future

    // Render to find the dot position in Future list
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let area = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let row_y = list.y + 1;
    let dot_x = (list.x..list.x + list.width)
        .find(|&x| buf[(x, row_y)].symbol() == "●")
        .expect("did not find category dot in Future");
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: dot_x,
            row: row_y,
            modifiers: KeyModifiers::empty(),
        },
        area,
    );
    assert!(app.is_category_picker(), "expected picker to open on right-click in Future");
}
