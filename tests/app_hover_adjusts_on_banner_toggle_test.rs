use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

fn left_down_at(x: u16, y: u16) -> MouseEvent {
    MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: x,
        row: y,
        modifiers: crossterm::event::KeyModifiers::empty(),
    }
}

#[test]
fn hover_realigns_when_active_banner_toggles() {
    // Arrange: 2 tasks and a fixed terminal area
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };

    // Move mouse to the second row (index 1) before starting
    let (_tabs1, _banner1, list1, _help1) = ui::compute_layout(&app, area);
    let col = list1.x + 2;
    let row = list1.y + 1; // index 1
    app.handle_mouse_move(col, row, area);
    assert_eq!(app.hovered_index(), Some(1), "precondition: hovered=1 before toggling");

    // Act: double-click at the cursor location to start the task -> active banner appears
    app.handle_mouse_event(left_down_at(col, row), area);
    app.handle_mouse_event(left_down_at(col, row), area);

    // The list Y has shifted by +1 due to the banner; hover should realign to the same physical row
    let (_tabs2, _banner2, list2, _help2) = ui::compute_layout(&app, area);
    let expected_after_start = (row - list2.y) as usize;
    assert_eq!(
        app.hovered_index(),
        Some(expected_after_start),
        "hover should match pointer row after banner appears"
    );

    // Act: double-click again to pause -> banner disappears, list Y shifts back
    app.handle_mouse_event(left_down_at(col, row), area);
    app.handle_mouse_event(left_down_at(col, row), area);

    let (_tabs3, _banner3, list3, _help3) = ui::compute_layout(&app, area);
    let expected_after_pause = (row - list3.y) as usize;
    assert_eq!(
        app.hovered_index(),
        Some(expected_after_pause),
        "hover should match pointer row after banner disappears"
    );
}
