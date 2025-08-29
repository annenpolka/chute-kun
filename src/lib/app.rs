use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
            KeyCode::Enter => {
                // start first planned or resume the first paused if nothing active
                if self.day.active_index().is_none() {
                    if let Some(idx) = (0..self.day.tasks.len()).find(|&i| matches!(self.day.tasks[i].state, crate::task::TaskState::Paused | crate::task::TaskState::Planned)) {
                        self.day.start(idx);
                    }
                }
            }
            KeyCode::Char(' ') => {
                self.day.pause_active();
            }
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, ev: KeyEvent) {
        if ev.code == KeyCode::Enter && ev.modifiers.contains(KeyModifiers::SHIFT) {
            self.finish_active();
            return;
        }
        self.handle_key(ev.code);
    }

    pub fn add_task(&mut self, title: &str, estimate_min: u16) -> usize {
        self.day.add_task(Task::new(title, estimate_min))
    }

    pub fn finish_active(&mut self) {
        self.day.finish_active();
    }
}
