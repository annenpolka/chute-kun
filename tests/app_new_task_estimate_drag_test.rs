use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn dragging_on_new_task_estimate_slider_sets_value_and_confirms() {
    let mut app = App::new();
    // Open new task title input, accept default title
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Enter);
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_new_task_estimate_popup_rect(&app, area).expect("popup");
    let (track, _ok, _cancel) = ui::estimate_slider_hitboxes(&app, popup);
    // Drag to around 50m
    let to_x = ui::slider_x_for_minutes(track, 0, 240, 5, 50);
    app.handle_mouse_event(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: track.x, row: track.y, modifiers: crossterm::event::KeyModifiers::empty() },
        area,
    );
    app.handle_mouse_event(
        MouseEvent { kind: MouseEventKind::Drag(MouseButton::Left), column: to_x, row: track.y, modifiers: crossterm::event::KeyModifiers::empty() },
        area,
    );
    // Confirm via action button
    let (ok_btn, _cancel_btn) = ui::input_popup_button_hitboxes(&app, popup);
    app.handle_mouse_event(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column: ok_btn.x, row: ok_btn.y, modifiers: crossterm::event::KeyModifiers::empty() },
        area,
    );
    assert!(!app.in_input_mode());
    assert_eq!(app.day.tasks.len(), 1);
    assert_eq!(app.day.tasks[0].estimate_min, ui::minutes_from_slider_x(track, 0, 240, 5, to_x));
}

