use chute_kun::{app::App, ui::header_title_line};
use ratatui::style::{Color, Modifier};

#[test]
fn header_title_line_has_colorful_pills() {
    let mut app = App::new();
    // Make the numbers deterministic but non‑zero
    app.add_task("A", 30);
    // Now is 09:00
    let line = header_title_line(9 * 60, &app);

    // Expected span indices based on construction order in header_title_line
    // 0: " ESD " (pill), 1: space, 2: time value, 3: sep,
    // 4: " Est " (pill), 5: space, 6: remaining value, 7: sep,
    // 8: " Act " (pill), 9: space, 10: actual value
    assert!(line.spans.len() >= 11);

    let esd_pill = &line.spans[0];
    assert_eq!(esd_pill.content.as_ref(), "ESD");
    assert_eq!(esd_pill.style.bg, Some(Color::Blue));
    assert_eq!(esd_pill.style.fg, Some(Color::Black));
    assert!(esd_pill.style.add_modifier.contains(Modifier::BOLD));

    let time_val = &line.spans[2];
    assert_eq!(time_val.style.fg, Some(Color::Cyan));
    assert!(time_val.style.add_modifier.contains(Modifier::BOLD));

    let est_pill = &line.spans[4];
    assert_eq!(est_pill.content.as_ref(), "Est");
    assert_eq!(est_pill.style.bg, Some(Color::Green));
    assert_eq!(est_pill.style.fg, Some(Color::Black));
    assert!(est_pill.style.add_modifier.contains(Modifier::BOLD));

    let act_pill = &line.spans[8];
    assert_eq!(act_pill.content.as_ref(), "Act");
    assert_eq!(act_pill.style.bg, Some(Color::Magenta));
    assert_eq!(act_pill.style.fg, Some(Color::Black));
    assert!(act_pill.style.add_modifier.contains(Modifier::BOLD));
}
