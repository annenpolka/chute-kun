use chute_kun::{app::App, ui};
use ratatui::style::{Color, Modifier};

#[test]
fn header_action_buttons_line_contains_five_buttons_with_styles() {
    let app = App::new();
    let line = ui::header_action_buttons_line(&app);

    // Labels are short: "New", "Start", "Stop", "Finish", "Delete"
    let find = |name: &str| line.spans.iter().find(|s| s.content.as_ref() == name).unwrap().clone();

    let new = find("New");
    assert!(new.style.add_modifier.contains(Modifier::BOLD));
    assert_eq!(new.style.fg, Some(Color::Black));
    assert_eq!(new.style.bg, Some(Color::Green));

    // With no tasks, Start/Stop/Finish/Delete should be disabled (grayed out)
    for name in ["Start", "Stop", "Finish", "Delete"] {
        let span = find(name);
        assert!(!span.style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(span.style.fg, Some(Color::DarkGray));
        assert_eq!(span.style.bg, None);
    }
}
