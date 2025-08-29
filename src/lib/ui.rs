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

    // Split inner area: tabs on top, task list, help line at bottom (if space).
    // Use Min(0) for the list so rendering can gracefully degrade in tiny terminals.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
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
    if chunks.len() >= 2 && chunks[1].height > 0 {
        f.render_widget(para, chunks[1]);
    }

    // Help line at the bottom
    if chunks.len() >= 3 && chunks[2].height > 0 {
        let help = Paragraph::new(format_help_line_for(app)).style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }
}

// Tab metadata for the date views (Past/Today/Future).
// Returned as (titles, selected_index) to keep rendering logic decoupled for testing.
pub fn tab_titles(app: &App) -> (Vec<String>, usize) {
    let titles = vec!["Past".to_string(), "Today".to_string(), "Future".to_string()];
    let selected = match app.view() {
        View::Past => 0,
        View::Today => 1,
        View::Future => 2,
    };
    (titles, selected)
}

pub fn format_task_lines(app: &App) -> Vec<String> {
    format_task_lines_at(local_minutes(), app)
}

// Deterministic variant for tests: inject current minutes since midnight.
pub fn format_task_lines_at(now_min: u16, app: &App) -> Vec<String> {
    match app.view() {
        View::Past => render_list_slice(now_min, app, app.history_tasks()),
        View::Today => render_list_slice(now_min, app, &app.day.tasks),
        View::Future => render_list_slice(now_min, app, app.tomorrow_tasks()),
    }
}

fn render_list_slice(now_min: u16, app: &App, tasks: &[crate::task::Task]) -> Vec<String> {
    if tasks.is_empty() {
        return vec!["No tasks — press 'i' to add".to_string()];
    }
    let active_idx = app.day.active_index();

    // Build schedule start times from `now_min`, adding remaining durations of preceding tasks.
    let mut cursor = now_min;
    let starts: Vec<u16> = tasks
        .iter()
        .map(|t| {
            let this = cursor;
            // remaining minutes for this task (ignoring partial seconds for simplicity)
            let remaining = match t.state {
                TaskState::Done => 0,
                _ => t.estimate_min.saturating_sub(t.actual_min),
            };
            cursor = cursor.saturating_add(remaining);
            this
        })
        .collect();

    tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let sel = if i == app.selected_index() { "▶" } else { " " };
            let secs = if active_idx == Some(i) { app.active_carry_seconds() } else { 0 };
            let hh = (starts[i] / 60) % 24;
            let mm = starts[i] % 60;
            format!(
                "{:02}:{:02} {} {} {} (est:{}m act:{}m {}s)",
                hh,
                mm,
                sel,
                state_icon(t.state),
                t.title,
                t.estimate_min,
                t.actual_min,
                secs
            )
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

    let total_est_min: u32 = app.day.tasks.iter().map(|t| t.estimate_min as u32).sum();
    let total_act_min: u32 = app.day.tasks.iter().map(|t| t.actual_min as u32).sum();
    let carry_sec: u32 =
        if app.day.active_index().is_some() { app.active_carry_seconds() as u32 } else { 0 };

    let total_act_sec = total_act_min * 60 + carry_sec;
    let rem_total_sec = (total_est_min * 60).saturating_sub(total_act_sec);

    let rem_m = (rem_total_sec / 60) as u16;
    let rem_s = (rem_total_sec % 60) as u16;
    let act_m = (total_act_sec / 60) as u16;
    let act_s = (total_act_sec % 60) as u16;

    format!("ESD {:02}:{:02} | Est {}m {}s | Act {}m {}s", esd_h, esd_m, rem_m, rem_s, act_m, act_s)
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

// One-line help for common keys. Keep concise to fit narrow terminals.
pub fn format_help_line() -> String {
    // Group by intent: lifecycle, edit, navigate, quit.
    // Keep tokens like "Shift+Enter" and brackets to make tests and user scanning easy.
    "Enter: start/resume  Shift+Enter: finish  Space: pause  i: interrupt  e: +5m  [: up  ]: down  p: postpone  Tab: next  Shift+Tab: prev  q: quit".to_string()
}

// Contextual help: optimize content for current view.
pub fn format_help_line_for(app: &App) -> String {
    match app.view() {
        View::Today => {
            // Keep full actionable help on Today where operations are valid.
            format!(
                "{}  j/k,↑/↓: move",
                format_help_line()
            )
        }
        View::Future | View::Past => {
            // Limit to navigation/quit; lifecycle and edit ops are not applicable in these views.
            "j/k,↑/↓: move  Tab: next  Shift+Tab: prev  q: quit".to_string()
        }
    }
}
