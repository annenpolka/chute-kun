use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
};
use unicode_width::UnicodeWidthStr;

use crate::app::{App, View};
use crate::clock::Clock;
use crate::task::TaskState;

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.area();
    let header_line = header_title_line(app_display_base(app), app);
    let block = Block::default().title(header_line).borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Pre-compute wrapped help lines for current width, to size the layout.
    let help_lines = help_lines_for_width(app, inner.width.max(1));
    // Clamp help height so the task list keeps at least 1 row visible
    let mut help_height = help_lines.len() as u16; // 1+ lines depending on width
    let max_help = inner.height.saturating_sub(2); // tabs(1) + list(>=1)
    if max_help > 0 {
        help_height = help_height.min(max_help);
    }
    help_height = help_height.max(1);

    // Optional active-task banner just under the tabs
    let active_banner = format_active_banner(app);

    // Split inner area: tabs, optional banner, task list, help block.
    // Use Min(0) for the list so rendering can gracefully degrade in tiny terminals.
    let mut constraints: Vec<Constraint> = vec![Constraint::Length(1)];
    if active_banner.is_some() {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0));
    constraints.push(Constraint::Length(help_height.max(1)));
    let chunks =
        Layout::default().direction(Direction::Vertical).constraints(constraints).split(inner);

    // Tabs for date views
    let (titles, selected) = tab_titles(app);
    let titles: Vec<Line> = titles.into_iter().map(Line::from).collect();
    let tabs = Tabs::new(titles)
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("│"));
    f.render_widget(tabs, chunks[0]);

    // If we have an active banner, render it right below tabs
    let mut content_idx = 1usize; // index into chunks for main content
    if let Some(line) = active_banner {
        let para = Paragraph::new(line);
        f.render_widget(para, chunks[1]);
        content_idx = 2;
    }

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
        f.render_widget(para, chunks[content_idx]);
    } else if app.in_input_mode() {
        // In input/command modes, render plain paragraph without row highlight
        let lines = format_task_lines(app).join("\n");
        let para = Paragraph::new(lines);
        f.render_widget(para, chunks[content_idx]);
    } else {
        // Normal list rendering with selected-row background highlight
        let content_lines = format_task_lines(app);
        let mut styled: Vec<Line> = content_lines.into_iter().map(Line::from).collect();
        let cur_len = match app.view() {
            View::Past => app.history_tasks().len(),
            View::Today => app.day.tasks.len(),
            View::Future => app.tomorrow_tasks().len(),
        };
        if cur_len > 0 {
            let idx = app.selected_index().min(styled.len().saturating_sub(1));
            if let Some(line) = styled.get_mut(idx) {
                // Apply background to the whole line; also set each span's bg to be safe.
                let blue_bg = Style::default().bg(Color::Blue);
                line.style = blue_bg;
                for span in line.spans.iter_mut() {
                    span.style = span.style.patch(blue_bg);
                }
            }
        }
        let para = Paragraph::new(styled);
        f.render_widget(para, chunks[content_idx]);
    }

    // Help block at the bottom (wrapped to fit width)
    // When an active banner is present, help resides at the last chunk, not index 2.
    let help_idx = chunks.len().saturating_sub(1);
    if chunks[help_idx].height > 0 {
        let help_text = help_lines.join("\n");
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
        f.render_widget(help, chunks[help_idx]);
    }

    // Overlay: centered delete confirmation popup with colored text
    if app.is_confirm_delete() {
        // Compute message and popup size within the inner area
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let msg = format!("Delete? — {}  (Enter=Delete Esc=Cancel)", title);
        let content_w = UnicodeWidthStr::width(msg.as_str()) as u16;
        let popup_w = content_w.saturating_add(4).min(inner.width).max(20).min(inner.width);
        let popup_h: u16 = 3;
        let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
        let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
        let popup = Rect { x: px, y: py, width: popup_w, height: popup_h };

        let border_style = Style::default().fg(Color::Red);
        let title =
            Line::from(Span::styled(" Confirm ", border_style.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title).border_style(border_style);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner_popup = block.inner(popup);
        let para = Paragraph::new(Span::styled(msg, Style::default().fg(Color::Red)));
        f.render_widget(para, inner_popup);
    }
}

/// Like `draw`, but uses an injected `Clock` for current time.
pub fn draw_with_clock(f: &mut Frame, app: &App, clock: &dyn Clock) {
    let area: Rect = f.area();
    let now = clock.now_minutes();
    let header_line = header_title_line(now, app);
    let block = Block::default().title(header_line).borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Keep layout consistent with `draw`: tabs, content, help block
    let help_lines = help_lines_for_width(app, inner.width.max(1));
    // Clamp help height so the task list keeps at least 1 row visible
    let mut help_height = help_lines.len() as u16;
    let max_help = inner.height.saturating_sub(2);
    if max_help > 0 {
        help_height = help_height.min(max_help);
    }
    help_height = help_height.max(1);
    let active_banner = format_active_banner(app);
    let mut constraints: Vec<Constraint> = vec![Constraint::Length(1)];
    if active_banner.is_some() {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0));
    constraints.push(Constraint::Length(help_height.max(1)));
    let chunks =
        Layout::default().direction(Direction::Vertical).constraints(constraints).split(inner);

    let (titles, selected) = tab_titles(app);
    let titles: Vec<Line> = titles.into_iter().map(Line::from).collect();
    let tabs = Tabs::new(titles)
        .select(selected)
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("│"));
    f.render_widget(tabs, chunks[0]);

    // Optional banner under tabs
    let mut content_idx = 1usize;
    if let Some(line) = active_banner {
        let para = Paragraph::new(line);
        f.render_widget(para, chunks[1]);
        content_idx = 2;
    }

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
    if chunks.len() > content_idx && chunks[content_idx].height > 0 {
        f.render_widget(para, chunks[content_idx]);
    }

    // Help block at the bottom (wrapped to fit width)
    let help_idx = chunks.len().saturating_sub(1);
    if chunks[help_idx].height > 0 {
        let help_text = help_lines.join("\n");
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
        f.render_widget(help, chunks[help_idx]);
    }

    // Overlay: centered delete confirmation popup with colored text
    if app.is_confirm_delete() {
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let msg = format!("Delete? — {}  (Enter=Delete Esc=Cancel)", title);
        let content_w = UnicodeWidthStr::width(msg.as_str()) as u16;
        let popup_w = content_w.saturating_add(4).min(inner.width).max(20).min(inner.width);
        let popup_h: u16 = 3;
        let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
        let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
        let popup = Rect { x: px, y: py, width: popup_w, height: popup_h };

        let border_style = Style::default().fg(Color::Red);
        let title =
            Line::from(Span::styled(" Confirm ", border_style.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title).border_style(border_style);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner_popup = block.inner(popup);
        let para = Paragraph::new(Span::styled(msg, Style::default().fg(Color::Red)));
        f.render_widget(para, inner_popup);
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
    // Show specialized prompts only for relevant modes. For delete confirmation
    // we keep main content unchanged and render a popup overlay in `draw`.
    if app.is_estimate_editing() {
        let est = app.selected_estimate().unwrap_or(0);
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let suffix = if title.is_empty() { "".to_string() } else { format!(" — {}", title) };
        return vec![format!("Estimate: {}m{}  (+/-5m, Enter=OK Esc=Cancel)", est, suffix)];
    }
    // Command palette prompt (+ show target task title)
    if app.is_command_mode() {
        let buf = app.input_buffer().unwrap_or("");
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let suffix = if title.is_empty() { "".to_string() } else { format!(" — {}", title) };
        return vec![format!("Command: {} _{}  (Enter=Run Esc=Cancel)", buf, suffix)];
    }
    // Input mode (Normal/Interrupt) prompt only when not in delete confirm
    if app.in_input_mode() && !app.is_confirm_delete() {
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

/// Colorful, lazygit-inspired title line for the outer `Block`.
/// This keeps `format_header_line` unchanged for tests that rely on plain text,
/// while rendering a more readable, colorful header in the UI.
pub fn header_title_line(now_min: u16, app: &App) -> Line<'static> {
    // Reuse the same numbers as `format_header_line` to stay consistent.
    let esd_min = app.day.esd(now_min);
    let esd_h = esd_min / 60;
    let esd_m = esd_min % 60;

    let total_est_min: u32 = app.day.tasks.iter().map(|t| t.estimate_min as u32).sum();
    let total_act_min: u32 = app.day.tasks.iter().map(|t| t.actual_min as u32).sum();
    let carry_sec: u32 = app.day.tasks.iter().map(|t| t.actual_carry_sec as u32).sum();
    let total_act_sec = total_act_min * 60 + carry_sec;
    let rem_total_sec = (total_est_min * 60).saturating_sub(total_act_sec);
    let rem_m = (rem_total_sec / 60) as u16;
    let rem_s = (rem_total_sec % 60) as u16;
    let act_m = (total_act_sec / 60) as u16;
    let act_s = (total_act_sec % 60) as u16;

    // Colors: pills with bright BGs and bold text, separators in dim gray.
    let sep_style = Style::default().fg(Color::DarkGray);

    let pill = |label: &str, bg: Color| -> Span {
        Span::styled(
            label.to_string(),
            Style::default().fg(Color::Black).bg(bg).add_modifier(Modifier::BOLD),
        )
    };
    let val = |text: String, fg: Color| -> Span {
        Span::styled(text, Style::default().fg(fg).add_modifier(Modifier::BOLD))
    };

    let mut line: Line<'static> = Line::default();
    // ESD
    line.spans.push(pill("ESD", Color::Blue));
    line.spans.push(Span::raw(" "));
    line.spans.push(val(format!("{:02}:{:02}", esd_h, esd_m), Color::Cyan));
    line.spans.push(Span::styled("  |  ", sep_style));
    // Est remaining
    line.spans.push(pill("Est", Color::Green));
    line.spans.push(Span::raw(" "));
    line.spans.push(val(format!("{}m {}s", rem_m, rem_s), Color::Green));
    line.spans.push(Span::styled("  |  ", sep_style));
    // Actual total
    line.spans.push(pill("Act", Color::Magenta));
    line.spans.push(Span::raw(" "));
    line.spans.push(val(format!("{}m {}s", act_m, act_s), Color::Magenta));

    line
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

/// Build a one-line banner describing the currently running task, if any.
/// Example: "Now: > Focus Work (est:30m act:3m 12s)"
pub fn format_active_banner(app: &App) -> Option<Line<'static>> {
    let idx = app.day.active_index()?;
    let t = &app.day.tasks[idx];
    let mut line = Line::default();
    line.spans.push(Span::styled(
        "Now:".to_string(),
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    ));
    line.spans.push(Span::raw(" "));
    line.spans.push(Span::raw(state_icon(t.state).to_string()));
    line.spans.push(Span::raw(" "));
    line.spans.push(Span::styled(t.title.clone(), Style::default().fg(Color::Cyan)));
    line.spans.push(Span::raw(format!(
        " (est:{}m act:{}m {}s)",
        t.estimate_min, t.actual_min, t.actual_carry_sec
    )));
    Some(line)
}
