use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
};
use unicode_width::UnicodeWidthStr;

use crate::app::{App, View};
use crate::clock::Clock;
use crate::task::TaskState;

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.size();
    let header = format_header_line(app_display_base(app), app);
    let block = Block::default().title(header).borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Pre-compute wrapped help lines for current width, to size the layout.
    let help_lines = help_lines_for_width(app, inner.width.max(1));
    let help_height = help_lines.len() as u16; // 1+ lines depending on width

    // Split inner area: tabs on top, task list, help block at bottom (if space).
    // Use Min(0) for the list so rendering can gracefully degrade in tiny terminals.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(help_height.max(1)),
        ])
        .split(inner);

    // Tabs for date views
    let (titles, selected) = tab_titles(app);
    let titles: Vec<Line> = titles.into_iter().map(Line::from).collect();
    let tabs = Tabs::new(titles)
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("│"));
    f.render_widget(tabs, chunks[0]);

    // Main content: if in stepper, render a styled line with highlighted minutes
    if app.is_estimate_editing() {
        let est = app.selected_estimate().unwrap_or(0);
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let mut line = Line::default();
        line.spans.push(Span::raw("Estimate: "));
        line.spans.push(Span::styled(
            format!("{}m", est),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ));
        if !title.is_empty() {
            line.spans.push(Span::raw(" — "));
            line.spans.push(Span::styled(title.to_string(), Style::default().fg(Color::Cyan)));
        }
        line.spans.push(Span::raw("  (+/-5m or j/k, Enter=OK Esc=Cancel)"));
        let para = Paragraph::new(line);
        f.render_widget(para, chunks[1]);
    } else {
        let lines = format_task_lines(app).join("\n");
        let para = Paragraph::new(lines);
        f.render_widget(para, chunks[1]);
    }

    // Help block at the bottom (wrapped to fit width)
    if chunks.len() >= 3 && chunks[2].height > 0 {
        let help_text = help_lines.join("\n");
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
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

    // Keep layout consistent with `draw`: tabs, content, help block
    let help_lines = help_lines_for_width(app, inner.width.max(1));
    let help_height = help_lines.len() as u16;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(help_height.max(1)),
        ])
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

    // Help block at the bottom (wrapped to fit width)
    if chunks.len() >= 3 && chunks[2].height > 0 {
        let help_text = help_lines.join("\n");
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
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
        // Estimate edit mode shows explicit stepper line with task title
        if app.is_estimate_editing() {
            let est = app.selected_estimate().unwrap_or(0);
            let title =
                app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
            let suffix = if title.is_empty() { "".to_string() } else { format!(" — {}", title) };
            return vec![format!("Estimate: {}m{}  (+/-5m, Enter=OK Esc=Cancel)", est, suffix)];
        }
        // Delete confirmation prompt
        if app.is_confirm_delete() {
            let title = app
                .day
                .tasks
                .get(app.selected_index())
                .map(|t| t.title.as_str())
                .unwrap_or("");
            let suffix = if title.is_empty() { "".to_string() } else { format!(" — {}", title) };
            return vec![format!("Delete?{}  (Enter=Delete Esc=Cancel)", suffix)];
        }
        // Command palette prompt (+ show target task title)
        if app.is_command_mode() {
            let title =
                app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
            let suffix = if title.is_empty() { "".to_string() } else { format!(" — {}", title) };
            return vec![format!("Command: {} _{}  (Enter=Run Esc=Cancel)", buf, suffix)];
        }
        // Fallback: normal input mode for adding a task
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
    // active index not needed for seconds rendering anymore (per-task seconds)

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
            let secs = match t.state {
                TaskState::Active | TaskState::Paused => t.actual_carry_sec,
                _ => 0,
            };
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
    // 正攻法: 全タスクの部分秒を合算
    let carry_sec: u32 = app.day.tasks.iter().map(|t| t.actual_carry_sec as u32).sum();

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
        "Enter: start/pause | Shift+Enter/f: finish | Space: pause | i: interrupt | p: postpone | x: delete | b: bring | [: up | ]: down | e: edit | j/k";
    format!("{} | {}", nav, task)
}

/// Optimized help depending on the current view.
/// - Today: show full task actions
/// - Past/Future: show only navigation and quit to reduce noise
pub fn format_help_line_for(app: &App) -> String {
    match app.view() {
        View::Today => format_help_line(),
        View::Past => "q: quit | Tab: switch view".to_string(),
        View::Future => "q: quit | Tab: switch view | b: bring".to_string(),
    }
}

/// Build help items depending on the current view. Used for wrapping.
pub fn help_items_for(app: &App) -> Vec<&'static str> {
    let mut items: Vec<&'static str> = vec!["q: quit", "Tab: switch view"];
    if matches!(app.view(), View::Today) {
        items.extend([
            "Enter: start/resume",
            "Space: pause",
            "Shift+Enter/f: finish",
            "i: interrupt",
            "p: postpone",
            "x: delete",
            "[: up",
            "]: down",
            "e: edit",
            "j/k",
        ]);
    }
    items
}

/// Wrap help items into lines that fit within `width` cells, inserting ` | ` between items.
/// This uses Unicode width to count display cells.
pub fn wrap_help_items_to_width(items: &[&str], width: u16) -> Vec<String> {
    let width = width as usize;
    if width == 0 {
        return vec![String::new()];
    }
    let mut lines: Vec<String> = Vec::new();
    let mut cur = String::new();
    let sep = " | ";
    for item in items {
        if cur.is_empty() {
            cur.push_str(item);
            continue;
        }
        let candidate = format!("{}{}{}", cur, sep, item);
        if UnicodeWidthStr::width(candidate.as_str()) <= width {
            cur = candidate;
        } else {
            // commit current line and start a new one
            lines.push(cur);
            cur = (*item).to_string();
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Convenience: get wrapped help lines for current app state and width.
pub fn help_lines_for_width(app: &App, width: u16) -> Vec<String> {
    let items = help_items_for(app);
    wrap_help_items_to_width(&items, width)
}

fn app_display_base(app: &App) -> u16 {
    app.config.day_start_minutes
}
