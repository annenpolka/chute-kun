use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
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
    if app.day.tasks.is_empty() {
        return vec!["No tasks — press 'i' to add".to_string()];
    }
    app
        .day
        .tasks
        .iter()
        .map(|t| format!("{} {} (est:{}m act:{}m)", state_icon(t.state), t.title, t.estimate_min, t.actual_min))
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
    let remaining = app.day.remaining_total_min();
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
