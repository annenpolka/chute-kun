use crate::config::Config;
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
    input: Option<Input>,
    pub config: Config,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputKind {
    Normal,
    Interrupt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    kind: InputKind,
    buffer: String,
}

impl App {
    pub fn new() -> Self { Self::with_config(Config::load()) }

    pub fn with_config(config: Config) -> Self {
        Self {
            title: "Chute_kun".to_string(),
            should_quit: false,
            day: DayPlan::new(vec![]),
            selected: 0,
            tomorrow: vec![],
            history: vec![],
            view: View::default(),
            active_accum_sec: 0,
            input: None,
            config,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        // If we are in input mode, interpret keys as text editing/submit/cancel
        if let Some(input) = self.input.as_mut() {
            match code {
                KeyCode::Enter => {
                    let (default_title, est) = match input.kind {
                        InputKind::Normal => ("New Task", 25u16),
                        InputKind::Interrupt => ("Interrupt", 15u16),
                    };
                    let title = if input.buffer.trim().is_empty() {
                        default_title.to_string()
                    } else {
                        input.buffer.trim().to_string()
                    };
                    let idx = self.add_task(&title, est);
                    self.selected = idx;
                    self.input = None;
                }
                KeyCode::Esc => {
                    self.input = None;
                }
                KeyCode::Backspace => {
                    input.buffer.pop();
                }
                KeyCode::Char(c) => {
                    input.buffer.push(c);
                }
                _ => {}
            }
            return;
        }
        match code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('i') => {
                // Enter input mode for a normal task
                self.input = Some(Input { kind: InputKind::Normal, buffer: String::new() });
            }
            KeyCode::Char('I') => {
                // Enter input mode for an interrupt task
                self.input = Some(Input { kind: InputKind::Interrupt, buffer: String::new() });
            }
            KeyCode::Enter => {
                // Toggle: if active -> pause; else start/resume selected or first eligible.
                if self.day.active_index().is_some() {
                    self.day.pause_active();
                } else {
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
        // If in input mode, delegate to text edit handling
        if self.in_input_mode() {
            self.handle_key(ev.code);
            return;
        }
        // Try config-based keymap first
        if let Some(action) = self.config.keys.action_for(&ev) {
            self.apply_action(action);
            return;
        }
        // Fallback to legacy code-based handling to keep backward compatibility in tests
        self.handle_key(ev.code);
    }

    /// Handle pasted text from the terminal (bracketed/kitty paste etc.).
    /// Appends to the input buffer only when in input mode.
    pub fn handle_paste(&mut self, s: &str) {
        if let Some(input) = self.input.as_mut() {
            input.buffer.push_str(s);
        }
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

    fn apply_action(&mut self, action: crate::config::Action) {
        use crate::config::Action as A;
        match action {
            A::Quit => {
                self.should_quit = true;
            }
            A::AddTask => {
                self.input = Some(Input { kind: InputKind::Normal, buffer: String::new() });
            }
            A::AddInterrupt => {
                self.input = Some(Input { kind: InputKind::Interrupt, buffer: String::new() });
            }
            A::StartOrResume => {
                // Toggle behavior for Enter-mapped action: pause if active, otherwise start/resume
                if self.day.active_index().is_some() {
                    self.day.pause_active();
                } else {
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
            A::FinishActive => {
                self.finish_active();
            }
            A::Pause => {
                self.day.pause_active();
            }
            A::ReorderUp => {
                let new = self.day.reorder_up(self.selected);
                self.selected = new;
            }
            A::ReorderDown => {
                let new = self.day.reorder_down(self.selected);
                self.selected = new;
            }
            A::EstimatePlus => {
                self.day.adjust_estimate(self.selected, 5);
            }
            A::Postpone => {
                self.postpone_selected();
            }
            A::ViewNext => {
                self.set_view(self.view.next());
            }
            A::ViewPrev => {
                self.set_view(self.view.prev());
            }
            A::SelectUp => self.select_up(),
            A::SelectDown => self.select_down(),
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

    // Input mode helpers for UI/tests
    pub fn in_input_mode(&self) -> bool {
        self.input.is_some()
    }
    pub fn input_buffer(&self) -> Option<&str> {
        self.input.as_ref().map(|i| i.buffer.as_str())
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

impl App {
    /// Replace task lists from an external snapshot.
    /// - Resets selection and carry seconds; keeps config.
    pub fn apply_snapshot(&mut self, today: Vec<crate::task::Task>, future: Vec<crate::task::Task>, past: Vec<crate::task::Task>) {
        self.day = DayPlan::new(today);
        self.tomorrow = future;
        self.history = past;
        self.selected = 0;
        self.active_accum_sec = 0;
        self.set_view(View::Today);
    }
}
