use chute_kun::ui;
use ratatui::layout::Rect;
use unicode_width::UnicodeWidthStr;

#[test]
fn header_action_buttons_hitboxes_align_right_with_expected_widths() {
    let area = Rect { x: 0, y: 0, width: 60, height: 10 };
    let boxes = ui::header_action_buttons_hitboxes(area);
    assert_eq!(boxes.len(), 5);
    // All on the top border row (y == 0)
    for r in &boxes {
        assert_eq!(r.y, 0);
        assert_eq!(r.height, 1);
    }
    // Widths equal to label lengths (short labels)
    let labels = ui::header_action_button_labels();
    assert_eq!(labels.len(), 5);
    for (i, s) in labels.iter().enumerate() {
        assert_eq!(boxes[i].width, UnicodeWidthStr::width(s.as_str()) as u16);
    }
    // Rightmost button (Delete) should end at or before the cell left of the right border
    let right_edge = area.x + area.width - 2; // leave room for the top-right corner
    let last = boxes.last().unwrap();
    assert!(last.x + last.width - 1 <= right_edge);
}
