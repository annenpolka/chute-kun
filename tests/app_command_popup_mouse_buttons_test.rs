use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn command_popup_run_click_executes_command_and_closes() {
    let mut app = App::new();
    app.add_task("A", 20);
    let area = Rect { x: 0, y: 0, width: 80, height: 20 };

    // Open command palette and type a command
    app.handle_key(KeyCode::Char(':'));
    for c in "est +15m".chars() {
        app.handle_key(KeyCode::Char(c));
    }

    let popup = ui::compute_command_popup_rect(&app, area).expect("command popup rect");
    let (run_btn, _cancel_btn) = ui::command_popup_button_hitboxes(&app, popup);

    // Click Run
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: run_btn.x + 1,
            row: run_btn.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    // Should close popup and apply command
    assert!(!app.is_command_mode(), "run click should close command popup");
    assert_eq!(app.day.tasks[0].estimate_min, 35);
}

#[test]
fn command_popup_cancel_click_closes_without_running() {
    let mut app = App::new();
    app.add_task("A", 20);
    let area = Rect { x: 0, y: 0, width: 80, height: 20 };

    app.handle_key(KeyCode::Char(':'));
    for c in "est +15m".chars() {
        app.handle_key(KeyCode::Char(c));
    }

    let popup = ui::compute_command_popup_rect(&app, area).expect("command popup rect");
    let (_run_btn, cancel_btn) = ui::command_popup_button_hitboxes(&app, popup);

    // Click Cancel
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: cancel_btn.x + 1,
            row: cancel_btn.y,
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    // Should close popup and not apply command
    assert!(!app.is_command_mode(), "cancel click should close command popup");
    assert_eq!(app.day.tasks[0].estimate_min, 20);
}
