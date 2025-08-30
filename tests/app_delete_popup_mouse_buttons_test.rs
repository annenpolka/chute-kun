use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn popup_buttons_cancel_and_delete_via_mouse() {
    let mut app = App::new();
    app.add_task("Danger", 15);
    app.add_task("Keep", 10);
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };

    // Open confirm
    app.handle_key(KeyCode::Char('x'));
    assert!(app.is_confirm_delete());
    let popup = ui::compute_delete_popup_rect(&app, area).expect("popup rect");
    let (_del_btn, cancel_btn) = ui::delete_popup_button_hitboxes(&app, popup);

    // Click Cancel -> should close popup and not delete
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: cancel_btn.x + 1,
            row: cancel_btn.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    assert!(!app.is_confirm_delete(), "cancel should close popup");
    assert_eq!(app.day.tasks.len(), 2, "no deletion on cancel");

    // Re-open and click Delete
    app.handle_key(KeyCode::Char('x'));
    let popup = ui::compute_delete_popup_rect(&app, area).unwrap();
    let (del_btn, _cancel_btn) = ui::delete_popup_button_hitboxes(&app, popup);
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: del_btn.x + 1,
            row: del_btn.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    assert!(!app.is_confirm_delete(), "delete should close popup");
    assert_eq!(app.day.tasks.len(), 1, "one item should be deleted");
}
