use chute_kun::{app::App, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

// Red: dragging a list row downward should reorder tasks on mouse up.
#[test]
fn mouse_drag_moves_task_down_on_release() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);
    app.add_task("D", 40);

    let area = Rect { x: 0, y: 0, width: 60, height: 16 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let col_x = list.x + 2;
    let row_of = |i: u16| list.y + 1 + i; // header at list.y

    // Start drag on B (index 1)
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: col_x,
            row: row_of(1),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    // Drag over D (index 3)
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: col_x,
            row: row_of(3),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    // Release to drop at D's row
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: col_x,
            row: row_of(3),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    let titles: Vec<_> = app.day.tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["A", "C", "D", "B"], "B should be dropped after D");
    assert_eq!(app.selected_index(), 3, "selection should follow the moved task");
}

// Red: dragging upward should insert before the target row.
#[test]
fn mouse_drag_moves_task_up_on_release() {
    let mut app = App::new();
    app.add_task("A", 10);
    app.add_task("B", 20);
    app.add_task("C", 30);
    app.add_task("D", 40);

    let area = Rect { x: 0, y: 0, width: 60, height: 16 };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let col_x = list.x + 2;
    let row_of = |i: u16| list.y + 1 + i; // header at list.y

    // Start drag on C (index 2)
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: col_x,
            row: row_of(2),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    // Drag over A (index 0)
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: col_x,
            row: row_of(0),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );
    // Release to drop before A
    app.handle_mouse_event(
        MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: col_x,
            row: row_of(0),
            modifiers: crossterm::event::KeyModifiers::empty(),
        },
        area,
    );

    let titles: Vec<_> = app.day.tasks.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["C", "A", "B", "D"], "C should be inserted before A");
    assert_eq!(app.selected_index(), 0, "selection should move with C to index 0");
}
