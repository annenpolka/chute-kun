use chute_kun::{app::App, task::Session, ui};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};

fn count_titles_on_row(buf: &ratatui::buffer::Buffer, y: u16, initials: &[char]) -> usize {
    let mut s = String::new();
    for x in 0..buf.area.width { s.push_str(buf[(x, y)].symbol()); }
    initials.iter().filter(|&&ch| s.contains(ch)).count()
}

#[test]
fn calendar_limits_visible_columns_to_three() {
    let backend = TestBackend::new(80, 18);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    let base = app.config.day_start_minutes;

    // Four overlapping sessions starting at the same minute (A,B,C,D)
    let titles = ["Alpha", "Bravo", "Charlie", "Delta"];
    for t in titles { app.add_task(t, 60); }
    for i in 0..4 {
        if let Some(t) = app.day.tasks.get_mut(i) {
            t.sessions.push(Session { start_min: base + 10, end_min: Some(base + 30) });
        }
    }
    app.toggle_display_mode();

    // Draw later; all sessions are closed
    struct Cl(u16); impl chute_kun::clock::Clock for Cl { fn now_minutes(&self)->u16{self.0} }
    let clock = Cl(base + 100);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();
    let buf = terminal.backend().buffer().clone();

    let area = Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);

    // Find the row with any of the titles (their start row)
    let mut start_row = None;
    for y in list.y..list.y + list.height {
        let c = count_titles_on_row(&buf, y, &['A','B','C','D']);
        if c > 0 { start_row = Some(y); break; }
    }
    let y = start_row.expect("start row not found");
    // Should show at most 3 distinct initials among A,B,C,D
    let c = count_titles_on_row(&buf, y, &['A','B','C','D']);
    assert!(c <= 3, "expected at most 3 titles on the row, got {}", c);
}

