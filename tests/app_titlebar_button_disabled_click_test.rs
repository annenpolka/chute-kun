use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

fn click_at(r: ratatui::layout::Rect) -> MouseEvent {
    MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: r.x + r.width.saturating_sub(1),
        row: r.y,
        modifiers: crossterm::event::KeyModifiers::empty(),
    }
}

#[test]
fn disabled_start_does_not_start_when_no_tasks() {
    let mut app = App::new();
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    // Start is disabled without tasks; clicking should not panic or start
    app.handle_mouse_event(click_at(boxes[1]), area);
    assert_eq!(app.day.active_index(), None);
}

#[test]
fn disabled_stop_does_nothing_when_no_active() {
    let mut app = App::new();
    app.add_task("A", 20); // not active yet
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    // Stop is disabled; clicking should not change state
    app.handle_mouse_event(click_at(boxes[2]), area);
    assert_eq!(app.day.active_index(), None);
}

#[test]
fn delete_click_is_ignored_on_future_view() {
    let mut app = App::new();
    app.add_task("A", 10);
    // Switch Today -> Future
    app.handle_key(KeyCode::Tab);
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    app.handle_mouse_event(click_at(boxes[4]), area);
    assert!(!app.is_confirm_delete());
}
