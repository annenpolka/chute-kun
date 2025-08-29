use crossterm::event::KeyCode;

#[derive(Debug, Default)]
pub struct App {
    pub title: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Chute_kun".to_string(),
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }
}

