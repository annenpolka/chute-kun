use crossterm::event::KeyCode;
use crate::task::{DayPlan, Task};

#[derive(Debug, Default)]
pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub day: DayPlan,
}

impl App {
    pub fn new() -> Self {
        Self { title: "Chute_kun".to_string(), should_quit: false, day: DayPlan::new(vec![]) }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('i') => {
                let idx = self.add_task("Interrupt", 15);
                self.day.start(idx);
            }
            _ => {}
        }
    }

    pub fn add_task(&mut self, title: &str, estimate_min: u16) -> usize {
        self.day.add_task(Task::new(title, estimate_min))
    }
}
