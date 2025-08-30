use chute_kun::{
    app::{App, View},
    ui,
};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

#[test]
fn clicking_inside_each_tab_label_switches_view() {
    let mut app = App::new();
    let area = Rect { x: 0, y: 0, width: 60, height: 10 };
    let (tabs, _banner, _list, _help) = ui::compute_layout(&app, area);
    let boxes = ui::tab_hitboxes(&app, tabs);
    assert_eq!(boxes.len(), 3);

    // Click inside each label box
    let click = |r: Rect| MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: r.x + r.width.saturating_sub(1), // rightmost cell of the label
        row: r.y,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    app.handle_mouse_event(click(boxes[0]), area);
    assert_eq!(app.view(), View::Past);

    app.handle_mouse_event(click(boxes[1]), area);
    assert_eq!(app.view(), View::Today);

    app.handle_mouse_event(click(boxes[2]), area);
    assert_eq!(app.view(), View::Future);
}
