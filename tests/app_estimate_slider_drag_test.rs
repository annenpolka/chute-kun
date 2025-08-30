use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn dragging_on_estimate_slider_updates_value() {
    let mut app = App::new();
    app.add_task("A", 20);
    // Open estimate editor
    app.handle_key(crossterm::event::KeyCode::Char('e'));
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_estimate_popup_rect(&app, area).expect("popup");
    let (track, ok, _cancel) = ui::estimate_slider_hitboxes(&app, popup);
    // Start drag on track then move to target
    let start_x = ui::slider_x_for_minutes(track, 0, 240, 5, 10);
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: start_x,
            row: track.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    let target_x = ui::slider_x_for_minutes(track, 0, 240, 5, 45);
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: target_x,
            row: track.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    let expected = ui::minutes_from_slider_x(track, 0, 240, 5, target_x);
    assert_eq!(app.selected_estimate(), Some(expected));
    // Close with OK
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: ok.x,
            row: ok.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    assert!(!app.is_estimate_editing());
}
