use crate::task::{DayPlan, Task};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    Past,
    #[default]
    Today,
    Future,
}

impl View {
    fn next(self) -> Self {
        match self {
            View::Past => View::Today,
            View::Today => View::Future,
            View::Future => View::Past,
        }
    }
    fn prev(self) -> Self {
        match self {
            View::Past => View::Future,
            View::Today => View::Past,
            View::Future => View::Today,
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub day: DayPlan,
    selected: usize,
    tomorrow: Vec<Task>,
    history: Vec<Task>,
    view: View,
    active_accum_sec: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Chute_kun".to_string(),
            should_quit: false,
            day: DayPlan::new(vec![]),
            selected: 0,
            tomorrow: vec![],
            history: vec![],
            view: View::default(),
            active_accum_sec: 0,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('i') => {
                // Create an interrupt task without auto-starting it
                let _idx = self.add_task("Interrupt", 15);
            }
            KeyCode::Enter => {
                // If nothing active, start/resume the selected if eligible; else first eligible.
                if self.day.active_index().is_none() {
                    let s = self.selected;
                    let eligible = matches!(
                        self.day.tasks.get(s).map(|t| t.state),
                        Some(crate::task::TaskState::Paused | crate::task::TaskState::Planned)
                    );
                    if eligible {
                        self.day.start(s);
                    } else if let Some(idx) = (0..self.day.tasks.len()).find(|&i| {
                        matches!(
                            self.day.tasks[i].state,
                            crate::task::TaskState::Paused | crate::task::TaskState::Planned
                        )
                    }) {
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
            KeyCode::Char('p') => {
                self.postpone_selected();
            }
            KeyCode::Tab => {
                self.set_view(self.view.next());
            }
            KeyCode::BackTab => {
                self.set_view(self.view.prev());
            }
            KeyCode::Up => self.select_up(),
            KeyCode::Down => self.select_down(),
            KeyCode::Char('k') => self.select_up(),
            KeyCode::Char('j') => self.select_down(),
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
        // mark done
        let _before_len = self.day.tasks.len();
        self.day.finish_active();
        // move done (former active) to history if exists
        if let Some(pos) = (0..self.day.tasks.len())
            .find(|&i| matches!(self.day.tasks[i].state, crate::task::TaskState::Done))
        {
            if let Some(task) = self.day.remove(pos) {
                self.history.push(task);
            }
        }
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }
    pub fn select_up(&mut self) {
        let len = self.current_len();
        if len == 0 {
            return;
        }
        self.selected = self.selected.saturating_sub(1);
    }
    pub fn select_down(&mut self) {
        let len = self.current_len();
        if len == 0 {
            return;
        }
        let last = len - 1;
        self.selected = (self.selected + 1).min(last);
    }

    pub fn postpone_selected(&mut self) {
        if self.day.tasks.is_empty() {
            return;
        }
        let idx = self.selected.min(self.day.tasks.len() - 1);
        if let Some(task) = self.day.remove(idx) {
            // stay planned for tomorrow
            self.tomorrow.push(Task { state: crate::task::TaskState::Planned, ..task });
        }
        if !self.day.tasks.is_empty() {
            self.selected = self.selected.min(self.day.tasks.len() - 1);
        } else {
            self.selected = 0;
        }
    }

    pub fn tomorrow_tasks(&self) -> &Vec<Task> {
        &self.tomorrow
    }
    pub fn history_tasks(&self) -> &Vec<Task> {
        &self.history
    }

    pub fn view(&self) -> View {
        self.view
    }

    pub fn active_carry_seconds(&self) -> u16 {
        self.active_accum_sec
    }

    fn set_view(&mut self, v: View) {
        self.view = v;
        // clamp selection to current view length
        let len = self.current_len();
        if len == 0 {
            self.selected = 0;
        } else {
            self.selected = self.selected.min(len - 1);
        }
    }

    fn current_len(&self) -> usize {
        match self.view {
            View::Past => self.history.len(),
            View::Today => self.day.tasks.len(),
            View::Future => self.tomorrow.len(),
        }
    }

    pub fn tick(&mut self, seconds: u16) {
        if let Some(active) = self.day.active_index() {
            self.active_accum_sec = self.active_accum_sec.saturating_add(seconds);
            while self.active_accum_sec >= 60 {
                self.active_accum_sec -= 60;
                if let Some(t) = self.day.tasks.get_mut(active) {
                    t.actual_min = t.actual_min.saturating_add(1);
                }
            }
        }
    }
}
