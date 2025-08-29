use crate::config::{Config, ESDBase, KeySpec};
use crate::task::{DayPlan, Task};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum KeyAction {
    Quit,
    Interrupt,
    StartOrResume,
    PauseActive,
    ReorderDown,
    ReorderUp,
    IncreaseEstimate,
    Postpone,
    NextView,
    PrevView,
    SelectUp,
    SelectDown,
    FinishActive,
}

#[derive(Default, Debug)]
struct KeyMap(HashMap<KeySpec, KeyAction>);

impl KeyMap {
    fn with_defaults() -> Self {
        use KeyAction::*;
        use KeyCode::*;
        use KeyModifiers as KM;
        let mut m = HashMap::new();
        let mut ins = |code: KeyCode, modifiers: KeyModifiers, act: KeyAction| {
            m.insert(KeySpec { code, modifiers }, act);
        };
        ins(Char('q'), KM::NONE, Quit);
        ins(Char('i'), KM::NONE, Interrupt);
        ins(Enter, KM::NONE, StartOrResume);
        ins(Char(' '), KM::NONE, PauseActive);
        ins(Char(']'), KM::NONE, ReorderDown);
        ins(Char('['), KM::NONE, ReorderUp);
        ins(Char('e'), KM::NONE, IncreaseEstimate);
        ins(Char('p'), KM::NONE, Postpone);
        ins(Tab, KM::NONE, NextView);
        ins(BackTab, KM::NONE, PrevView);
        ins(Up, KM::NONE, SelectUp);
        ins(Down, KM::NONE, SelectDown);
        ins(Char('k'), KM::NONE, SelectUp);
        ins(Char('j'), KM::NONE, SelectDown);
        ins(Enter, KM::SHIFT, FinishActive);
        KeyMap(m)
    }

    fn bind(&mut self, spec: KeySpec, action: KeyAction) {
        // remove any existing binding for this action
        let to_remove: Vec<KeySpec> =
            self.0.iter().filter_map(|(k, v)| if *v == action { Some(*k) } else { None }).collect();
        for k in to_remove {
            self.0.remove(&k);
        }
        self.0.insert(spec, action);
    }

    fn action_for(&self, ev: KeyEvent) -> Option<KeyAction> {
        let spec = KeySpec { code: ev.code, modifiers: ev.modifiers };
        self.0.get(&spec).copied()
    }
}

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
    config: Config,
    keymap: KeyMap,
}

impl App {
    pub fn new() -> Self {
        let cfg = Config::load().unwrap_or_default();
        let mut keymap = KeyMap::with_defaults();
        // Overlay custom key bindings from config
        Self::apply_config_keys(&mut keymap, &cfg);
        Self {
            title: "Chute_kun".to_string(),
            should_quit: false,
            day: DayPlan::new(vec![]),
            selected: 0,
            tomorrow: vec![],
            history: vec![],
            view: View::default(),
            active_accum_sec: 0,
            config: cfg,
            keymap,
        }
    }

    fn apply_config_keys(map: &mut KeyMap, cfg: &Config) {
        use KeyAction::*;
        let maybe = |name: &str| cfg.key_for(name);
        if let Some(s) = maybe("quit") {
            map.bind(s, Quit)
        }
        if let Some(s) = maybe("interrupt") {
            map.bind(s, Interrupt)
        }
        if let Some(s) = maybe("start_resume") {
            map.bind(s, StartOrResume)
        }
        if let Some(s) = maybe("pause") {
            map.bind(s, PauseActive)
        }
        if let Some(s) = maybe("reorder_down") {
            map.bind(s, ReorderDown)
        }
        if let Some(s) = maybe("reorder_up") {
            map.bind(s, ReorderUp)
        }
        if let Some(s) = maybe("increase_estimate") {
            map.bind(s, IncreaseEstimate)
        }
        if let Some(s) = maybe("postpone") {
            map.bind(s, Postpone)
        }
        if let Some(s) = maybe("next_view") {
            map.bind(s, NextView)
        }
        if let Some(s) = maybe("prev_view") {
            map.bind(s, PrevView)
        }
        if let Some(s) = maybe("select_up") {
            map.bind(s, SelectUp)
        }
        if let Some(s) = maybe("select_down") {
            map.bind(s, SelectDown)
        }
        if let Some(s) = maybe("finish_active") {
            map.bind(s, FinishActive)
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        let ev = KeyEvent::new(code, KeyModifiers::NONE);
        self.handle_key_event(ev);
    }

    pub fn handle_key_event(&mut self, ev: KeyEvent) {
        if let Some(action) = self.keymap.action_for(ev) {
            self.apply_action(action);
        }
    }

    fn apply_action(&mut self, action: KeyAction) {
        use KeyAction::*;
        match action {
            Quit => {
                self.should_quit = true;
            }
            Interrupt => {
                let idx = self.add_task("Interrupt", 15);
                self.day.start(idx);
                self.active_accum_sec = 0;
            }
            StartOrResume => {
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
            PauseActive => {
                self.day.pause_active();
            }
            ReorderDown => {
                let new = self.day.reorder_down(self.selected);
                self.selected = new;
            }
            ReorderUp => {
                let new = self.day.reorder_up(self.selected);
                self.selected = new;
            }
            IncreaseEstimate => {
                self.day.adjust_estimate(self.selected, 5);
            }
            Postpone => {
                self.postpone_selected();
            }
            NextView => {
                self.set_view(self.view.next());
            }
            PrevView => {
                self.set_view(self.view.prev());
            }
            SelectUp => self.select_up(),
            SelectDown => self.select_down(),
            FinishActive => self.finish_active(),
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

    pub fn schedule_start_minute_from(&self, now_min: u16) -> u16 {
        self.config.start_of_day_min.unwrap_or(9 * 60)
    }

    pub fn esd_base_minute_from(&self, now_min: u16) -> u16 {
        match (self.config.esd_base, self.config.start_of_day_min) {
            (ESDBase::StartOfDay, Some(min)) => min,
            (ESDBase::StartOfDay, None) => 9 * 60,
            _ => now_min,
        }
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
