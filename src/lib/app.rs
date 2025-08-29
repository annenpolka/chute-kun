use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::task::{DayPlan, Task};

#[derive(Debug, Default)]
pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub day: DayPlan,
    selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self { title: "Chute_kun".to_string(), should_quit: false, day: DayPlan::new(vec![]), selected: 0 }
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
                // If nothing active, start/resume the selected if eligible; else first eligible.
                if self.day.active_index().is_none() {
                    let s = self.selected;
                    let eligible = matches!(self.day.tasks.get(s).map(|t| t.state), Some(crate::task::TaskState::Paused | crate::task::TaskState::Planned));
                    if eligible {
                        self.day.start(s);
                    } else if let Some(idx) = (0..self.day.tasks.len()).find(|&i| matches!(self.day.tasks[i].state, crate::task::TaskState::Paused | crate::task::TaskState::Planned)) {
                        self.day.start(idx);
                        self.selected = idx;
                    }
                }
            }
            KeyCode::Char(' ') => {
                self.day.pause_active();
            }
            KeyCode::Char(']') => {
                let new = self.day.reorder_down(self.selected);
                self.selected = new;
            }
            KeyCode::Char('[') => {
                let new = self.day.reorder_up(self.selected);
                self.selected = new;
            }
            KeyCode::Char('e') => {
                self.day.adjust_estimate(self.selected, 5);
            }
            KeyCode::Up => self.select_up(),
            KeyCode::Down => self.select_down(),
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

    pub fn selected_index(&self) -> usize { self.selected }
    pub fn select_up(&mut self) {
        if self.day.tasks.is_empty() { return; }
        self.selected = self.selected.saturating_sub(1);
    }
    pub fn select_down(&mut self) {
        if self.day.tasks.is_empty() { return; }
        let last = self.day.tasks.len() - 1;
        self.selected = (self.selected + 1).min(last);
    }
}
