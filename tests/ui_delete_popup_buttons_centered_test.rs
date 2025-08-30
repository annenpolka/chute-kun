use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::layout::Rect;

#[test]
fn delete_buttons_are_centered() {
    let mut app = App::new();
    app.add_task("Danger", 15);
    app.handle_key(KeyCode::Char('x'));
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_delete_popup_rect(&app, area).unwrap();
    let (del, cancel) = ui::delete_popup_button_hitboxes(&app, popup);
    let inner =
        Rect { x: popup.x + 1, y: popup.y + 1, width: popup.width - 2, height: popup.height - 2 };
    let total = del.width + (cancel.x - (del.x + del.width)) + cancel.width;
    let center = inner.x + inner.width / 2;
    let buttons_center = del.x + total / 2;
    assert!((buttons_center as i32 - center as i32).abs() <= 1);
}
