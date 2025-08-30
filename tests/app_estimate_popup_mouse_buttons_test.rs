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
    let (_minus, plus, ok, _cancel) = ui::estimate_popup_button_hitboxes(&app, popup);
    // Click +5m twice
    for _ in 0..2 {
        app.handle_mouse_event(
            crossterm::event::MouseEvent {
                kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
                column: plus.x,
                row: plus.y,
                modifiers: crossterm::event::KeyModifiers::empty(),
            },
            area,
        );
    }
    assert_eq!(app.selected_estimate(), Some(30));
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
