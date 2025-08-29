use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, View};
use crate::task::TaskState;

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.size();
    let header = format_header_line(local_minutes(), app);
    let block = Block::default().title(header).borders(Borders::ALL);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = format_task_lines(app).join("\n");
    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
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
            format!(
                "{} {} {} (est:{}m act:{}m {}s)",
                sel,
                state_icon(t.state),
                t.title,
                t.estimate_min,
                t.actual_min,
                t.actual_sec
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
    let total_act_sec: u32 = app
        .day
        .tasks
        .iter()
        .map(|t| (t.actual_min as u32) * 60 + (t.actual_sec as u32))
        .sum();
    let rem_total_sec = (total_est_min * 60).saturating_sub(total_act_sec);
    let rem_m = (rem_total_sec / 60) as u16;
    let rem_s = (rem_total_sec % 60) as u16;
    let act_m = (total_act_sec / 60) as u16;
    let act_s = (total_act_sec % 60) as u16;
    let view = match app.view() {
        crate::app::View::Past => "Past",
        crate::app::View::Today => "Today",
        crate::app::View::Future => "Future",
    };
    format!(
        "ESD {:02}:{:02} | Est {}m {}s | Act {}m {}s | View: {}",
        esd_h, esd_m, rem_m, rem_s, act_m, act_s, view
    )
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
