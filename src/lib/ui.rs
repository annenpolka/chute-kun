use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::task::TaskState;

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.size();
    let block = Block::default().title(app.title.as_str()).borders(Borders::ALL);
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
        .map(|t| format!("{} {} ({}m)", state_icon(t.state), t.title, t.estimate_min))
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
