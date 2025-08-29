use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::{App, View};
use crate::task::TaskState;

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.size();
    let header = format_header_line(local_minutes(), app);
    let block = Block::default().title(header).borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split inner area: tabs on top, task list below
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Tabs for date views
    let (titles, selected) = tab_titles(app);
    let titles: Vec<Line> = titles.into_iter().map(Line::from).collect();
    let tabs = Tabs::new(titles)
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("│"));
    f.render_widget(tabs, chunks[0]);

    // Main list content
    let lines = format_task_lines(app).join("\n");
    let para = Paragraph::new(lines);
    f.render_widget(para, chunks[1]);
}

// Tab metadata for the date views (Past/Today/Future).
// Returned as (titles, selected_index) to keep rendering logic decoupled for testing.
pub fn tab_titles(app: &App) -> (Vec<String>, usize) {
    let titles = vec!["Past".to_string(), "Today".to_string(), "Future".to_string()];
    let selected = match app.view() { View::Past => 0, View::Today => 1, View::Future => 2 };
    (titles, selected)
}

pub fn format_task_lines(app: &App) -> Vec<String> {
    match app.view() {
        View::Past => render_list_slice(app, app.history_tasks()),
        View::Today => render_list_slice(app, &app.day.tasks),
        View::Future => render_list_slice(app, app.tomorrow_tasks()),
    }
}

fn render_list_slice(app: &App, tasks: &Vec<crate::task::Task>) -> Vec<String> {
    if tasks.is_empty() {
        return vec!["No tasks — press 'i' to add".to_string()];
    }
    tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let sel = if i == app.selected_index() { "▶" } else { " " };
            format!("{} {} {} (est:{}m act:{}m)", sel, state_icon(t.state), t.title, t.estimate_min, t.actual_min)
        })
        .collect()
}

fn state_icon(state: TaskState) -> &'static str {
    match state {
        TaskState::Planned => " ",
        TaskState::Active => ">",
        TaskState::Paused => "=",
        TaskState::Done => "x",
    }
}

pub fn format_header_line(now_min: u16, app: &App) -> String {
    let _remaining = app.day.remaining_total_min();
    let esd_min = app.day.esd(now_min);
    let esd_h = esd_min / 60;
    let esd_m = esd_min % 60;
    let total_est: u16 = app.day.tasks.iter().map(|t| t.estimate_min).sum();
    let total_act: u16 = app.day.tasks.iter().map(|t| t.actual_min).sum();
    format!("ESD {:02}:{:02} | Est {}m | Act {}m", esd_h, esd_m, total_est - total_act, total_act)
}

fn local_minutes() -> u16 {
    // Best-effort minutes since UTC midnight; good enough for on-screen ESD.
    use std::time::{SystemTime, UNIX_EPOCH};
    if let Ok(dur) = SystemTime::now().duration_since(UNIX_EPOCH) {
        let minutes = (dur.as_secs() % 86_400) / 60;
        minutes as u16
    } else {
        9 * 60
    }
}
