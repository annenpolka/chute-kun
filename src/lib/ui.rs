use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::{App, View};
use crate::clock::Clock;
use crate::task::TaskState;

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.size();
    let header = format_header_line(app_display_base(app), app);
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

    // Main content: input prompt when in input mode; otherwise task list.
    // Render as styled lines so we can highlight the selected row.
    let content_lines = if app.in_input_mode() {
        let buf = app.input_buffer().unwrap_or("");
        vec![format!("Input: {} _  (Enter=Add Esc=Cancel)", buf)]
    } else {
        format_task_lines(app)
    };
    let mut styled: Vec<Line> = content_lines.into_iter().map(Line::from).collect();
    if !app.in_input_mode() {
        // Only highlight when showing task lists and there is at least one task in the current view.
        let cur_len = match app.view() {
            View::Past => app.history_tasks().len(),
            View::Today => app.day.tasks.len(),
            View::Future => app.tomorrow_tasks().len(),
        };
        if cur_len > 0 {
            let idx = app.selected_index().min(styled.len().saturating_sub(1));
            if let Some(line) = styled.get_mut(idx) {
                line.style = Style::default().bg(Color::Blue);
            }
        }
    }
    let para = Paragraph::new(styled);
    f.render_widget(para, chunks[1]);

    // Help line at the bottom
    if chunks.len() >= 3 && chunks[2].height > 0 {
        let help =
            Paragraph::new(format_help_line_for(app)).style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }
}

/// Like `draw`, but uses an injected `Clock` for current time.
pub fn draw_with_clock(f: &mut Frame, app: &App, clock: &dyn Clock) {
    let area: Rect = f.size();
    let now = clock.now_minutes();
    let header = format_header_line(now, app);
    let block = Block::default().title(header).borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Keep layout consistent with `draw`: tabs, content, help line
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let (titles, selected) = tab_titles(app);
    let titles: Vec<Line> = titles.into_iter().map(Line::from).collect();
    let tabs = Tabs::new(titles)
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("│"));
    f.render_widget(tabs, chunks[0]);

    // Content with injected clock, using styled lines for selection highlight
    let mut styled: Vec<Line> =
        format_task_lines_at(now, app).into_iter().map(Line::from).collect();
    let cur_len = match app.view() {
        View::Past => app.history_tasks().len(),
        View::Today => app.day.tasks.len(),
        View::Future => app.tomorrow_tasks().len(),
    };
    if cur_len > 0 {
        let idx = app.selected_index().min(styled.len().saturating_sub(1));
        if let Some(line) = styled.get_mut(idx) {
            line.style = Style::default().bg(Color::Blue);
        }
    }
    let para = Paragraph::new(styled);
    if chunks.len() >= 2 && chunks[1].height > 0 {
        f.render_widget(para, chunks[1]);
    }

    // Help line at the bottom
    if chunks.len() >= 3 && chunks[2].height > 0 {
        let help =
            Paragraph::new(format_help_line_for(app)).style(Style::default().fg(Color::DarkGray));
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
    format_task_lines_at(app_display_base(app), app)
}

// Deterministic variant for tests: inject current minutes since midnight.
pub fn format_task_lines_at(now_min: u16, app: &App) -> Vec<String> {
    if app.in_input_mode() {
        let buf = app.input_buffer().unwrap_or("");
        return vec![format!("Input: {} _  (Enter=Add Esc=Cancel)", buf)];
    }
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

// Local time retrieval moved to `crate::clock`.

/// Generic keyboard help string (superset). Used by tests and as fallback.
pub fn format_help_line() -> String {
    // Keep wording substrings aligned with tests
    // - quitting / navigation
    let nav = "q: quit | Tab: switch view";
    // - task lifecycle and operations (Today view only in optimized variant)
    let task =
        "Enter: start/pause | Shift+Enter/f: finish | Space: pause | i: interrupt | p: postpone | [: up | ]: down | e: +5m";
    format!("{} | {}", nav, task)
}

/// Optimized help depending on the current view.
/// - Today: show full task actions
/// - Past/Future: show only navigation and quit to reduce noise
pub fn format_help_line_for(app: &App) -> String {
    match app.view() {
        View::Today => format_help_line(),
        View::Past | View::Future => "q: quit | Tab: switch view".to_string(),
    }
}

fn app_display_base(app: &App) -> u16 { app.config.day_start_minutes }
