use chute_kun::{app::App, config::Config, task::Category, ui};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{backend::TestBackend, layout::Rect, style::Color, Terminal};

#[test]
fn category_color_and_name_can_be_overridden_by_config() {
    // Custom config: rename Work to Client and color to cyan
    let toml = r#"
day_start = "09:00"

[categories.work]
name = "Client"
color = "cyan"
"#;
    let cfg = Config::from_toml_str(toml).expect("parse");
    let mut app = App::with_config(cfg);

    // Add one Work task and render list
    app.add_task("Deal", 15);
    app.day.tasks[0].category = Category::Work;

    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();
    let area = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let row_y = list.y + 1;
    // Check the dot color is cyan
    let dot_x = (list.x..list.x + list.width)
        .find(|&x| buf[(x, row_y)].symbol() == "●")
        .expect("did not find category dot");
    assert_eq!(buf[(dot_x, row_y)].style().fg, Some(Color::Cyan));

    // Open category picker via right-click on the dot and ensure label 'Client' appears
    app.handle_mouse_event(
        MouseEvent { kind: MouseEventKind::Down(MouseButton::Right), column: dot_x, row: row_y, modifiers: crossterm::event::KeyModifiers::empty() },
        area,
    );
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf2 = terminal.backend().buffer().clone();
    let mut found_label = false;
    for y in 0..buf2.area.height {
        let mut s = String::new();
        for x in 0..buf2.area.width { s.push_str(buf2[(x, y)].symbol()); }
        if s.contains("Client") { found_label = true; break; }
    }
    assert!(found_label, "picker should contain overridden category name");
}
