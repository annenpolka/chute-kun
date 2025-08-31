use chute_kun::{app::App, ui};
use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn new_task_estimate_date_can_increment_with_mouse_buttons() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Enter);
    let area = Rect { x: 0, y: 0, width: 80, height: 16 };
    let popup = ui::compute_new_task_estimate_popup_rect(&app, area).expect("popup");
    let (_prev, _label, next) = ui::date_picker_hitboxes(&app, popup);
    let before = app.new_task_planned_ymd().unwrap();
    // Click next (▶)
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: next.x,
            row: next.y,
            modifiers: KeyModifiers::empty(),
        },
        area,
    );
    let after = app.new_task_planned_ymd().unwrap();
    assert!(after > before);
    // Recompute hitboxes because label width may change
    let (prev2, _label2, _next2) = ui::date_picker_hitboxes(&app, popup);
    // Click prev (◀)
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: prev2.x,
            row: prev2.y,
            modifiers: KeyModifiers::empty(),
        },
        area,
    );
    let back = app.new_task_planned_ymd().unwrap();
    assert_eq!(back, before);
}

#[test]
fn estimate_edit_date_can_change_with_mouse_buttons() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let mut app = App::new();
    app.add_task("A", 10);
    app.handle_key(KeyCode::Char('e'));
    let area = Rect { x: 0, y: 0, width: 80, height: 16 };
    let popup = ui::compute_estimate_popup_rect(&app, area).expect("popup");
    let (_prev, _label, next) = ui::date_picker_hitboxes(&app, popup);
    let before = app.day.tasks[0].planned_ymd;
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: next.x,
            row: next.y,
            modifiers: KeyModifiers::empty(),
        },
        area,
    );
    let after = app.day.tasks[0].planned_ymd;
    assert!(after > before);
}
