use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn space_opens_start_time_slider_and_sets_fixed_time() {
    let mut app = App::new();
    app.add_task("A", 30);

    // Open Start Time slider with Space
    app.handle_key(KeyCode::Char(' '));
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_start_time_popup_rect(&app, area).expect("start-time popup");

    // Click on the slider at 10:00
    let (track, ok, _cancel) = ui::estimate_slider_hitboxes(&app, popup);
    let x = ui::slider_x_for_minutes(track, 0, 23 * 60 + 59, 5, 10 * 60);
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: x,
            row: track.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    // Confirm with OK
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: ok.x,
            row: ok.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    assert!(!app.is_start_time_edit());
    let expected = ui::minutes_from_slider_x(track, 0, 23 * 60 + 59, 5, x);
    assert_eq!(app.day.tasks[0].fixed_start_min, Some(expected));

    // Planned time reflects the fixed time set via slider
    let lines = ui::format_task_lines_at(9 * 60, &app);
    let hh = (expected / 60) % 24;
    let mm = expected % 60;
    let prefix = format!("{:02}:{:02} ", hh, mm);
    assert!(lines[0].starts_with(&prefix), "got: {}", lines[0]);
}
