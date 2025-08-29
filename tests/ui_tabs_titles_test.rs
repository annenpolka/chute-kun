use chute_kun::{
    app::App,
    ui::tab_titles,
};

// Red: specify expected tab labels and selection mapping for views.
#[test]
fn tab_titles_and_selected_index_follow_view() {
    let mut app = App::new();

    // Default view is Today
    let (titles, sel) = tab_titles(&app);
    assert_eq!(titles, vec!["Past".to_string(), "Today".to_string(), "Future".to_string()]);
    assert_eq!(sel, 1);

    // Move to Past
    // There is no direct setter, so cycle with BackTab from Today
    app.handle_key(crossterm::event::KeyCode::BackTab);
    let (_, sel) = tab_titles(&app);
    assert_eq!(sel, 0);

    // Move to Future
    app.handle_key(crossterm::event::KeyCode::Tab);
    app.handle_key(crossterm::event::KeyCode::Tab);
    let (_, sel) = tab_titles(&app);
    assert_eq!(sel, 2);
}
