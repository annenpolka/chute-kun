use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{Block, Borders},
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let area: Rect = f.size();
    let block = Block::default().title(app.title.as_str()).borders(Borders::ALL);
    f.render_widget(block, area);
}
