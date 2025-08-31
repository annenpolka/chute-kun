use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
};
use unicode_width::UnicodeWidthStr;

use crate::app::{App, DisplayMode, View};
use crate::clock::Clock;
use crate::task::Category as TaskCategory;
use crate::task::TaskState;

// Theme: darker list highlights for better contrast with default light (white) text.
// These colors aim to keep contrast acceptable on common terminals while avoiding
// the eye‑searing effect of bright Blue/Cyan backgrounds.
pub const SELECTED_ROW_BG: Color = Color::Rgb(0, 60, 120); // dark blue
pub const HOVER_ROW_BG: Color = Color::Rgb(0, 100, 100); // dark cyan/teal
                                                         // Drag visuals
pub const DRAG_SOURCE_BG_A: Color = Color::Rgb(100, 0, 120); // purple (pulse A)
pub const DRAG_SOURCE_BG_B: Color = Color::Rgb(140, 0, 160); // brighter purple (pulse B)
pub const DRAG_TARGET_BG_A: Color = Color::Rgb(0, 120, 60); // greenish (pulse A)
pub const DRAG_TARGET_BG_B: Color = Color::Rgb(0, 160, 80); // brighter greenish (pulse B)

const MIN_LIST_LINES: u16 = 3; // table header + at least two rows

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.area();
    let header_line = header_title_line(app_display_base(app), app);
    let actions_line = header_action_buttons_line(app);
    // Left-align stats and right-align action buttons independently on the title bar
    let block = Block::default()
        .title(header_line.left_aligned())
        .title(actions_line.right_aligned())
        .borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Optional active-task banner just under the tabs
    let active_banner = format_active_banner(app);

    // Pre-compute wrapped help lines for current width, to size the layout.
    let help_lines = help_lines_for_width(app, inner.width.max(1));
    // Clamp help height so the task table keeps at least header + two rows visible
    let mut help_height = help_lines.len() as u16; // 1+ lines depending on width
    let reserved = 1 /* tabs */ + if active_banner.is_some() { 1 } else { 0 } + MIN_LIST_LINES;
    let max_help = inner.height.saturating_sub(reserved);
    if max_help > 0 {
        help_height = help_height.min(max_help);
    }
    help_height = help_height.max(1);

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

    // Tabs for date views (custom-rendered to support hover + precise hitboxes)
    render_tabs_line(f, chunks[0], app);

    // If we have an active banner, render it right below tabs
    let mut content_idx = 1usize; // index into chunks for main content
    if let Some(line) = active_banner {
        // Underline the entire banner line for visibility
        let para = Paragraph::new(line.patch_style(Modifier::UNDERLINED));
        f.render_widget(para, chunks[1]);
        content_idx = 2;
    }

    // Main content: always keep task list/table; popups render as overlays
    // Table-based rendering: columns on the left for planned and actual logs
    let now = app_display_base(app);
    let tasks_slice: Vec<crate::task::Task> = match app.view() {
        View::Past => app.history_tasks().clone(),
        View::Today => app.day.tasks.clone(),
        View::Future => app.tomorrow_tasks().clone(),
    };
    if tasks_slice.is_empty() {
        let para = Paragraph::new("No tasks — press 'i' to add");
        f.render_widget(para, chunks[content_idx]);
    } else {
        match app.display_mode() {
            DisplayMode::List => {
                let table = build_task_table(now, app, &tasks_slice);
                f.render_widget(table, chunks[content_idx]);
            }
            DisplayMode::Calendar => {
                render_calendar_day_at(
                    f,
                    chunks[content_idx],
                    app,
                    &tasks_slice,
                    crate::clock::system_now_minutes(),
                );
            }
        }
    }

    // Help block + 24h gauge at the very bottom line
    // When an active banner is present, help resides at the last chunk, not index 2.
    let help_idx = chunks.len().saturating_sub(1);
    if chunks[help_idx].height > 0 {
        let help_area = chunks[help_idx];
        // Reserve the last line for the 24h category gauge, render help text above it
        let (help_text_area, gauge_area) = if help_area.height >= 1 {
            let g = Rect { x: help_area.x, y: help_area.y + help_area.height - 1, width: help_area.width, height: 1 };
            let h = Rect { x: help_area.x, y: help_area.y, width: help_area.width, height: help_area.height.saturating_sub(1) };
            (h, g)
        } else {
            (help_area, help_area)
        };
        if help_text_area.height > 0 {
            let help_text = help_lines.join("\n");
            let help = Paragraph::new(help_text)
                .style(Style::default().fg(Color::DarkGray))
                .wrap(Wrap { trim: true });
            f.render_widget(help, help_text_area);
        }
        // Draw 24h gauge on the reserved last line
        if help_area.height >= 1 {
            render_bottom_24h_gauge(f, app, gauge_area, crate::clock::system_now_minutes());
        }
    }

    // Overlay: centered estimate editor popup (date + slider + OK/Cancel)
    if let Some(popup) = compute_estimate_popup_rect(app, area) {
        let border = Style::default().fg(Color::Yellow);
        let title_line =
            Line::from(Span::styled(" Estimate ", border.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title_line).border_style(border);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner = block.inner(popup);
        let msg = {
            let t = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
            format!("Estimate: {}m — {}", app.selected_estimate().unwrap_or(0), t)
        };
        let msg_rect = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
        f.render_widget(
            Paragraph::new(Span::styled(msg, Style::default().fg(Color::Yellow))),
            msg_rect,
        );
        if let Some(t) = app.day.tasks.get(app.selected_index()) {
            render_date_line(f, app, popup, inner, Color::Yellow, t.planned_ymd);
        }
        let (track, ok, cancel) = estimate_slider_hitboxes(app, popup);
        render_slider_line(f, track, app.selected_estimate().unwrap_or(0));
        let btn_y = ok.y;
        let mut spans: Vec<Span> = Vec::new();
        let pad = (ok.x.saturating_sub(inner.x)) as usize;
        if pad > 0 {
            spans.push(Span::raw(" ".repeat(pad)));
        }
        let ok_style = if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::EstOk)) {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Black).bg(Color::Blue).add_modifier(Modifier::BOLD)
        };
        spans.push(Span::styled("OK".to_string(), ok_style));
        let gap2 = cancel.x.saturating_sub(ok.x + ok.width) as usize;
        if gap2 > 0 {
            spans.push(Span::raw(" ".repeat(gap2)));
        }
        let cancel_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::EstCancel)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Cancel".to_string(), cancel_style));
        let btn_rect = Rect { x: inner.x, y: btn_y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Line::from(spans)), btn_rect);
    }

    // Overlay: Start Time slider popup (Space)
    if let Some(popup) = compute_start_time_popup_rect(app, area) {
        let border = Style::default().fg(Color::Cyan);
        let title_line =
            Line::from(Span::styled(" Start Time ", border.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title_line).border_style(border);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner = block.inner(popup);
        let mins =
            app.input_buffer().and_then(|s| s.parse::<u16>().ok()).unwrap_or(app_display_base(app));
        let hh = (mins / 60) % 24;
        let mm = mins % 60;
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let msg = if title.is_empty() {
            format!("Start: {:02}:{:02}", hh, mm)
        } else {
            format!("Start: {:02}:{:02} — {}", hh, mm, title)
        };
        let msg_rect = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Span::styled(msg, border)), msg_rect);
        let (track, ok, cancel) = estimate_slider_hitboxes(app, popup);
        render_time_slider_line(f, track, mins);
        let btn_y = ok.y;
        let mut spans: Vec<Span> = Vec::new();
        let pad = (ok.x.saturating_sub(inner.x)) as usize;
        if pad > 0 {
            spans.push(Span::raw(" ".repeat(pad)));
        }
        let ok_style = if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::EstOk)) {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Black).bg(Color::Blue).add_modifier(Modifier::BOLD)
        };
        spans.push(Span::styled("OK".to_string(), ok_style));
        let gap2 = cancel.x.saturating_sub(ok.x + ok.width) as usize;
        if gap2 > 0 {
            spans.push(Span::raw(" ".repeat(gap2)));
        }
        let cancel_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::EstCancel)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Cancel".to_string(), cancel_style));
        let btn_rect = Rect { x: inner.x, y: btn_y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Line::from(spans)), btn_rect);
    }

    // Overlay: centered input popup (styled buttons)
    if let Some(popup) = compute_input_popup_rect(app, area) {
        let border = Style::default().fg(Color::Cyan);
        let title_line =
            Line::from(Span::styled(" New Task ", border.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title_line).border_style(border);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner = block.inner(popup);
        let buf = app.input_buffer().unwrap_or("");
        let msg = format!("Title: {} _", buf);
        let msg_rect = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
        f.render_widget(
            Paragraph::new(Span::styled(msg, Style::default().fg(Color::Cyan))),
            msg_rect,
        );
        let (add, cancel) = input_popup_button_hitboxes(app, popup);
        let btn_y = add.y;
        let mut spans: Vec<Span> = Vec::new();
        let pad = (add.x.saturating_sub(inner.x)) as usize;
        if pad > 0 {
            spans.push(Span::raw(" ".repeat(pad)));
        }
        let add_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::InputAdd)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("OK".to_string(), add_style));
        let gap = cancel.x.saturating_sub(add.x + add.width) as usize;
        if gap > 0 {
            spans.push(Span::raw(" ".repeat(gap)));
        }
        let cancel_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::InputCancel)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Cancel".to_string(), cancel_style));
        let btn_rect = Rect { x: inner.x, y: btn_y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Line::from(spans)), btn_rect);
    }

    // Overlay: command palette popup (two buttons Run/Cancel)
    if let Some(popup) = compute_command_popup_rect(app, area) {
        let border = Style::default().fg(Color::Magenta);
        let title_line = Line::from(Span::styled(" Command ", border.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title_line).border_style(border);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner = block.inner(popup);
        let buf = app.input_buffer().unwrap_or("");
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let suffix = if title.is_empty() { "".to_string() } else { format!(" — {}", title) };
        let msg = format!("Command: {} _{}", buf, suffix);
        let msg_rect = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Span::styled(msg, border)), msg_rect);
        let (run, cancel) = command_popup_button_hitboxes(app, popup);
        let btn_y = run.y;
        let mut spans: Vec<Span> = Vec::new();
        let pad = (run.x.saturating_sub(inner.x)) as usize;
        if pad > 0 {
            spans.push(Span::raw(" ".repeat(pad)));
        }
        let run_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::InputAdd)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Blue).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Run".to_string(), run_style));
        let gap = cancel.x.saturating_sub(run.x + run.width) as usize;
        if gap > 0 {
            spans.push(Span::raw(" ".repeat(gap)));
        }
        let cancel_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::InputCancel)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Cancel".to_string(), cancel_style));
        let btn_rect = Rect { x: inner.x, y: btn_y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Line::from(spans)), btn_rect);
    }

    // Overlay: new-task estimate slider input popup
    if let Some(popup) = compute_new_task_estimate_popup_rect(app, area) {
        let border = Style::default().fg(Color::Green);
        let title_line =
            Line::from(Span::styled(" Estimate ", border.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title_line).border_style(border);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner = block.inner(popup);
        let cur_est: u16 = app
            .input_buffer()
            .and_then(|s| s.parse::<u16>().ok())
            .or_else(|| app.new_task_default_estimate())
            .unwrap_or(25);
        let title = app.new_task_title().unwrap_or("");
        let msg = if title.is_empty() {
            format!("Estimate: {}m", cur_est)
        } else {
            format!("Estimate: {}m — {}", cur_est, title)
        };
        let msg_rect = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
        f.render_widget(
            Paragraph::new(Span::styled(msg, Style::default().fg(Color::Green))),
            msg_rect,
        );
        if let Some(ymd) = app.new_task_planned_ymd() {
            render_date_line(f, app, popup, inner, Color::Green, ymd);
        }
        // Slider track
        let (track, _ok, _cancel) = estimate_slider_hitboxes(app, popup);
        render_slider_line(f, track, cur_est);
        let (add, cancel) = input_popup_button_hitboxes(app, popup);
        let btn_y = add.y;
        let mut spans: Vec<Span> = Vec::new();
        let pad = (add.x.saturating_sub(inner.x)) as usize;
        if pad > 0 {
            spans.push(Span::raw(" ".repeat(pad)));
        }
        let add_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::InputAdd)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Add".to_string(), add_style));
        let gap = cancel.x.saturating_sub(add.x + add.width) as usize;
        if gap > 0 {
            spans.push(Span::raw(" ".repeat(gap)));
        }
        let cancel_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::InputCancel)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Cancel".to_string(), cancel_style));
        let btn_rect = Rect { x: inner.x, y: btn_y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Line::from(spans)), btn_rect);
    }

    // Overlay: centered delete confirmation popup with colored text + styled buttons
    if app.is_confirm_delete() {
        let popup = compute_delete_popup_rect(app, area).unwrap();
        let border_style = Style::default().fg(Color::Red);
        let title_line =
            Line::from(Span::styled(" Confirm ", border_style.add_modifier(Modifier::BOLD)));
        let block =
            Block::default().borders(Borders::ALL).title(title_line).border_style(border_style);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner_popup = block.inner(popup);
        // First inner line: red message
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let msg = format!("Delete? — {}  (Enter=Delete Esc=Cancel)", title);
        let msg_para = Paragraph::new(Span::styled(msg, Style::default().fg(Color::Red)));
        let msg_rect =
            Rect { x: inner_popup.x, y: inner_popup.y, width: inner_popup.width, height: 1 };
        f.render_widget(msg_para, msg_rect);
        // Second inner line: pill-styled buttons aligned with hitboxes
        let (del_rect, cancel_rect) = delete_popup_button_hitboxes(app, popup);
        let btn_y = del_rect.y;
        let mut spans: Vec<Span> = Vec::new();
        let pad_del = (del_rect.x.saturating_sub(inner_popup.x)) as usize;
        if pad_del > 0 {
            spans.push(Span::raw(" ".repeat(pad_del)));
        }
        let del_style = if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::Delete))
        {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)
        };
        spans.push(Span::styled("Delete".to_string(), del_style));
        let gap = cancel_rect.x.saturating_sub(del_rect.x + del_rect.width) as usize;
        if gap > 0 {
            spans.push(Span::raw(" ".repeat(gap)));
        }
        let can_style = if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::Cancel))
        {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD)
        };
        spans.push(Span::styled("Cancel".to_string(), can_style));
        let btn_line = Paragraph::new(Line::from(spans));
        let btn_rect = Rect { x: inner_popup.x, y: btn_y, width: inner_popup.width, height: 1 };
        f.render_widget(btn_line, btn_rect);
    }

    // Overlay: category picker
    if let Some(popup) = compute_category_popup_rect(app, area) {
        // Title
        let header_line = header_title_line(app_display_base(app), app);
        let block = Block::default().title(header_line).borders(Borders::ALL);
        f.render_widget(block, area);
        // Inner box for list
        let inner = Rect { x: popup.x, y: popup.y, width: popup.width, height: popup.height };
        let mut rows: Vec<Row> = Vec::new();
        let options = category_options(app);
        for (i, (label, color, _cat)) in options.iter().enumerate() {
            let bullet = Span::styled("●".to_string(), Style::default().fg(*color));
            let text = Span::styled(
                label.clone(),
                Style::default().fg(*color).add_modifier(Modifier::BOLD),
            );
            let line = Line::from(vec![bullet, Span::raw(" "), text]);
            let mut row = Row::new(vec![Cell::from(line)]);
            if app.category_pick_index() == i {
                row = row.style(Style::default().bg(Color::Blue));
            }
            rows.push(row);
        }
        let table = Table::new(rows, [Constraint::Min(10)])
            .header(
                Row::new(vec![Cell::from("Category")])
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            )
            .block(Block::default().borders(Borders::ALL).title("Select Category (Enter)"));
        f.render_widget(table, inner);
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

    // Keep layout consistent with `draw`: tabs, optional banner, content, help block
    let active_banner = format_active_banner(app);
    let help_lines = help_lines_for_width(app, inner.width.max(1));
    // Clamp help height so the task table keeps at least header + two rows visible
    let mut help_height = help_lines.len() as u16;
    let reserved = 1 /* tabs */ + if active_banner.is_some() { 1 } else { 0 } + MIN_LIST_LINES;
    let max_help = inner.height.saturating_sub(reserved);
    if max_help > 0 {
        help_height = help_height.min(max_help);
    }
    help_height = help_height.max(1);
    let mut constraints: Vec<Constraint> = vec![Constraint::Length(1)];
    if active_banner.is_some() {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Min(0));
    constraints.push(Constraint::Length(help_height.max(1)));
    let chunks =
        Layout::default().direction(Direction::Vertical).constraints(constraints).split(inner);

    render_tabs_line(f, chunks[0], app);

    // Optional banner under tabs
    let mut content_idx = 1usize;
    if let Some(line) = active_banner {
        let para = Paragraph::new(line.patch_style(Modifier::UNDERLINED));
        f.render_widget(para, chunks[1]);
        content_idx = 2;
    }

    // Content with injected clock: render as a table
    let tasks_slice: Vec<crate::task::Task> = match app.view() {
        View::Past => app.history_tasks().clone(),
        View::Today => app.day.tasks.clone(),
        View::Future => app.tomorrow_tasks().clone(),
    };
    if tasks_slice.is_empty() {
        let para = Paragraph::new("No tasks — press 'i' to add");
        if chunks.len() > content_idx && chunks[content_idx].height > 0 {
            f.render_widget(para, chunks[content_idx]);
        }
    } else if chunks.len() > content_idx && chunks[content_idx].height > 0 {
        match app.display_mode() {
            DisplayMode::List => {
                let table = build_task_table(now, app, &tasks_slice);
                f.render_widget(table, chunks[content_idx]);
            }
            DisplayMode::Calendar => {
                render_calendar_day_at(
                    f,
                    chunks[content_idx],
                    app,
                    &tasks_slice,
                    clock.now_minutes(),
                );
            }
        }
    }

    // Help block + 24h gauge at the bottom
    let help_idx = chunks.len().saturating_sub(1);
    if chunks[help_idx].height > 0 {
        let help_area = chunks[help_idx];
        let (help_text_area, gauge_area) = if help_area.height >= 1 {
            let g = Rect { x: help_area.x, y: help_area.y + help_area.height - 1, width: help_area.width, height: 1 };
            let h = Rect { x: help_area.x, y: help_area.y, width: help_area.width, height: help_area.height.saturating_sub(1) };
            (h, g)
        } else {
            (help_area, help_area)
        };
        if help_text_area.height > 0 {
            let help_text = help_lines.join("\n");
            let help = Paragraph::new(help_text)
                .style(Style::default().fg(Color::DarkGray))
                .wrap(Wrap { trim: true });
            f.render_widget(help, help_text_area);
        }
        if help_area.height >= 1 {
            render_bottom_24h_gauge(f, app, gauge_area, clock.now_minutes());
        }
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
        let para = Paragraph::new(Span::styled(msg.clone(), Style::default().fg(Color::Red)));
        f.render_widget(para, inner_popup);
    }
    // Overlay: Start Time popup under injected clock path as well
    if let Some(popup) = compute_start_time_popup_rect(app, area) {
        let border = Style::default().fg(Color::Cyan);
        let title_line =
            Line::from(Span::styled(" Start Time ", border.add_modifier(Modifier::BOLD)));
        let block = Block::default().borders(Borders::ALL).title(title_line).border_style(border);
        f.render_widget(Clear, popup);
        f.render_widget(block.clone(), popup);
        let inner = block.inner(popup);
        let mins =
            app.input_buffer().and_then(|s| s.parse::<u16>().ok()).unwrap_or(app_display_base(app));
        let hh = (mins / 60) % 24;
        let mm = mins % 60;
        let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
        let msg = if title.is_empty() {
            format!("Start: {:02}:{:02}", hh, mm)
        } else {
            format!("Start: {:02}:{:02} — {}", hh, mm, title)
        };
        let msg_rect = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Span::styled(msg, border)), msg_rect);
        let (track, ok, cancel) = estimate_slider_hitboxes(app, popup);
        render_time_slider_line(f, track, mins);
        let btn_y = ok.y;
        let mut spans: Vec<Span> = Vec::new();
        let pad = (ok.x.saturating_sub(inner.x)) as usize;
        if pad > 0 {
            spans.push(Span::raw(" ".repeat(pad)));
        }
        let ok_style = if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::EstOk)) {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Black).bg(Color::Blue).add_modifier(Modifier::BOLD)
        };
        spans.push(Span::styled("OK".to_string(), ok_style));
        let gap2 = cancel.x.saturating_sub(ok.x + ok.width) as usize;
        if gap2 > 0 {
            spans.push(Span::raw(" ".repeat(gap2)));
        }
        let cancel_style =
            if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::EstCancel)) {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD)
            };
        spans.push(Span::styled("Cancel".to_string(), cancel_style));
        let btn_rect = Rect { x: inner.x, y: btn_y, width: inner.width, height: 1 };
        f.render_widget(Paragraph::new(Line::from(spans)), btn_rect);
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
    // For estimate edit/new-task estimate/title input/delete confirm, keep main list lines
    match app.view() {
        View::Past => render_list_slice(now_min, app, app.history_tasks()),
        View::Today => render_list_slice(now_min, app, &app.day.tasks),
        View::Future => render_list_slice(now_min, app, app.tomorrow_tasks()),
    }
}

/// Compute planned start minutes for each task, respecting per-task fixed start times.
/// Algorithm:
/// - `cursor` starts at `now_min` (typically day_start)
/// - For each task in order: if it has `fixed_start_min`, set `cursor = max(cursor, fixed)`.
///   The task's start is `cursor`; then advance `cursor += estimate_min`.
fn compute_planned_starts(now_min: u16, tasks: &[crate::task::Task]) -> Vec<u16> {
    let mut cursor = now_min;
    let mut out: Vec<u16> = Vec::with_capacity(tasks.len());
    for t in tasks.iter() {
        if let Some(fs) = t.fixed_start_min {
            cursor = cursor.max(fs);
        }
        out.push(cursor);
        cursor = cursor.saturating_add(t.estimate_min);
    }
    out
}

fn render_list_slice(now_min: u16, app: &App, tasks: &[crate::task::Task]) -> Vec<String> {
    if tasks.is_empty() {
        return vec!["No tasks — press 'i' to add".to_string()];
    }
    // active index not needed for seconds rendering anymore (per-task seconds)

    // Build schedule start times considering per-task fixed start time.
    let starts: Vec<u16> = compute_planned_starts(now_min, tasks);

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
            let planned = format!(
                "{:02}:{:02} {} {} {} (est:{}m act:{}m {}s)",
                hh,
                mm,
                sel,
                state_icon(t.state),
                t.title,
                t.estimate_min,
                t.actual_min,
                secs
            );
            // Actual start/end column
            let act_col = match (t.started_at_min, t.finished_at_min) {
                (Some(s), Some(e)) => format!(
                    "実測 {:02}:{:02}-{:02}:{:02}",
                    (s / 60) % 24,
                    s % 60,
                    (e / 60) % 24,
                    e % 60
                ),
                (Some(s), None) => {
                    format!("実測 {:02}:{:02}-", (s / 60) % 24, s % 60)
                }
                _ => "実測 --:--".to_string(),
            };
            format!("{}  |  {}", planned, act_col)
        })
        .collect()
}

fn format_actual_last_finish_time(t: &crate::task::Task) -> String {
    // Prefer the explicit finished_at_min if present (final finish)
    if let Some(e) = t.finished_at_min {
        return format!("{:02}:{:02}", (e / 60) % 24, e % 60);
    }
    // Otherwise, find the most recent session with an end time
    if let Some(e) = t.sessions.iter().rev().find_map(|s| s.end_min) {
        return format!("{:02}:{:02}", (e / 60) % 24, e % 60);
    }
    // No finished session yet
    "--:--".to_string()
}

fn build_task_table(now_min: u16, app: &App, tasks_slice: &[crate::task::Task]) -> Table<'static> {
    // If empty, show the hint paragraph to save space
    let mut rows: Vec<Row> = Vec::new();
    // Build schedule start times similar to `render_list_slice`
    let starts: Vec<u16> = compute_planned_starts(now_min, tasks_slice);

    let selected = app.selected_index().min(tasks_slice.len().saturating_sub(1));
    let hovered = app.hovered_index();
    let dragging = app.is_dragging();
    let drag_from = app.drag_source_index();
    let pulse_on = app.pulse_on();
    for (i, t) in tasks_slice.iter().enumerate() {
        let hh = (starts[i] / 60) % 24;
        let mm = starts[i] % 60;
        let mut planned_cell = Cell::from(format!("{:02}:{:02}", hh, mm));
        if t.fixed_start_min.is_some() {
            planned_cell = planned_cell.style(Style::default().fg(Color::Cyan));
        }
        let actual_cell = Cell::from(format_actual_last_finish_time(t));
        // Title cell with colored state icon and plain title (estimate is a dedicated column)
        let mut spans: Vec<Span> = Vec::new();
        // Drag target indicator arrow before icon (only while dragging over this row)
        if dragging && hovered == Some(i) {
            let arrow = match drag_from {
                Some(from) if from < i => "↓",
                Some(from) if from > i => "↑",
                _ => "•",
            };
            let arrow_style = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
            spans.push(Span::styled(arrow.to_string(), arrow_style));
            spans.push(Span::raw(" "));
        }
        spans.push(state_icon_span(t.state));
        spans.push(Span::raw(" "));
        // Category colored dot (configurable)
        let cat_color = app.config.category_color(t.category);
        spans.push(Span::styled("●".to_string(), Style::default().fg(cat_color)));
        spans.push(Span::raw(" "));
        // Title color: by category; Done overrides to gray. Keep strikethrough for Done.
        let title_fg = if matches!(t.state, TaskState::Done) { Color::DarkGray } else { cat_color };
        let mut title_style = if matches!(t.state, TaskState::Done) {
            Style::default().add_modifier(Modifier::CROSSED_OUT)
        } else {
            Style::default()
        };
        title_style = title_style.fg(title_fg);
        spans.push(Span::styled(t.title.clone(), title_style));
        let title_cell = Cell::from(Line::from(spans));
        // New dedicated estimate column
        let est_cell = Cell::from(format!("{}m", t.estimate_min));
        // New dedicated accumulated time column with seconds
        // Show seconds while Active or Paused
        let secs = if matches!(t.state, TaskState::Active | TaskState::Paused) {
            t.actual_carry_sec
        } else {
            0
        };
        let act_cell = Cell::from(format!("{}m {}s", t.actual_min, secs));
        let highlight_bg = if dragging {
            if Some(i) == drag_from {
                Some(if pulse_on { DRAG_SOURCE_BG_B } else { DRAG_SOURCE_BG_A })
            } else if hovered == Some(i) {
                Some(if pulse_on { DRAG_TARGET_BG_B } else { DRAG_TARGET_BG_A })
            } else if i == selected {
                Some(SELECTED_ROW_BG)
            } else {
                None
            }
        } else if i == selected {
            Some(SELECTED_ROW_BG)
        } else if hovered == Some(i) {
            Some(HOVER_ROW_BG)
        } else {
            None
        };
        // Column order: Plan | Est | Task | Act | Actual
        let mut row = Row::new(vec![planned_cell, est_cell, title_cell, act_cell, actual_cell]);
        if let Some(bg) = highlight_bg {
            let s = Style::default().bg(bg);
            row = row.style(s);
        }
        rows.push(row);
    }

    // Header
    // Header labels follow the same order: Plan | Est | Task | Act | Actual
    let header = Row::new(vec![
        Cell::from("Plan"),
        Cell::from("Est"),
        Cell::from("Task"),
        Cell::from("Act"),
        Cell::from("Actual"),
    ])
    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    // Column widths: Plan fixed 5, Est fits e.g. "120m" (4), Task grows, Act fits e.g. "120m 59s" (9), Actual label width (6)
    let widths = [
        Constraint::Length(5), // Plan
        Constraint::Length(4), // Est
        Constraint::Min(10),   // Task
        Constraint::Length(9), // Act
        Constraint::Length(6), // Actual
    ];

    // Render table using a minimal block to avoid nested borders (outer block already drawn)
    Table::new(rows, widths).header(header).column_spacing(1).block(Block::default())
}

fn state_icon(state: TaskState) -> &'static str {
    match state {
        TaskState::Planned => " ",
        TaskState::Active => ">",
        TaskState::Paused => "=",
        TaskState::Done => "x",
    }
}

fn state_icon_span(state: TaskState) -> Span<'static> {
    let sym = state_icon(state).to_string();
    let style = match state {
        TaskState::Active => Style::default().fg(Color::Green),
        TaskState::Paused => Style::default().fg(Color::Yellow),
        TaskState::Done => Style::default().fg(Color::DarkGray),
        TaskState::Planned => Style::default(),
    };
    Span::styled(sym, style)
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

// ---- Category picker UI helpers ----

pub fn category_options(app: &App) -> Vec<(String, Color, crate::task::Category)> {
    vec![
        (
            app.config.category_name(crate::task::Category::General),
            app.config.category_color(crate::task::Category::General),
            crate::task::Category::General,
        ),
        (
            app.config.category_name(crate::task::Category::Work),
            app.config.category_color(crate::task::Category::Work),
            crate::task::Category::Work,
        ),
        (
            app.config.category_name(crate::task::Category::Home),
            app.config.category_color(crate::task::Category::Home),
            crate::task::Category::Home,
        ),
        (
            app.config.category_name(crate::task::Category::Hobby),
            app.config.category_color(crate::task::Category::Hobby),
            crate::task::Category::Hobby,
        ),
    ]
}

pub fn compute_category_popup_rect(app: &App, area: Rect) -> Option<Rect> {
    if !app.is_category_picker() {
        return None;
    }
    let width: u16 = 24;
    let height: u16 = 1 /* header */ + category_options(app).len() as u16 + 2; // box padding
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Some(Rect { x, y, width, height })
}

pub fn category_picker_hitboxes(app: &App, popup: Rect) -> Vec<Rect> {
    let list_y = popup.y + 2; // leave a small margin
    let mut rects = Vec::new();
    for i in 0..category_options(app).len() as u16 {
        rects.push(Rect {
            x: popup.x + 1,
            y: list_y + i,
            width: popup.width.saturating_sub(2),
            height: 1,
        });
    }
    rects
}

/// Right-aligned action buttons for the title bar: New | Start | Stop | Finish | Delete.
/// Buttons render as bold, black-on-colored "pills" similar to other UI elements.
pub fn header_action_buttons_line(app: &App) -> Line<'static> {
    let hovered = app.hovered_header_button();
    let labels = header_action_button_labels();
    let enabled = header_action_button_enabled(app);
    let colors = [Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::Red];
    let mut spans: Vec<Span> = Vec::new();
    for i in 0..labels.len() {
        let label = &labels[i];
        let is_enabled = enabled[i];
        let is_hover = matches!(
            (i, hovered),
            (0, Some(crate::app::HeaderButton::New))
                | (1, Some(crate::app::HeaderButton::Start))
                | (2, Some(crate::app::HeaderButton::Stop))
                | (3, Some(crate::app::HeaderButton::Finish))
                | (4, Some(crate::app::HeaderButton::Delete))
        );
        let style = if is_enabled {
            let bg = if is_hover { Color::Cyan } else { colors[i] };
            Style::default().fg(Color::Black).bg(bg).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(Span::styled(label.clone(), style));
        if i + 1 != labels.len() {
            spans.push(Span::raw(" "));
        }
    }
    Line::from(spans)
}

/// Compute hitboxes (terminal rectangles) for each header action button on the top border line.
/// Returns boxes in the same order as rendering: [New, Start, Stop, Finish, Delete].
/// Coordinates are relative to the full `area` passed to the app draw loop.
pub fn header_action_buttons_hitboxes(area: Rect) -> Vec<Rect> {
    // Available width for titles excludes the two corner cells.
    let available = area.width.saturating_sub(2);
    let labels = header_action_button_labels(); // include shortcut hints
    let gaps = 4u16; // 4 spaces between 5 labels
    let labels_w: u16 = labels.iter().map(|s| UnicodeWidthStr::width(s.as_str()) as u16).sum();
    let total_w = labels_w + gaps;
    // Start X for the right-aligned group: left corner + (available - total)
    let start_x = area.x + 1 + available.saturating_sub(total_w);
    let mut xs = start_x;
    let mut rects: Vec<Rect> = Vec::with_capacity(labels.len());
    for (i, s) in labels.iter().enumerate() {
        let w = UnicodeWidthStr::width(s.as_str()) as u16;
        rects.push(Rect { x: xs, y: area.y, width: w.max(1), height: 1 });
        xs = xs.saturating_add(w);
        if i + 1 != labels.len() {
            xs = xs.saturating_add(1); // gap space
        }
    }
    rects
}

/// Labels (with keyboard shortcut hints) used for header buttons in both render and hitboxes.
pub fn header_action_button_labels() -> Vec<String> {
    // Keep labels short to avoid overlapping the left stats header on narrow terminals.
    // Keyboard shortcuts remain documented in the help line.
    vec![
        "New".to_string(),
        "Start".to_string(),
        "Stop".to_string(),
        "Finish".to_string(),
        "Delete".to_string(),
    ]
}

/// Enabled state for each header button (New, Start, Stop, Finish, Delete).
pub fn header_action_button_enabled(app: &App) -> [bool; 5] {
    use crate::app::View;
    let on_today = matches!(app.view(), View::Today);
    let has_today = !app.day.tasks.is_empty();
    let selected_eligible = on_today
        && app
            .day
            .tasks
            .get(app.selected_index())
            .map(|t| {
                matches!(t.state, crate::task::TaskState::Paused | crate::task::TaskState::Planned)
            })
            .unwrap_or(false);
    let has_active = on_today && app.day.active_index().is_some();
    let can_finish = on_today && has_today;
    let can_delete = on_today && has_today;
    [true, selected_eligible, has_active, can_finish, can_delete]
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
        "Enter: start/pause | Shift+Enter/f: finish | Space: time | i: interrupt | p: postpone | x: delete | b: bring | [: up | ]: down | e: edit | j/k";
    format!("{} | {}", nav, task)
}

/// Optimized help depending on the current view.
/// - Today: show full task actions
/// - Past/Future: show only navigation and quit to reduce noise
pub fn format_help_line_for(app: &App) -> String {
    // Build items using the same source as wrapped help
    let items = help_items_for(app);
    items.join(" | ")
}

/// Build help items depending on the current view. Used for wrapping.
pub fn help_items_for(app: &App) -> Vec<String> {
    use crate::config::join_key_labels as join;
    // Popup‑scoped help: when a popup is open, restrict to its operations only.
    if app.is_confirm_delete() {
        return vec!["Enter/y: delete".to_string(), "Esc/n: cancel".to_string()];
    }
    if app.is_start_time_edit() {
        return vec![
            "Enter: OK".to_string(),
            "Esc: cancel".to_string(),
            "Left/Right/Up/Down/j/k: +/-5m".to_string(),
            "click slider: set time".to_string(),
        ];
    }
    if app.is_estimate_editing() {
        return vec![
            "Enter: OK".to_string(),
            "Esc: cancel".to_string(),
            "Left/Right/Up/Down/j/k: +/-5m".to_string(),
            ".,: +/-1 day".to_string(),
            "click slider: set estimate".to_string(),
            "click < >: date".to_string(),
        ];
    }
    if app.is_new_task_estimate() {
        return vec![
            "Enter: add".to_string(),
            "Esc: cancel".to_string(),
            ".,: +/-1 day".to_string(),
            "click slider: set estimate".to_string(),
            "click < >: date".to_string(),
        ];
    }
    if app.is_command_mode() {
        return vec![
            "Enter: run".to_string(),
            "Esc: cancel".to_string(),
            "type/backspace: edit".to_string(),
        ];
    }
    if app.is_text_input_mode() {
        return vec![
            "Enter: next".to_string(),
            "Esc: cancel".to_string(),
            "type/backspace: edit".to_string(),
        ];
    }

    // Default (no popup): view‑aware general help
    let km = &app.config.keys;
    let mut items: Vec<String> =
        vec![format!("{}: quit", join(&km.quit)), format!("{}: switch view", join(&km.view_next))];
    match app.view() {
        View::Today => {
            items.push(format!("{}: start/pause", join(&km.start_or_resume)));
            items.push(format!("{}: time", join(&km.popup)));
            items.push(format!("{}: finish", join(&km.finish_active)));
            // Interrupt: reflect configured keys
            items.push(format!("{}: interrupt", join(&km.add_interrupt)));
            items.push(format!("{}: postpone", join(&km.postpone)));
            // delete key now configurable
            items.push(format!("{}: delete", join(&km.delete)));
            items.push(format!("{}: up", join(&km.reorder_up)));
            items.push(format!("{}: down", join(&km.reorder_down)));
            items.push(format!("{}: edit", join(&km.estimate_plus)));
            // Toggle display mode (List <-> Calendar)
            items.push(format!("{}: calendar", join(&km.toggle_blocks)));
            // Category cycle (configurable)
            items.push(format!("{}: category", join(&km.category_cycle)));
            // Category picker open (configurable)
            items.push(format!("{}: picker", join(&km.category_picker)));
            // Compact vim-like navigation chars as trailing hint (if present)
            let up_chars: Vec<char> = km
                .select_up
                .iter()
                .filter_map(|k| match k.code {
                    crossterm::event::KeyCode::Char(c) if k.modifiers.is_empty() => Some(c),
                    _ => None,
                })
                .collect();
            let down_chars: Vec<char> = km
                .select_down
                .iter()
                .filter_map(|k| match k.code {
                    crossterm::event::KeyCode::Char(c) if k.modifiers.is_empty() => Some(c),
                    _ => None,
                })
                .collect();
            if let (Some(d), Some(u)) = (down_chars.first(), up_chars.first()) {
                items.push(format!("{}/{}", d, u));
            } else {
                items.push("j/k".to_string());
            }
        }
        View::Past => {
            // Minimal + category operations now available
            items.push(format!("{}: category", join(&km.category_cycle)));
            items.push(format!("{}: picker", join(&km.category_picker)));
        }
        View::Future => {
            items.push(format!("{}: bring", join(&km.bring_to_today)));
            items.push(format!("{}: category", join(&km.category_cycle)));
            items.push(format!("{}: picker", join(&km.category_picker)));
        }
    }
    items
}

/// Wrap help items into lines that fit within `width` cells, inserting ` | ` between items.
/// This uses Unicode width to count display cells.
pub fn wrap_help_items_to_width(items: &[String], width: u16) -> Vec<String> {
    let width = width as usize;
    if width == 0 {
        return vec![String::new()];
    }
    let mut lines: Vec<String> = Vec::new();
    let mut cur = String::new();
    let sep = " | ";
    for item in items.iter() {
        if cur.is_empty() {
            cur.push_str(item.as_str());
            continue;
        }
        let candidate = format!("{}{}{}", cur, sep, item);
        if UnicodeWidthStr::width(candidate.as_str()) <= width {
            cur = candidate;
        } else {
            // commit current line and start a new one
            lines.push(cur);
            cur = item.to_string();
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
    line.spans.push(state_icon_span(t.state));
    line.spans.push(Span::raw(" "));
    // Running task title (no underline here; banner is underlined as a whole)
    line.spans.push(Span::styled(t.title.clone(), Style::default().fg(Color::Cyan)));
    line.spans.push(Span::raw(format!(
        " (est:{}m act:{}m {}s)",
        t.estimate_min, t.actual_min, t.actual_carry_sec
    )));
    Some(line)
}

fn render_tabs_line(f: &mut Frame, rect: Rect, app: &App) {
    let (titles, selected) = tab_titles(app);
    let hover = app.hovered_tab_index();
    let mut line = Line::default();
    for (i, title) in titles.iter().enumerate() {
        let mut style = Style::default();
        if Some(i) == hover && Some(i) != Some(selected) {
            style = style.fg(Color::Cyan);
        }
        if i == selected {
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        line.spans.push(Span::styled(title.clone(), style));
        if i + 1 != titles.len() {
            line.spans.push(Span::styled("│".to_string(), Style::default().fg(Color::DarkGray)));
        }
    }
    let para = Paragraph::new(line);
    f.render_widget(para, rect);
}

/// Render a one-line 24h horizontal gauge showing actual work sessions by category.
/// - Range is fixed to 00:00..24:00 mapped across `rect.width` cells.
/// - Colored segments reflect task categories; overlapping sessions pick the last seen.
/// - Ongoing session uses `now_min` as its temporary end.
fn render_bottom_24h_gauge(f: &mut Frame, app: &App, rect: Rect, now_min: u16) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let w = rect.width as usize;
    // Prepare per-cell colors; default None means no work recorded
    let mut cells: Vec<Option<Color>> = vec![None; w];
    // Gather all actual sessions for the current view's tasks
    let tasks_slice: Vec<crate::task::Task> = match app.view() {
        View::Past => app.history_tasks().clone(),
        View::Today => app.day.tasks.clone(),
        View::Future => app.tomorrow_tasks().clone(),
    };
    for t in tasks_slice.iter() {
        let cat_color = app.config.category_color(t.category);
        for s in t.sessions.iter() {
            let s_min = s.start_min.min(23 * 60 + 59);
            let e_min = s.end_min.unwrap_or(now_min).min(23 * 60 + 59);
            if e_min < s_min {
                continue;
            }
            let x0 = ((s_min as u32) * (rect.width as u32) / 1440) as usize;
            let x1 = ((e_min as u32) * (rect.width as u32) / 1440) as usize;
            let x0 = x0.min(w.saturating_sub(1));
            let x1 = x1.min(w.saturating_sub(1)).max(x0);
            for x in x0..=x1 {
                cells[x] = Some(cat_color);
            }
        }
    }
    // Compress into styled spans
    let mut spans: Vec<Span> = Vec::new();
    let mut i = 0usize;
    while i < w {
        let color = cells[i];
        let j = ((i + 1)..=w).find(|&k| k == w || cells[k] != color).unwrap_or(w);
        let len = j - i;
        let sym = if color.is_some() { "█" } else { "·" };
        let text = sym.repeat(len);
        let style = match color {
            Some(c) => Style::default().fg(c).add_modifier(Modifier::BOLD),
            None => Style::default().fg(Color::DarkGray),
        };
        spans.push(Span::styled(text, style));
        i = j;
    }
    let line = Line::from(spans);
    let para = Paragraph::new(line);
    f.render_widget(para, rect);
}

/// Compute hitboxes for tab titles inside the `tabs_rect`.
pub fn tab_hitboxes(app: &App, tabs_rect: Rect) -> Vec<Rect> {
    let (titles, _sel) = tab_titles(app);
    let mut xs = tabs_rect.x;
    let mut boxes = Vec::with_capacity(titles.len());
    for (i, t) in titles.iter().enumerate() {
        let w = UnicodeWidthStr::width(t.as_str()) as u16;
        let r = Rect { x: xs, y: tabs_rect.y, width: w.max(1), height: 1 };
        boxes.push(r);
        xs = xs.saturating_add(w);
        if i + 1 != titles.len() {
            // account for divider "│"
            xs = xs.saturating_add(1);
        }
    }
    boxes
}

pub fn compute_delete_popup_rect(app: &App, area: Rect) -> Option<Rect> {
    if !app.is_confirm_delete() {
        return None;
    }
    // Reconstruct inner same as draw
    let area: Rect = area;
    let header_line = header_title_line(app_display_base(app), app);
    let block = Block::default().title(header_line).borders(Borders::ALL);
    let inner = block.inner(area);
    // Message content identical to draw
    let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
    let msg = format!("Delete? — {}  (Enter=Delete Esc=Cancel)", title);
    let content_w = UnicodeWidthStr::width(msg.as_str()) as u16;
    let popup_w = content_w.saturating_add(4).min(inner.width).max(20).min(inner.width);
    let popup_h: u16 = 4;
    let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
    let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
    Some(Rect { x: px, y: py, width: popup_w, height: popup_h })
}

pub fn delete_popup_button_hitboxes(_app: &App, popup: Rect) -> (Rect, Rect) {
    // Buttons are rendered on the second inner line, left-aligned, separated by two spaces
    let inner_popup = Rect {
        x: popup.x + 1,
        y: popup.y + 1,
        width: popup.width.saturating_sub(2),
        height: popup.height.saturating_sub(2),
    };
    let btn_y = inner_popup.y + 1;
    let del_w = UnicodeWidthStr::width("Delete") as u16;
    let can_w = UnicodeWidthStr::width("Cancel") as u16;
    let total = del_w + 2 + can_w;
    let start_x = inner_popup.x + (inner_popup.width.saturating_sub(total)) / 2;
    let del_rect = Rect { x: start_x, y: btn_y, width: del_w, height: 1 };
    let can_x = start_x + del_w + 2;
    let cancel_rect = Rect { x: can_x, y: btn_y, width: can_w, height: 1 };
    (del_rect, cancel_rect)
}

// Estimate popup
pub fn compute_estimate_popup_rect(app: &App, area: Rect) -> Option<Rect> {
    if !app.is_estimate_editing() {
        return None;
    }
    let block =
        Block::default().title(header_title_line(app_display_base(app), app)).borders(Borders::ALL);
    let inner = block.inner(area);
    let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
    let msg = format!("Estimate: {}m — {}", app.selected_estimate().unwrap_or(0), title);
    let content_w = (UnicodeWidthStr::width(msg.as_str()).max(date_line_min_width().into()) as u16)
        .saturating_add(0);
    let popup_w = content_w.saturating_add(4).min(inner.width).max(34).min(inner.width);
    let popup_h: u16 = 6; // message + date + slider + buttons line
    let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
    let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
    Some(Rect { x: px, y: py, width: popup_w, height: popup_h })
}

// Slider hitboxes for estimate editor
pub fn estimate_slider_hitboxes(app: &App, popup: Rect) -> (Rect, Rect, Rect) {
    let inner = Rect {
        x: popup.x + 1,
        y: popup.y + 1,
        width: popup.width.saturating_sub(2),
        height: popup.height.saturating_sub(2),
    };
    // In estimate popups, one extra line (date) is above the slider
    let track_y = if app.is_new_task_estimate() || app.is_estimate_editing() {
        inner.y + 2
    } else {
        inner.y + 1
    };
    let track =
        Rect { x: inner.x + 2, y: track_y, width: inner.width.saturating_sub(4), height: 1 };
    let ok_w = UnicodeWidthStr::width("OK") as u16;
    let ca_w = UnicodeWidthStr::width("Cancel") as u16;
    let total = ok_w + 2 + ca_w;
    let start_x = inner.x + (inner.width.saturating_sub(total)) / 2;
    let ok = Rect { x: start_x, y: track_y + 1, width: ok_w, height: 1 };
    let cancel = Rect { x: start_x + ok_w + 2, y: track_y + 1, width: ca_w, height: 1 };
    (track, ok, cancel)
}

pub fn slider_x_for_minutes(track: Rect, min: u16, max: u16, step: u16, minutes: u16) -> u16 {
    let minutes = minutes.clamp(min, max);
    let steps = ((max - min) / step).max(1);
    let pos = ((minutes - min) / step).min(steps);
    let w = track.width.max(1) as u32;
    let x = track.x as u32 + (pos as u32 * (w - 1)) / (steps as u32);
    x as u16
}

pub fn minutes_from_slider_x(track: Rect, min: u16, max: u16, step: u16, x: u16) -> u16 {
    let w = track.width.max(1);
    let x = x.clamp(track.x, track.x + w.saturating_sub(1));
    let rel = (x - track.x) as u32;
    let steps = ((max - min) / step).max(1) as u32;
    let pos = (rel * steps + (w as u32 - 1) / 2) / (w as u32 - 1).max(1);
    (min + (pos as u16) * step).clamp(min, max)
}

// Input popup (Normal/Interrupt)
pub fn compute_input_popup_rect(app: &App, area: Rect) -> Option<Rect> {
    if !(app.is_text_input_mode()) {
        return None;
    }
    let block =
        Block::default().title(header_title_line(app_display_base(app), app)).borders(Borders::ALL);
    let inner = block.inner(area);
    let buf = app.input_buffer().unwrap_or("");
    let msg = format!("Title: {} _", buf);
    let content_w = UnicodeWidthStr::width(msg.as_str()) as u16;
    let popup_w = content_w.saturating_add(4).min(inner.width).max(30).min(inner.width);
    let popup_h: u16 = 4; // message + buttons
    let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
    let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
    Some(Rect { x: px, y: py, width: popup_w, height: popup_h })
}

// New-task estimate popup (slider)
pub fn compute_new_task_estimate_popup_rect(app: &App, area: Rect) -> Option<Rect> {
    if !app.is_new_task_estimate() {
        return None;
    }
    let block =
        Block::default().title(header_title_line(app_display_base(app), app)).borders(Borders::ALL);
    let inner = block.inner(area);
    let est = app
        .input_buffer()
        .and_then(|s| s.parse::<u16>().ok())
        .or_else(|| app.new_task_default_estimate())
        .unwrap_or(25);
    let title = app.new_task_title().unwrap_or("");
    let msg = if title.is_empty() {
        format!("Estimate: {}m", est)
    } else {
        format!("Estimate: {}m — {}", est, title)
    };
    let content_w = (UnicodeWidthStr::width(msg.as_str()).max(date_line_min_width().into()) as u16)
        .saturating_add(0);
    let popup_w = content_w.saturating_add(4).min(inner.width).max(34).min(inner.width);
    // Include: message, date, slider, buttons (inner height 4)
    let popup_h: u16 = 6;
    let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
    let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
    Some(Rect { x: px, y: py, width: popup_w, height: popup_h })
}

// Command popup geometry (single-line prompt + buttons)
pub fn compute_command_popup_rect(app: &App, area: Rect) -> Option<Rect> {
    if !app.is_command_mode() {
        return None;
    }
    let inner = Block::default().borders(Borders::ALL).inner(area);
    if inner.width < 10 || inner.height < 3 {
        return None;
    }
    let buf = app.input_buffer().unwrap_or("");
    let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
    let suffix = if title.is_empty() { "".to_string() } else { format!(" — {}", title) };
    let content = format!("Command: {} _{}", buf, suffix);
    let content_w = content.width() as u16;
    let popup_w = content_w.saturating_add(4).min(inner.width).max(30).min(inner.width);
    let popup_h: u16 = 4; // message + buttons
    let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
    let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
    Some(Rect { x: px, y: py, width: popup_w, height: popup_h })
}

pub fn input_popup_button_hitboxes(_app: &App, popup: Rect) -> (Rect, Rect) {
    let inner = Rect {
        x: popup.x + 1,
        y: popup.y + 1,
        width: popup.width.saturating_sub(2),
        height: popup.height.saturating_sub(2),
    };
    // Place buttons on the last inner line, so it adapts to both 4-line and 5-line popups
    let y = inner.y + inner.height.saturating_sub(1);
    let add_w = UnicodeWidthStr::width("OK") as u16;
    let ca_w = UnicodeWidthStr::width("Cancel") as u16;
    let total = add_w + 2 + ca_w;
    let start_x = inner.x + (inner.width.saturating_sub(total)) / 2;
    let add = Rect { x: start_x, y, width: add_w, height: 1 };
    let cancel = Rect { x: start_x + add_w + 2, y, width: ca_w, height: 1 };
    (add, cancel)
}

pub fn command_popup_button_hitboxes(_app: &App, popup: Rect) -> (Rect, Rect) {
    let inner = Rect {
        x: popup.x + 1,
        y: popup.y + 1,
        width: popup.width.saturating_sub(2),
        height: popup.height.saturating_sub(2),
    };
    let y = inner.y + inner.height.saturating_sub(1);
    let run_w = UnicodeWidthStr::width("Run") as u16;
    let ca_w = UnicodeWidthStr::width("Cancel") as u16;
    let total = run_w + 2 + ca_w;
    let start_x = inner.x + (inner.width.saturating_sub(total)) / 2;
    let run = Rect { x: start_x, y, width: run_w, height: 1 };
    let cancel = Rect { x: start_x + run_w + 2, y, width: ca_w, height: 1 };
    (run, cancel)
}

// note: helpers that parsed message text for positions were removed in favor of
// explicit hitbox geometry to reduce dead code and simplify clippy compliance.

fn date_label_for(ymd: u32) -> String {
    let base = if crate::date::is_valid_ymd(ymd) { ymd } else { crate::date::today_ymd() };
    let wd = crate::date::weekday_short_en(base);
    if base == crate::date::today_ymd() {
        format!("Today ({})", wd)
    } else if base == crate::date::add_days_to_ymd(crate::date::today_ymd(), 1) {
        format!("Tomorrow ({})", wd)
    } else {
        format!("{} ({})", crate::date::format_ymd(base), wd)
    }
}

fn date_line_min_width() -> u16 {
    use unicode_width::UnicodeWidthStr as UW;
    let w1 = UW::width("Date: Today (Wed)") as u16;
    let w2 = UW::width("Date: Tomorrow (Wed)") as u16;
    let w3 = UW::width("Date: 2099-12-31 (Wed)") as u16;
    w1.max(w2).max(w3)
}

/// Return hitboxes for the date picker line: (prev_btn, label, next_btn).
/// The line lives at `inner.y + 1` in both estimate popups.
pub fn date_picker_hitboxes(_app: &App, popup: Rect) -> (Rect, Rect, Rect) {
    let inner = Rect {
        x: popup.x + 1,
        y: popup.y + 1,
        width: popup.width.saturating_sub(2),
        height: popup.height.saturating_sub(2),
    };
    let y = inner.y + 1; // date line
                         // Stable anchors: fixed buttons so mouse targets do not move with label width
    let prev = Rect { x: inner.x + 2, y, width: 1, height: 1 };
    let next = Rect { x: inner.x + inner.width.saturating_sub(3), y, width: 1, height: 1 };
    // Label fills the space between prev and next minus single spaces around it
    let label_x = prev.x + 2; // "< " then label
    let label_w = next.x.saturating_sub(label_x).saturating_sub(1);
    let label_rect = Rect { x: label_x, y, width: label_w, height: 1 };
    (prev, label_rect, next)
}

// Simple quick popup geometry
pub fn compute_start_time_popup_rect(app: &App, area: Rect) -> Option<Rect> {
    if !app.is_start_time_edit() {
        return None;
    }
    let inner = Block::default().borders(Borders::ALL).inner(area);
    if inner.width < 24 || inner.height < 4 {
        return None;
    }
    let title = app.day.tasks.get(app.selected_index()).map(|t| t.title.as_str()).unwrap_or("");
    let mins =
        app.input_buffer().and_then(|s| s.parse::<u16>().ok()).unwrap_or(app_display_base(app));
    let hh = (mins / 60) % 24;
    let mm = mins % 60;
    let msg = if title.is_empty() {
        format!("Start: {:02}:{:02}", hh, mm)
    } else {
        format!("Start: {:02}:{:02} — {}", hh, mm, title)
    };
    let content_w = UnicodeWidthStr::width(msg.as_str()) as u16;
    let popup_w = content_w.saturating_add(6).min(inner.width).max(28).min(inner.width);
    let popup_h: u16 = 4; // message + slider + buttons
    let px = inner.x + (inner.width.saturating_sub(popup_w)) / 2;
    let py = inner.y + (inner.height.saturating_sub(popup_h)) / 2;
    Some(Rect { x: px, y: py, width: popup_w, height: popup_h })
}

fn render_time_slider_line(f: &mut Frame, track: Rect, minutes: u16) {
    let min = 0u16;
    let max = 23 * 60 + 59;
    let step = 5u16;
    let knob_x = slider_x_for_minutes(track, min, max, step, minutes);
    let mut line = Line::default();
    line.spans.push(Span::styled("[".to_string(), Style::default().fg(Color::DarkGray)));
    for x in track.x..track.x + track.width {
        if x == knob_x {
            line.spans.push(Span::styled(
                "●".to_string(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
        } else if x < knob_x {
            line.spans.push(Span::styled("=".to_string(), Style::default().fg(Color::Green)));
        } else {
            line.spans.push(Span::styled("·".to_string(), Style::default().fg(Color::DarkGray)));
        }
    }
    line.spans.push(Span::styled("]".to_string(), Style::default().fg(Color::DarkGray)));
    let para = Paragraph::new(line);
    let expanded = Rect {
        x: track.x.saturating_sub(1),
        y: track.y,
        width: track.width.saturating_add(2),
        height: 1,
    };
    f.render_widget(para, expanded);
}

fn render_date_line(f: &mut Frame, app: &App, popup: Rect, inner: Rect, color: Color, ymd: u32) {
    let (prev, label_rect, next) = date_picker_hitboxes(app, popup);
    let date_label = date_label_for(ymd);
    let text = format!("Date: {}", date_label);
    let mut spans: Vec<Span> = Vec::new();
    // pad until prev
    let left_pad = prev.x.saturating_sub(inner.x) as usize;
    if left_pad > 0 {
        spans.push(Span::raw(" ".repeat(left_pad)));
    }
    let prev_style = if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::DatePrev))
    {
        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    };
    spans.push(Span::styled("<".to_string(), prev_style));
    spans.push(Span::raw(" "));
    // Fit label to available width
    let fitted = fit_to_width(&text, label_rect.width as usize);
    spans.push(Span::styled(fitted, Style::default().fg(color)));
    // Compute spaces so that '>' appears at next.x
    let printed_w = (UnicodeWidthStr::width(text.as_str()) as u16).min(label_rect.width);
    let gap = next.x.saturating_sub(prev.x + 2 + printed_w) as usize;
    if gap > 0 {
        spans.push(Span::raw(" ".repeat(gap)));
    }
    let next_style = if matches!(app.popup_hover_button(), Some(crate::app::PopupButton::DateNext))
    {
        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    };
    spans.push(Span::styled(">".to_string(), next_style));
    let date_rect = Rect { x: inner.x, y: label_rect.y, width: inner.width, height: 1 };
    f.render_widget(Paragraph::new(Line::from(spans)), date_rect);
}

fn fit_to_width(s: &str, width: usize) -> String {
    use unicode_width::UnicodeWidthChar;
    if UnicodeWidthStr::width(s) <= width {
        return s.to_string();
    }
    let mut out = String::new();
    let mut used = 0usize;
    for ch in s.chars() {
        let w = ch.width().unwrap_or(1);
        if used + w > width {
            break;
        }
        out.push(ch);
        used += w;
    }
    out
}

fn render_slider_line(f: &mut Frame, track: Rect, minutes: u16) {
    // Styled slider: [====●····]
    let min = 0u16;
    let max = 240u16;
    let step = 5u16;
    let knob_x = slider_x_for_minutes(track, min, max, step, minutes);
    let mut line = Line::default();
    // Left bracket just before track
    line.spans.push(Span::styled("[".to_string(), Style::default().fg(Color::DarkGray)));
    for x in track.x..track.x + track.width {
        if x == knob_x {
            line.spans.push(Span::styled(
                "●".to_string(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
        } else if x < knob_x {
            line.spans.push(Span::styled("=".to_string(), Style::default().fg(Color::Green)));
        } else {
            line.spans.push(Span::styled("·".to_string(), Style::default().fg(Color::DarkGray)));
        }
    }
    line.spans.push(Span::styled("]".to_string(), Style::default().fg(Color::DarkGray)));
    let para = Paragraph::new(line);
    // Expand rect by one cell left/right to contain brackets when possible
    let expanded = Rect {
        x: track.x.saturating_sub(1),
        y: track.y,
        width: track.width.saturating_add(2),
        height: 1,
    };
    f.render_widget(para, expanded);
}

/// Compute key layout rectangles used by `draw`, for hit testing and tests.
/// Returns (tabs, optional active banner, list/content, help) within the inner bordered area.
pub fn compute_layout(app: &App, area: Rect) -> (Rect, Option<Rect>, Rect, Rect) {
    // Replicate the same sizing logic as `draw`.
    // First, account for the outer Block's borders.
    let inner = Rect {
        x: area.x.saturating_add(1),
        y: area.y.saturating_add(1),
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    // Optional active banner allocates one line under tabs
    let has_banner = format_active_banner(app).is_some();

    // Help height depends on wrapping for the current width; keep at least
    // table header + two rows visible in the list area.
    let help_lines = help_lines_for_width(app, inner.width.max(1));
    let mut help_height = help_lines.len() as u16;
    let reserved = 1 /* tabs */ + if has_banner { 1 } else { 0 } + MIN_LIST_LINES;
    let max_help = inner.height.saturating_sub(reserved);
    if max_help > 0 {
        help_height = help_height.min(max_help);
    }
    help_height = help_height.max(1);
    let tabs = Rect { x: inner.x, y: inner.y, width: inner.width, height: 1 };
    let mut y = inner.y + 1;
    let banner = if has_banner {
        let b = Rect { x: inner.x, y, width: inner.width, height: 1 };
        y += 1;
        Some(b)
    } else {
        None
    };
    // List/content takes remaining minus help
    let list_height = inner.height.saturating_sub(y - inner.y).saturating_sub(help_height);
    let list = Rect { x: inner.x, y, width: inner.width, height: list_height };
    let help = Rect {
        x: inner.x,
        y: inner.y + inner.height.saturating_sub(help_height),
        width: inner.width,
        height: help_height,
    };
    (tabs, banner, list, help)
}

// ---- Time Blocks (Timeline) Mode ----

// (removed horizontal time blocks view)

/// Google Calendar風の縦軸タイムライン。左に時刻ラベル、右に2レーン（Plan/Actual）。
fn render_calendar_day_at(
    f: &mut Frame,
    rect: Rect,
    app: &App,
    tasks: &[crate::task::Task],
    now_min: u16,
) {
    if rect.height == 0 || rect.width < 12 {
        return;
    }
    let start_min = app_display_base(app);
    // Planned end accumulates estimates sequentially from day start
    let mut cur = start_min;
    let mut planned_ranges: Vec<(u16, u16, String, TaskCategory)> = Vec::new();
    for t in tasks.iter() {
        if let Some(fs) = t.fixed_start_min {
            cur = cur.max(fs);
        }
        let s = cur;
        let e = cur.saturating_add(t.estimate_min);
        planned_ranges.push((s, e, t.title.clone(), t.category));
        cur = e;
    }
    let mut act_ranges: Vec<(usize, u16, u16, String, TaskCategory, bool)> = Vec::new();
    for (ti, t) in tasks.iter().enumerate() {
        for s in t.sessions.iter() {
            let end = s.end_min.unwrap_or(now_min);
            let closed = s.end_min.is_some();
            act_ranges.push((ti, s.start_min, end, t.title.clone(), t.category, closed));
        }
    }
    // Hide sub-minute work on the calendar: drop zero-minute ranges (start == end)
    act_ranges.retain(|(_ti, s, e, _title, _cat, _closed)| e > s);
    let latest = planned_ranges
        .iter()
        .map(|&(_, e, _, _)| e)
        .chain(act_ranges.iter().map(|&(_, _, e, _, _, _)| e))
        .max()
        .unwrap_or(start_min);
    let end_min = latest.max(start_min + 90); // ensure some space (≥1.5h)
    let span = end_min.saturating_sub(start_min).max(1);

    // Layout: [gutter 6] [space 1] [plan lane] [gap 1] [act lane]
    let gutter = 6u16; // e.g., "09:00"
    let gaps = 2u16; // spaces between lanes and gutter
    let lanes_w = rect.width.saturating_sub(gutter + gaps);
    let lane_w = (lanes_w.saturating_sub(1)) / 2; // leave 1 col gap between lanes
    if lane_w == 0 {
        return;
    }
    // lanes start after the gutter; we render strings via Paragraph

    // Map minute -> y row in [0..h-1]
    let to_y = |m: u16, h: u16| -> u16 {
        let rel = m.saturating_sub(start_min) as u32;
        let y = (rel * (h.saturating_sub(1) as u32)) / (span as u32);
        y as u16
    };

    // Prepare line-by-line strings for lanes
    let mut lines_plan: Vec<String> = vec![" ".repeat(lane_w as usize); rect.height as usize];
    // We'll build Actual lane strings later (supports horizontal split)
    let mut lines_act: Vec<String> = vec![" ".repeat(lane_w as usize); rect.height as usize];
    // Per-row colors derived from the category of the task covering the row (plan side)
    let mut plan_colors: Vec<Option<Color>> = vec![None; rect.height as usize];
    // For actual side after columnization (per-column colors are tracked separately)

    for (s, e, title, cat) in planned_ranges.into_iter() {
        let y0 = to_y(s, rect.height);
        let y1 = to_y(e, rect.height).max(y0);
        // Resolve this block's category color directly from range
        let this_color = app.config.category_color(cat);
        for y in y0..=y1 {
            if let Some(row) = lines_plan.get_mut(y as usize) {
                *row = "█".repeat(lane_w as usize);
            }
            if let Some(slot) = plan_colors.get_mut(y as usize) {
                *slot = Some(this_color);
            }
        }
        // Put the task title on the first row of its planned block
        if let Some(row) = lines_plan.get_mut(y0 as usize) {
            let fitted = fit_to_width(&title, lane_w as usize);
            use unicode_width::UnicodeWidthStr as UW;
            let w = UW::width(fitted.as_str()) as u16;
            let mut s = String::new();
            s.push_str(&fitted);
            if w < lane_w {
                s.push_str(&"█".repeat((lane_w - w) as usize));
            }
            *row = s;
        }
    }
    // ---- Actual lane with overlap split into columns ----
    #[derive(Clone)]
    struct Block {
        ti: usize,
        s: u16,
        e: u16,
        y0: u16,
        y1: u16,
        title: String,
        cat: TaskCategory,
        closed: bool,
        col: usize,
    }
    let mut blocks_raw: Vec<Block> = act_ranges
        .iter()
        .cloned()
        .map(|(ti, s, e, title, cat, closed)| {
            let y0 = to_y(s, rect.height);
            let y1 = to_y(e, rect.height).max(y0);
            Block { ti, s, e, y0, y1, title, cat, closed, col: 0 }
        })
        .collect();
    // Merge per-task blocks that overlap or touch visually
    blocks_raw.sort_by_key(|b| (b.ti, b.y0, b.y1));
    let mut blocks: Vec<Block> = Vec::new();
    let mut k = 0usize;
    while k < blocks_raw.len() {
        let mut cur = blocks_raw[k].clone();
        k += 1;
        while k < blocks_raw.len() && blocks_raw[k].ti == cur.ti && blocks_raw[k].y0 <= cur.y1 {
            let b = blocks_raw[k].clone();
            cur.y1 = cur.y1.max(b.y1);
            cur.e = cur.e.max(b.e);
            cur.closed = cur.closed && b.closed;
            k += 1;
        }
        blocks.push(cur);
    }
    // Assign columns greedily
    blocks.sort_by_key(|b| b.s);
    // Track last occupied row (y1) per column to detect visual overlap (row-level)
    let mut col_yend: Vec<u16> = Vec::new();
    for b in blocks.iter_mut() {
        let mut placed = false;
        for (ci, yend) in col_yend.iter_mut().enumerate() {
            // Non-overlap visually when previous end row is strictly above next start row
            if *yend < b.y0 {
                b.col = ci;
                *yend = b.y1;
                placed = true;
                break;
            }
        }
        if !placed {
            b.col = col_yend.len();
            col_yend.push(b.y1);
        }
    }
    let mut ncols = col_yend.len().max(1);
    // Omission rule: show at most 3 columns
    if ncols > 3 {
        ncols = 3;
    }
    if ncols as u16 > lane_w {
        ncols = lane_w as usize;
    }
    let base_w = (lane_w / ncols as u16).max(1);
    let rem = (lane_w % ncols as u16) as usize;
    let col_widths: Vec<u16> = (0..ncols).map(|i| base_w + if i < rem { 1 } else { 0 }).collect();
    let mut lines_act_cols: Vec<Vec<String>> =
        vec![vec![String::new(); ncols]; rect.height as usize];
    let mut act_col_colors: Vec<Vec<Option<Color>>> = vec![vec![None; ncols]; rect.height as usize];
    for row in lines_act_cols.iter_mut().take(rect.height as usize) {
        for (c, cell) in row.iter_mut().enumerate().take(ncols) {
            *cell = " ".repeat(col_widths[c] as usize);
        }
    }
    // Fill blocks
    for b in blocks.iter() {
        let col = b.col.min(ncols.saturating_sub(1));
        let y0 = to_y(b.s, rect.height);
        let y1 = to_y(b.e, rect.height).max(y0);
        let cw = col_widths[col] as usize;
        for y in y0..=y1 {
            let yi = y as usize;
            if let Some(cell) = lines_act_cols.get_mut(yi).and_then(|row| row.get_mut(col)) {
                *cell = "▓".repeat(cw);
            }
            if let Some(slot) = act_col_colors.get_mut(yi).and_then(|row| row.get_mut(col)) {
                if slot.is_none() {
                    *slot = Some(app.config.category_color(b.cat));
                }
            }
        }
    }
    // Overlay titles per column on their start rows for closed sessions
    for b in blocks.iter() {
        if b.closed && b.e <= now_min {
            let y = to_y(b.s, rect.height).min(rect.height.saturating_sub(1));
            let yi = y as usize;
            let col = b.col.min(ncols.saturating_sub(1));
            let cw = col_widths[col] as usize;
            let fitted = fit_to_width(&b.title, cw);
            use unicode_width::UnicodeWidthStr as UW;
            let w = UW::width(fitted.as_str()) as usize;
            if let Some(cell) = lines_act_cols.get_mut(yi).and_then(|row| row.get_mut(col)) {
                let mut sline = String::new();
                sline.push_str(&fitted);
                if w < cw {
                    sline.push_str(&"▓".repeat(cw - w));
                }
                *cell = sline;
            }
            if let Some(slot) = act_col_colors.get_mut(yi).and_then(|row| row.get_mut(col)) {
                if slot.is_none() {
                    *slot = Some(app.config.category_color(b.cat));
                }
            }
        }
    }
    // Join columns per row into act strings
    for y in 0..rect.height as usize {
        let mut s = String::new();
        for c in 0..ncols {
            s.push_str(&lines_act_cols[y][c]);
        }
        let w = s.chars().count() as u16;
        if w < lane_w {
            s.push_str(&" ".repeat((lane_w - w) as usize));
        }
        lines_act[y] = s;
    }

    // Precompute hour labels mapped to row positions to avoid missing due to discretization
    let mut hour_labels: Vec<Option<String>> = vec![None; rect.height as usize];
    let mut hmark = start_min.saturating_sub(start_min % 60); // floor to hour
    while hmark <= end_min {
        let y = to_y(hmark, rect.height);
        let label = format!("{:02}:00", (hmark / 60) % 24);
        if (y as usize) < hour_labels.len() {
            hour_labels[y as usize] = Some(label);
        }
        hmark = hmark.saturating_add(60);
    }

    // Render per line: gutter label | plan lane | gap | act lane
    let active_title: Option<String> = if matches!(app.view(), View::Today) {
        app.day.active_index().and_then(|idx| app.day.tasks.get(idx)).map(|t| t.title.clone())
    } else {
        None
    };
    for i in 0..rect.height {
        let y = rect.y + i;
        let label = hour_labels[i as usize].clone().unwrap_or_default();
        let mut left = format!("{label:>6}");
        // ensure width
        if left.len() < gutter as usize {
            left = format!("{:>width$}", left, width = gutter as usize);
        }
        let is_now_row = {
            let y_now = {
                let rel = now_min.saturating_sub(start_min) as u32;
                (rel * (rect.height.saturating_sub(1) as u32) / (span as u32)) as u16
            };
            i == y_now
        };
        let left_span = Span::styled(left, Style::default().fg(Color::DarkGray));
        let plan_cell = if is_now_row {
            // Show Now HH:MM at the head of the plan lane, then extend with line
            let hh = (now_min / 60) % 24;
            let mm = now_min % 60;
            let label = format!("Now {:02}:{:02}", hh, mm);
            let fitted = fit_to_width(&label, lane_w as usize);
            use unicode_width::UnicodeWidthStr as UW;
            let w = UW::width(fitted.as_str()) as u16;
            let mut s = String::new();
            s.push_str(&fitted);
            if w < lane_w {
                s.push_str(&"─".repeat((lane_w - w) as usize));
            }
            s
        } else {
            lines_plan[i as usize].clone()
        };
        let act_cell = if is_now_row {
            if let Some(title) = active_title.as_ref() {
                let fitted = fit_to_width(title, lane_w as usize);
                use unicode_width::UnicodeWidthStr as UW;
                let w = UW::width(fitted.as_str()) as u16;
                let mut s = String::new();
                s.push_str(&fitted);
                if w < lane_w {
                    s.push_str(&"─".repeat((lane_w - w) as usize));
                }
                s
            } else {
                "─".repeat(lane_w as usize)
            }
        } else {
            lines_act[i as usize].clone()
        };
        let plan_span = Span::styled(
            plan_cell,
            if is_now_row {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(plan_colors[i as usize].unwrap_or(Color::Green))
            },
        );
        let gap_span = if is_now_row {
            Span::styled("─", Style::default().fg(Color::Red))
        } else {
            Span::raw(" ")
        };
        let act_style = if is_now_row {
            if active_title.is_some() {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Red)
            }
        } else {
            // Unused for per-column styles; keep default
            Style::default()
        };
        let mut parts: Vec<Span> = vec![left_span, Span::raw(" "), plan_span, gap_span];
        if is_now_row {
            parts.push(Span::styled(act_cell, act_style));
        } else {
            // Build per-column styled spans
            let yi = i as usize;
            for (c, seg) in lines_act_cols[yi].iter().enumerate() {
                let fg = act_col_colors[yi][c].unwrap_or(Color::Magenta);
                parts.push(Span::styled(seg.clone(), Style::default().fg(fg)));
            }
        }
        let line = Line::from(parts);
        let para = Paragraph::new(line);
        f.render_widget(para, Rect { x: rect.x, y, width: rect.width, height: 1 });
    }
}
