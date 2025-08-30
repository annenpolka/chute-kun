use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::layout::Rect;

#[test]
fn input_popup_add_and_cancel() {
    let mut app = App::new();
    // Open input (normal)
    app.handle_key(KeyCode::Char('i'));
    // Type nothing; clicking Add uses default title
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_input_popup_rect(&app, area).unwrap();
    let (add, _cancel) = ui::input_popup_button_hitboxes(&app, popup);
    app.handle_mouse_event(
        crossterm::event::MouseEvent {
            kind: crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left),
            column: add.x,
            row: add.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    assert!(!app.in_input_mode());
    assert_eq!(app.day.tasks.len(), 1);
}
