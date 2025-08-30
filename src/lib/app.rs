use crate::config::Config;
use crate::date::today_ymd;
use crate::task::{DayPlan, Task};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

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
    input: Option<Input>,
    pub config: Config,
    last_seen_ymd: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputKind {
    Normal,
    Interrupt,
    Command,
    EstimateEdit,
    ConfirmDelete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    kind: InputKind,
    buffer: String,
}

impl App {
    pub fn new() -> Self {
        Self::with_config(Config::load())
    }

    pub fn with_config(config: Config) -> Self {
        let ymd = today_ymd();
        Self {
            title: "Chute_kun".to_string(),
            should_quit: false,
            day: DayPlan::new(vec![]),
            selected: 0,
            tomorrow: vec![],
            history: vec![],
            view: View::default(),
            input: None,
            config,
            last_seen_ymd: ymd,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        // If we are in input mode, interpret keys as text editing/submit/cancel
        if let Some(input) = self.input.as_mut() {
            match input.kind {
                InputKind::Normal | InputKind::Interrupt => match code {
                    KeyCode::Enter => {
                        let (default_title, est) = match input.kind {
                            InputKind::Normal => ("New Task", 25u16),
                            InputKind::Interrupt => ("Interrupt", 15u16),
                            _ => unreachable!(),
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
                },
                InputKind::Command => match code {
                    KeyCode::Enter => {
                        let cmd = input.buffer.trim().to_string();
                        self.apply_command(&cmd);
                        self.input = None;
                    }
                    KeyCode::Esc => {
                        self.input = None;
                    }
                    KeyCode::Backspace => {
                        input.buffer.pop();
                    }
                    KeyCode::Char(c) => input.buffer.push(c),
                    _ => {}
                },
                InputKind::EstimateEdit => match code {
                    KeyCode::Enter | KeyCode::Esc => {
                        // finish editing
                        self.input = None;
                    }
                    KeyCode::Up => {
                        self.day.adjust_estimate(self.selected, 5);
                    }
                    KeyCode::Down => {
                        self.day.adjust_estimate(self.selected, -5);
                    }
                    KeyCode::Char('k') => {
                        self.day.adjust_estimate(self.selected, 5);
                    }
                    KeyCode::Char('j') => {
                        self.day.adjust_estimate(self.selected, -5);
                    }
                    _ => {}
                },
                InputKind::ConfirmDelete => match code {
                    // Confirm via Enter or 'y'; cancel via Esc or 'n'
                    KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                        self.delete_selected();
                        self.input = None;
                    }
                    KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.input = None;
                    }
                    _ => {}
                },
            }
            return;
        }
        match code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char(':') => {
                // Open command palette
                self.input = Some(Input { kind: InputKind::Command, buffer: String::new() });
            }
            KeyCode::Char('E') => {
                // Enter estimate edit mode if a task is available
                if !self.day.tasks.is_empty() {
                    self.input =
                        Some(Input { kind: InputKind::EstimateEdit, buffer: String::new() });
                }
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
                // Open estimate edit mode
                if !self.day.tasks.is_empty() {
                    self.input =
                        Some(Input { kind: InputKind::EstimateEdit, buffer: String::new() });
                }
            }
            KeyCode::Char('p') => {
                self.postpone_selected();
            }
            KeyCode::Char('x') => {
                // Open delete confirmation on Today view with an existing task
                if self.view == View::Today && !self.day.tasks.is_empty() {
                    self.input = Some(Input { kind: InputKind::ConfirmDelete, buffer: String::new() });
                }
            }
            KeyCode::Char('b') => {
                self.bring_selected_from_future();
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
        // Only act on key press/repeat; ignore key release to avoid double-handling
        // on terminals reporting event types (iTerm2, Ghostty, etc.).
        if matches!(ev.kind, KeyEventKind::Release) {
            return;
        }
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
        if let Some(idx) = self.day.active_index() {
            let ymd = self.last_seen_ymd;
            self.day.finish_at(idx, ymd);
        }
    }

    /// Finish the currently selected task regardless of its active state.
    pub fn finish_selected(&mut self) {
        if self.day.tasks.is_empty() {
            return;
        }
        let idx = self.selected.min(self.day.tasks.len() - 1);
        let ymd = self.last_seen_ymd;
        self.day.finish_at(idx, ymd);
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
                // Now defined as "finish selected"
                self.finish_selected();
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
                // Repurpose to open estimate editor for backward compatibility with config name
                if !self.day.tasks.is_empty() {
                    self.input =
                        Some(Input { kind: InputKind::EstimateEdit, buffer: String::new() });
                }
            }
            A::Postpone => {
                self.postpone_selected();
            }
            A::BringToToday => {
                self.bring_selected_from_future();
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

    /// Bring a task from Future to Today (mirror of postpone). No-op unless in Future view.
    pub fn bring_selected_from_future(&mut self) {
        if self.view != View::Future || self.tomorrow.is_empty() {
            return;
        }
        let idx = self.selected.min(self.tomorrow.len() - 1);
        let task = self.tomorrow.remove(idx);
        // Ensure it becomes Planned in Today and append to the end.
        let t = Task { state: crate::task::TaskState::Planned, ..task };
        self.day.add_task(t);
        // Adjust selection within Future list
        if !self.tomorrow.is_empty() {
            self.selected = self.selected.min(self.tomorrow.len() - 1);
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

    // Input mode helpers for UI/tests
    pub fn in_input_mode(&self) -> bool {
        self.input.is_some()
    }
    pub fn input_buffer(&self) -> Option<&str> {
        self.input.as_ref().map(|i| i.buffer.as_str())
    }
    pub fn is_estimate_editing(&self) -> bool {
        matches!(self.input.as_ref().map(|i| i.kind), Some(InputKind::EstimateEdit))
    }
    pub fn is_command_mode(&self) -> bool {
        matches!(self.input.as_ref().map(|i| i.kind), Some(InputKind::Command))
    }
    pub fn is_confirm_delete(&self) -> bool {
        matches!(self.input.as_ref().map(|i| i.kind), Some(InputKind::ConfirmDelete))
    }
    pub fn selected_estimate(&self) -> Option<u16> {
        self.day.tasks.get(self.selected).map(|t| t.estimate_min)
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
        // Sweep when the local date changes
        let today = today_ymd();
        if today != self.last_seen_ymd {
            self.last_seen_ymd = today;
            self.sweep_done_before(today);
        }
        if let Some(active) = self.day.active_index() {
            if let Some(t) = self.day.tasks.get_mut(active) {
                t.actual_carry_sec = t.actual_carry_sec.saturating_add(seconds);
                while t.actual_carry_sec >= 60 {
                    t.actual_carry_sec -= 60;
                    t.actual_min = t.actual_min.saturating_add(1);
                }
            }
        }
    }
}

impl App {
    fn delete_selected(&mut self) {
        if self.view != View::Today || self.day.tasks.is_empty() {
            return;
        }
        let idx = self.selected.min(self.day.tasks.len() - 1);
        let _ = self.day.remove(idx);
        if !self.day.tasks.is_empty() {
            self.selected = self.selected.min(self.day.tasks.len() - 1);
        } else {
            self.selected = 0;
        }
    }

    /// Replace task lists from an external snapshot.
    /// - Resets selection and carry seconds; keeps config.
    pub fn apply_snapshot(
        &mut self,
        today: Vec<crate::task::Task>,
        future: Vec<crate::task::Task>,
        past: Vec<crate::task::Task>,
    ) {
        self.day = DayPlan::new(today);
        self.tomorrow = future;
        self.history = past;
        self.selected = 0;
        self.set_view(View::Today);
        // Ensure old done items are moved to Past on startup
        self.sweep_done_before(self.last_seen_ymd);
    }
}

impl App {
    /// Move any Today tasks with `done_ymd` strictly before `ymd` to history.
    pub fn sweep_done_before(&mut self, ymd: u32) {
        let mut i = 0;
        while i < self.day.tasks.len() {
            let move_to_past = matches!(self.day.tasks[i].done_ymd, Some(d) if d < ymd);
            if move_to_past {
                if let Some(task) = self.day.remove(i) {
                    self.history.push(task);
                }
                // don't increment i; elements shifted left
                continue;
            }
            i += 1;
        }
        // Clamp selection
        if !self.day.tasks.is_empty() {
            self.selected = self.selected.min(self.day.tasks.len() - 1);
        } else {
            self.selected = 0;
        }
    }

    fn apply_command(&mut self, cmd: &str) {
        // Supported: "est +15m", "est -5", "est 90m"
        let mut it = cmd.split_whitespace();
        let Some(head) = it.next() else {
            return;
        };
        if head != "est" {
            return;
        }
        let Some(arg) = it.next() else {
            return;
        };
        let s = arg.trim();
        if s.starts_with('+') || s.starts_with('-') {
            // relative delta
            let sign = if s.starts_with('+') { 1 } else { -1 };
            let num_part = s[1..].trim_end_matches('m');
            if let Ok(v) = num_part.parse::<i16>() {
                let delta = v.saturating_mul(sign);
                self.day.adjust_estimate(self.selected, delta);
            }
        } else {
            // absolute minutes
            let num_part = s.trim_end_matches('m');
            if let Ok(v) = num_part.parse::<u16>() {
                if let Some(t) = self.day.tasks.get_mut(self.selected) {
                    t.estimate_min = v;
                }
            }
        }
    }
}
