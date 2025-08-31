use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use unicode_width::UnicodeWidthStr;

#[test]
fn estimate_edit_popup_includes_date_line_and_fits_width() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let mut app = App::new();
    app.add_task("Title", 25);
    app.handle_key(KeyCode::Char('e'));

    let area = Rect { x: 0, y: 0, width: 80, height: 24 };
    let popup = ui::compute_estimate_popup_rect(&app, area).expect("popup");

    // Expect inner width to be >= both message widths
    let inner_w = popup.width.saturating_sub(2);
    let msg = format!(
        "Estimate: {}m — {}",
        app.selected_estimate().unwrap_or(0),
        app.day.tasks[app.selected_index()].title
    );
    let date_need = UnicodeWidthStr::width("Date: 2099-12-31 (Wed)").max(
        UnicodeWidthStr::width("Date: Tomorrow (Wed)")
            .max(UnicodeWidthStr::width("Date: Today (Wed)")),
    ) as u16;
    let need = (UnicodeWidthStr::width(msg.as_str()) as u16).max(date_need).saturating_add(0);
    assert!(inner_w >= need, "inner width {} must fit {}", inner_w, need);
}

#[test]
fn new_task_estimate_popup_includes_date_line_and_fits_width() {
    std::env::set_var("CHUTE_KUN_TODAY", "2025-08-31");
    let mut app = App::new();
    app.handle_key(KeyCode::Char('i'));
    app.handle_key(KeyCode::Char('A'));
    app.handle_key(KeyCode::Enter);
    assert!(app.is_new_task_estimate());
    let area = Rect { x: 0, y: 0, width: 60, height: 12 };
    let popup = ui::compute_new_task_estimate_popup_rect(&app, area).expect("popup");
    let inner_w = popup.width.saturating_sub(2);
    let msg = "Estimate: 25m — A"; // default
    let date_need = UnicodeWidthStr::width("Date: 2099-12-31 (Wed)").max(
        UnicodeWidthStr::width("Date: Tomorrow (Wed)")
            .max(UnicodeWidthStr::width("Date: Today (Wed)")),
    ) as u16;
    let need = (UnicodeWidthStr::width(msg) as u16).max(date_need).saturating_add(0);
    assert!(inner_w >= need, "inner width {} must fit {}", inner_w, need);
}
