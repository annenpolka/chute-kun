use chute_kun::{app::App, ui};
use crossterm::event::{KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn right_click_opens_estimate_editor_on_row() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    let area = Rect { x: 0, y: 0, width: 60, height: 10 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Right click on second row
    let row_y = list.y + 1;
    let col_x = list.x + 2;
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: col_x,
            row: row_y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    assert!(app.is_estimate_editing(), "should enter estimate edit mode");
    assert_eq!(app.selected_index(), 1, "should select clicked row");

    // While in edit mode, Up increases by 5
    app.handle_key_event(KeyEvent {
        code: crossterm::event::KeyCode::Up,
        modifiers: crossterm::event::KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    assert_eq!(app.selected_estimate(), Some(25));
}
