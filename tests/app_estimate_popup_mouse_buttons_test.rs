use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::layout::Rect;

#[test]
fn estimate_popup_plus_and_ok() {
    let mut app = App::new();
    app.add_task("A", 20);
    app.handle_key(KeyCode::Char('e'));
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_estimate_popup_rect(&app, area).unwrap();
    let (track, ok, _cancel) = ui::estimate_slider_hitboxes(&app, popup);
    // Click on the track position corresponding to 30m
    let target_x = ui::slider_x_for_minutes(track, 0, 240, 5, 30);
    let expected = ui::minutes_from_slider_x(track, 0, 240, 5, target_x);
    app.handle_mouse_event(
        crossterm::event::MouseEvent {
            kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: target_x,
            row: track.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    assert_eq!(app.selected_estimate(), Some(expected));
    // Click OK to close
    app.handle_mouse_event(
        crossterm::event::MouseEvent {
            kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: ok.x,
            row: ok.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    assert!(!app.is_estimate_editing());
}
