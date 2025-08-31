use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
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
fn clicking_new_opens_input_mode() {
    let mut app = App::new();
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    app.handle_mouse_event(click_at(boxes[0]), area);
    assert!(app.is_text_input_mode());
}

#[test]
fn clicking_start_starts_selected_task() {
    let mut app = App::new();
    app.add_task("A", 25);
    app.add_task("B", 15);
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    // Start
    app.handle_mouse_event(click_at(boxes[1]), area);
    assert_eq!(app.day.active_index(), Some(0));
}

#[test]
fn clicking_stop_pauses_active_task() {
    let mut app = App::new();
    app.add_task("A", 25);
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    // Start then Stop
    app.handle_mouse_event(click_at(boxes[1]), area);
    assert_eq!(app.day.active_index(), Some(0));
    app.handle_mouse_event(click_at(boxes[2]), area);
    assert_eq!(app.day.active_index(), None);
}

#[test]
fn clicking_finish_finishes_selected_task() {
    let mut app = App::new();
    app.add_task("A", 10);
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    app.handle_mouse_event(click_at(boxes[3]), area);
    assert!(matches!(app.day.tasks[0].state, chute_kun::task::TaskState::Done));
}

#[test]
fn clicking_delete_opens_confirm_popup() {
    let mut app = App::new();
    app.add_task("A", 10);
    let area = Rect { x: 0, y: 0, width: 80, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    app.handle_mouse_event(click_at(boxes[4]), area);
    assert!(app.is_confirm_delete());
}
