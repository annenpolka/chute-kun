use crate::config::Config;
use crate::date::today_ymd;
use crate::task::{DayPlan, Task};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use std::time::Instant;

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
    // Mouse UI state
    hovered: Option<usize>,
    hovered_tab: Option<usize>,
    popup_hover: Option<PopupButton>,
    last_click: Option<LastClick>,
    // Drag state for list reordering
    drag_from: Option<usize>,
    // Pulse toggle for simple UI animation effects
    pulse: bool,
    // Two-step task creation: after title input, prompt for estimate
    new_task: Option<NewTaskDraft>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputKind {
    Normal,
    Interrupt,
    Command,
    EstimateEdit,
    NewTaskEstimate,
    ConfirmDelete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    kind: InputKind,
    buffer: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NewTaskDraft {
    source: InputKind, // Normal or Interrupt
    title: String,
    default_estimate: u16,
}

impl App {
    fn handle_mouse_in_popup(&mut self, ev: MouseEvent, area: Rect) {
        // Delete confirmation
        if self.is_confirm_delete() {
            if let Some(popup) = crate::ui::compute_delete_popup_rect(self, area) {
                let (del_btn, cancel_btn) = crate::ui::delete_popup_button_hitboxes(self, popup);
                match ev.kind {
                    MouseEventKind::Moved => {
                        let pos = (ev.column, ev.row);
                        self.popup_hover = if point_in_rect(pos.0, pos.1, del_btn) {
                            Some(PopupButton::Delete)
                        } else if point_in_rect(pos.0, pos.1, cancel_btn) {
                            Some(PopupButton::Cancel)
                        } else {
                            None
                        };
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        let pos = (ev.column, ev.row);
                        if point_in_rect(pos.0, pos.1, del_btn) {
                            self.delete_selected();
                            self.input = None;
                        } else if point_in_rect(pos.0, pos.1, cancel_btn) {
                            self.input = None;
                        }
                    }
                    _ => {}
                }
            }
        } else if self.is_estimate_editing() {
            // Estimate editor (slider)
            if let Some(popup) = crate::ui::compute_estimate_popup_rect(self, area) {
                let (track, ok, cancel) = crate::ui::estimate_slider_hitboxes(self, popup);
                match ev.kind {
                    MouseEventKind::Moved => {
                        let pos = (ev.column, ev.row);
                        self.popup_hover = if point_in_rect(pos.0, pos.1, ok) {
                            Some(PopupButton::EstOk)
                        } else if point_in_rect(pos.0, pos.1, cancel) {
                            Some(PopupButton::EstCancel)
                        } else {
                            None
                        };
                    }
                    MouseEventKind::Down(MouseButton::Left)
                    | MouseEventKind::Drag(MouseButton::Left) => {
                        let pos = (ev.column, ev.row);
                        if point_in_rect(pos.0, pos.1, track) {
                            let m = crate::ui::minutes_from_slider_x(track, 0, 240, 5, pos.0);
                            if let Some(t) = self.day.tasks.get_mut(self.selected) {
                                t.estimate_min = m;
                            }
                        } else if point_in_rect(pos.0, pos.1, ok)
                            || point_in_rect(pos.0, pos.1, cancel)
                        {
                            self.input = None;
                        }
                    }
                    _ => {}
                }
            }
        } else if self.is_new_task_estimate() {
            if let Some(popup) = crate::ui::compute_new_task_estimate_popup_rect(self, area) {
                let (add, cancel) = crate::ui::input_popup_button_hitboxes(self, popup);
                match ev.kind {
                    MouseEventKind::Moved => {
                        let pos = (ev.column, ev.row);
                        self.popup_hover = if point_in_rect(pos.0, pos.1, add) {
                            Some(PopupButton::InputAdd)
                        } else if point_in_rect(pos.0, pos.1, cancel) {
                            Some(PopupButton::InputCancel)
                        } else {
                            None
                        };
                    }
                    MouseEventKind::Down(MouseButton::Left)
                    | MouseEventKind::Drag(MouseButton::Left) => {
                        let pos = (ev.column, ev.row);
                        // Allow clicking on the slider track to set value
                        let (track, _ok2, _cancel2) =
                            crate::ui::estimate_slider_hitboxes(self, popup);
                        if point_in_rect(pos.0, pos.1, track) {
                            let m = crate::ui::minutes_from_slider_x(track, 0, 240, 5, pos.0);
                            if let Some(inp) = self.input.as_mut() {
                                inp.buffer = m.to_string();
                            }
                        } else if point_in_rect(pos.0, pos.1, add) {
                            self.handle_key(KeyCode::Enter);
                        } else if point_in_rect(pos.0, pos.1, cancel) {
                            self.new_task = None;
                            self.input = None;
                        }
                    }
                    _ => {}
                }
            }
        } else if self.is_command_mode() {
            if let Some(popup) = crate::ui::compute_command_popup_rect(self, area) {
                let (run, cancel) = crate::ui::command_popup_button_hitboxes(self, popup);
                match ev.kind {
                    MouseEventKind::Moved => {
                        let pos = (ev.column, ev.row);
                        self.popup_hover = if point_in_rect(pos.0, pos.1, run) {
                            Some(PopupButton::InputAdd)
                        } else if point_in_rect(pos.0, pos.1, cancel) {
                            Some(PopupButton::InputCancel)
                        } else {
                            None
                        };
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        let pos = (ev.column, ev.row);
                        if point_in_rect(pos.0, pos.1, run) {
                            self.handle_key(KeyCode::Enter);
                        } else if point_in_rect(pos.0, pos.1, cancel) {
                            self.input = None;
                        }
                    }
                    _ => {}
                }
            }
        } else if self.in_input_mode() && !self.is_command_mode() {
            // Task name input (Normal/Interrupt)
            if let Some(popup) = crate::ui::compute_input_popup_rect(self, area) {
                let (add, cancel) = crate::ui::input_popup_button_hitboxes(self, popup);
                match ev.kind {
                    MouseEventKind::Moved => {
                        let pos = (ev.column, ev.row);
                        self.popup_hover = if point_in_rect(pos.0, pos.1, add) {
                            Some(PopupButton::InputAdd)
                        } else if point_in_rect(pos.0, pos.1, cancel) {
                            Some(PopupButton::InputCancel)
                        } else {
                            None
                        };
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        let pos = (ev.column, ev.row);
                        if point_in_rect(pos.0, pos.1, add) {
                            // Reuse key handling to submit
                            self.handle_key(KeyCode::Enter);
                        } else if point_in_rect(pos.0, pos.1, cancel) {
                            self.input = None;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    pub fn new() -> Self {
        if std::env::var("RUST_TEST_THREADS").is_ok() {
            Self::with_config(Config::default())
        } else {
            Self::with_config(Config::load())
        }
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
            hovered: None,
            hovered_tab: None,
            popup_hover: None,
            last_click: None,
            drag_from: None,
            pulse: false,
            new_task: None,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        // If we are in input mode, interpret keys as text editing/submit/cancel
        if let Some(input) = self.input.as_mut() {
            // Pre-capture default estimate for new-task flow to avoid borrowing self during edits
            let new_task_default = self.new_task.as_ref().map(|d| d.default_estimate).unwrap_or(25);
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
                        // Move to estimate entry step with default prefilled
                        self.new_task =
                            Some(NewTaskDraft { source: input.kind, title, default_estimate: est });
                        self.input =
                            Some(Input { kind: InputKind::NewTaskEstimate, buffer: String::new() });
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
                InputKind::NewTaskEstimate => match code {
                    KeyCode::Enter => {
                        if let Some(draft) = self.new_task.take() {
                            let cur = self
                                .input
                                .as_ref()
                                .and_then(|i| i.buffer.trim().parse::<u16>().ok())
                                .unwrap_or(draft.default_estimate);
                            let idx = self.add_task(&draft.title, cur);
                            self.selected = idx;
                        }
                        self.input = None;
                    }
                    KeyCode::Esc => {
                        // Cancel entire creation flow
                        self.new_task = None;
                        self.input = None;
                    }
                    KeyCode::Backspace => {
                        input.buffer.pop();
                    }
                    KeyCode::Up | KeyCode::Right | KeyCode::Char('k') => {
                        let base =
                            input.buffer.trim().parse::<u16>().ok().unwrap_or(new_task_default);
                        let next = base.saturating_add(5).min(240);
                        input.buffer = next.to_string();
                    }
                    KeyCode::Down | KeyCode::Left | KeyCode::Char('j') => {
                        let base =
                            input.buffer.trim().parse::<u16>().ok().unwrap_or(new_task_default);
                        let next = base.saturating_sub(5);
                        input.buffer = next.to_string();
                    }
                    KeyCode::Char(_c) => {}
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
                    KeyCode::Right => {
                        self.day.adjust_estimate(self.selected, 5);
                    }
                    KeyCode::Left => {
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
                if let Some(active_idx) = self.day.active_index() {
                    // End the running session before pausing
                    let now = crate::clock::system_now_minutes();
                    if let Some(t) = self.day.tasks.get_mut(active_idx) {
                        t.end_session(now);
                    }
                    self.day.pause_active();
                } else {
                    let s = self.selected;
                    let eligible = matches!(
                        self.day.tasks.get(s).map(|t| t.state),
                        Some(crate::task::TaskState::Paused | crate::task::TaskState::Planned)
                    );
                    if eligible {
                        self.day.start(s);
                        // Record actual start time if first activation
                        if let Some(t) = self.day.tasks.get_mut(s) {
                            if t.started_at_min.is_none() {
                                t.started_at_min = Some(crate::clock::system_now_minutes());
                            }
                            let now = crate::clock::system_now_minutes();
                            t.start_session(now);
                        }
                    } else if let Some(idx) = (0..self.day.tasks.len()).find(|&i| {
                        matches!(
                            self.day.tasks[i].state,
                            crate::task::TaskState::Paused | crate::task::TaskState::Planned
                        )
                    }) {
                        self.day.start(idx);
                        self.selected = idx;
                        if let Some(t) = self.day.tasks.get_mut(idx) {
                            if t.started_at_min.is_none() {
                                t.started_at_min = Some(crate::clock::system_now_minutes());
                            }
                            let now = crate::clock::system_now_minutes();
                            t.start_session(now);
                        }
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
                    self.input =
                        Some(Input { kind: InputKind::ConfirmDelete, buffer: String::new() });
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

    /// Handle mouse events using the current terminal area to map coordinates to UI regions.
    /// - Left click on a list row selects that task; double-click toggles start/pause.
    /// - Right click on a list row opens estimate edit.
    /// - Clicking tabs switches views.
    /// - Mouse move updates hover index.
    /// - Ignores clicks while in input/popup modes.
    pub fn handle_mouse_event(&mut self, ev: MouseEvent, area: Rect) {
        if self.in_input_mode() || self.is_confirm_delete() {
            self.handle_mouse_in_popup(ev, area);
            return;
        }
        let (tabs, _banner, list, _help) = crate::ui::compute_layout(self, area);
        match ev.kind {
            MouseEventKind::Moved => {
                self.update_hover_from_coords(ev.column, ev.row, list);
                // Tab hover
                if ev.row == tabs.y {
                    let boxes = crate::ui::tab_hitboxes(self, tabs);
                    let mut hit = None;
                    for (i, r) in boxes.iter().enumerate() {
                        if point_in_rect(ev.column, ev.row, *r) {
                            hit = Some(i);
                            break;
                        }
                    }
                    self.hovered_tab = hit;
                } else {
                    self.hovered_tab = None;
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                // Tabs click
                if ev.row == tabs.y {
                    let boxes = crate::ui::tab_hitboxes(self, tabs);
                    let mut clicked = None;
                    for (i, r) in boxes.iter().enumerate() {
                        if point_in_rect(ev.column, ev.row, *r) {
                            clicked = Some(i);
                            break;
                        }
                    }
                    match clicked {
                        Some(0) => self.set_view(View::Past),
                        Some(1) => self.set_view(View::Today),
                        Some(2) => self.set_view(View::Future),
                        _ => {}
                    }
                    // View change can shift list area (different banner/help height). Re-align hover
                    let (_t2, _b2, list2, _h2) = crate::ui::compute_layout(self, area);
                    self.update_hover_from_coords(ev.column, ev.row, list2);
                    self.drag_from = None;
                    return;
                }
                // List click (ignore header at list.y)
                if ev.row <= list.y || ev.row >= list.y.saturating_add(list.height) {
                    self.drag_from = None;
                    return;
                }
                let idx = self.index_from_list_row(ev.row, list);
                self.selected = idx;
                // Begin potential drag reorder in Today/Future lists
                self.drag_from = match self.view {
                    View::Today | View::Future => Some(idx),
                    View::Past => None,
                };
                // Detect double-click
                let now = Instant::now();
                const THRESHOLD_MS: u128 = 600;
                let is_double = matches!(
                    self.last_click,
                    Some(LastClick { when, index, button: MouseButton::Left })
                        if index == idx && now.duration_since(when).as_millis() <= THRESHOLD_MS
                );
                if is_double {
                    match self.view {
                        View::Today => self.toggle_task_start_pause(idx),
                        View::Future => {
                            // Bring from Future to Today but do not auto-start
                            self.bring_selected_from_future();
                        }
                        View::Past => {}
                    }
                    // Reset so the second Down of the pair doesn't trigger again
                    self.last_click = None;
                } else {
                    self.last_click =
                        Some(LastClick { when: now, index: idx, button: MouseButton::Left });
                }
                // Starting/pausing can add/remove the active banner which shifts list Y by ±1.
                // Recompute hover using the current coordinates against the new layout.
                let (_t2, _b2, list2, _h2) = crate::ui::compute_layout(self, area);
                self.update_hover_from_coords(ev.column, ev.row, list2);
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                // While dragging, update hover to provide visual guidance
                self.update_hover_from_coords(ev.column, ev.row, list);
            }
            MouseEventKind::Up(MouseButton::Left) => {
                // Finalize a drag-reorder if one started inside the list
                if let Some(from) = self.drag_from.take() {
                    // Compute drop target from current row; insert after the hovered row
                    // when dragging downward, and before when dragging upward.
                    let hover = self.index_from_list_row(ev.row, list);
                    let slot = if from < hover { hover.saturating_add(1) } else { hover };
                    match self.view {
                        View::Today => {
                            let new = self.day.move_index(from, slot);
                            self.selected = new;
                        }
                        View::Future => {
                            let new = self.move_future_index(from, slot);
                            self.selected = new;
                        }
                        View::Past => {}
                    }
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                // Right click opens estimate editor on the clicked row (ignore header)
                if ev.row <= list.y || ev.row >= list.y.saturating_add(list.height) {
                    return;
                }
                let idx = self.index_from_list_row(ev.row, list);
                self.selected = idx;
                if !self.day.tasks.is_empty() {
                    self.input =
                        Some(Input { kind: InputKind::EstimateEdit, buffer: String::new() });
                }
                // Popups don't affect list geometry, but realign hover defensively.
                let (_t2, _b2, list2, _h2) = crate::ui::compute_layout(self, area);
                self.update_hover_from_coords(ev.column, ev.row, list2);
            }
            _ => {}
        }
    }

    /// For tests: update hover by coordinates without constructing a MouseEvent.
    pub fn handle_mouse_move(&mut self, col: u16, row: u16, area: Rect) {
        let (tabs, _banner, list, _help) = crate::ui::compute_layout(self, area);
        self.update_hover_from_coords(col, row, list);
        // Tabs hover
        if row == tabs.y {
            let boxes = crate::ui::tab_hitboxes(self, tabs);
            self.hovered_tab = boxes
                .iter()
                .enumerate()
                .find(|(_, r)| point_in_rect(col, row, **r))
                .map(|(i, _)| i);
        } else {
            self.hovered_tab = None;
        }
        // Popup hover
        if self.is_confirm_delete() {
            if let Some(popup) = crate::ui::compute_delete_popup_rect(self, area) {
                let (del_btn, cancel_btn) = crate::ui::delete_popup_button_hitboxes(self, popup);
                if point_in_rect(col, row, del_btn) {
                    self.popup_hover = Some(PopupButton::Delete);
                } else if point_in_rect(col, row, cancel_btn) {
                    self.popup_hover = Some(PopupButton::Cancel);
                } else {
                    self.popup_hover = None;
                }
            }
        }
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
            // Use current date at the moment of finishing to respect test overrides
            // set via CHUTE_KUN_TODAY. Avoid relying on cached last_seen_ymd here
            // because tests may set the env var after App initialization.
            let ymd = crate::date::today_ymd();
            let now = crate::clock::system_now_minutes();
            if let Some(t) = self.day.tasks.get_mut(idx) {
                t.finished_at_min = Some(now);
                t.end_session(now);
            }
            self.day.finish_at(idx, ymd);
        }
    }

    /// Finish the currently selected task regardless of its active state.
    pub fn finish_selected(&mut self) {
        if self.day.tasks.is_empty() {
            return;
        }
        let idx = self.selected.min(self.day.tasks.len() - 1);
        // Use current date at the moment of finishing for the same reason as above.
        let ymd = crate::date::today_ymd();
        let now = crate::clock::system_now_minutes();
        if let Some(t) = self.day.tasks.get_mut(idx) {
            t.finished_at_min = Some(now);
            t.end_session(now);
        }
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
                if let Some(active_idx) = self.day.active_index() {
                    // End current session before pausing
                    let now = crate::clock::system_now_minutes();
                    if let Some(t) = self.day.tasks.get_mut(active_idx) {
                        t.end_session(now);
                    }
                    self.day.pause_active();
                } else {
                    let s = self.selected;
                    let eligible = matches!(
                        self.day.tasks.get(s).map(|t| t.state),
                        Some(crate::task::TaskState::Paused | crate::task::TaskState::Planned)
                    );
                    if eligible {
                        self.day.start(s);
                        // Record actual start time if first activation and open a new session
                        if let Some(t) = self.day.tasks.get_mut(s) {
                            let now = crate::clock::system_now_minutes();
                            if t.started_at_min.is_none() {
                                t.started_at_min = Some(now);
                            }
                            t.start_session(now);
                        }
                    } else if let Some(idx) = (0..self.day.tasks.len()).find(|&i| {
                        matches!(
                            self.day.tasks[i].state,
                            crate::task::TaskState::Paused | crate::task::TaskState::Planned
                        )
                    }) {
                        self.day.start(idx);
                        self.selected = idx;
                        if let Some(t) = self.day.tasks.get_mut(idx) {
                            let now = crate::clock::system_now_minutes();
                            if t.started_at_min.is_none() {
                                t.started_at_min = Some(now);
                            }
                            t.start_session(now);
                        }
                    }
                }
            }
            A::FinishActive => {
                // Now defined as "finish selected"
                self.finish_selected();
            }
            A::Pause => {
                if let Some(idx) = self.day.active_index() {
                    let now = crate::clock::system_now_minutes();
                    if let Some(t) = self.day.tasks.get_mut(idx) {
                        t.end_session(now);
                    }
                }
                self.day.pause_active();
            }
            A::Delete => {
                if self.view == View::Today && !self.day.tasks.is_empty() {
                    self.input =
                        Some(Input { kind: InputKind::ConfirmDelete, buffer: String::new() });
                }
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
        let new = self.selected.saturating_sub(1);
        if new != self.selected {
            self.selected = new;
        }
    }
    pub fn select_down(&mut self) {
        let len = self.current_len();
        if len == 0 {
            return;
        }
        let last = len - 1;
        let new = (self.selected + 1).min(last);
        if new != self.selected {
            self.selected = new;
        }
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
    pub fn is_new_task_estimate(&self) -> bool {
        matches!(self.input.as_ref().map(|i| i.kind), Some(InputKind::NewTaskEstimate))
    }
    pub fn is_command_mode(&self) -> bool {
        matches!(self.input.as_ref().map(|i| i.kind), Some(InputKind::Command))
    }
    pub fn is_confirm_delete(&self) -> bool {
        matches!(self.input.as_ref().map(|i| i.kind), Some(InputKind::ConfirmDelete))
    }
    /// True only when typing a task title (Normal/Interrupt), not for estimate/confirm popups.
    pub fn is_text_input_mode(&self) -> bool {
        matches!(
            self.input.as_ref().map(|i| i.kind),
            Some(InputKind::Normal | InputKind::Interrupt)
        )
    }
    pub fn selected_estimate(&self) -> Option<u16> {
        self.day.tasks.get(self.selected).map(|t| t.estimate_min)
    }
    pub fn new_task_title(&self) -> Option<&str> {
        self.new_task.as_ref().map(|d| d.title.as_str())
    }
    pub fn new_task_default_estimate(&self) -> Option<u16> {
        self.new_task.as_ref().map(|d| d.default_estimate)
    }
    pub fn hovered_index(&self) -> Option<usize> {
        self.hovered
    }
    pub fn is_dragging(&self) -> bool {
        self.drag_from.is_some()
    }
    pub fn drag_source_index(&self) -> Option<usize> {
        if self.view == View::Today {
            self.drag_from
        } else {
            None
        }
    }
    pub fn pulse_on(&self) -> bool {
        self.pulse
    }
    pub fn hovered_tab_index(&self) -> Option<usize> {
        self.hovered_tab
    }
    pub fn popup_hover_button(&self) -> Option<PopupButton> {
        self.popup_hover
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
        self.hovered = None;
    }

    fn current_len(&self) -> usize {
        match self.view {
            View::Past => self.history.len(),
            View::Today => self.day.tasks.len(),
            View::Future => self.tomorrow.len(),
        }
    }

    pub fn tick(&mut self, seconds: u16) {
        // Freeze app time updates while a confirmation popup is open
        if self.is_confirm_delete() {
            return;
        }
        // Simple pulse animation toggle
        if seconds > 0 {
            self.pulse = !self.pulse;
        }
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
        // Supported:
        // - "est +15m" / "est -5" / "est 90m" (estimate edit)
        // - "base HH:MM" (予定の基準時刻 = day_start を変更)
        let mut it = cmd.split_whitespace();
        let Some(head) = it.next() else {
            return;
        };
        match head {
            "est" => {
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
            "base" => {
                if let Some(arg) = it.next() {
                    if let Ok((h, m)) = crate::config::parse_hhmm_or_compact(arg) {
                        // Update in-memory immediately
                        self.config.day_start_minutes = h * 60 + m;
                        // Persist by default
                        let _ = crate::config::write_day_start(h, m);
                    }
                }
            }
            _ => {}
        }
    }
}

impl App {
    fn toggle_task_start_pause(&mut self, idx: usize) {
        if self.day.active_index() == Some(idx) {
            self.day.pause_active();
        } else {
            self.day.start(idx);
            self.selected = idx;
        }
    }

    fn index_from_list_row(&self, row: u16, list: Rect) -> usize {
        // Table uses a header row at list.y; first data row starts at list.y + 1.
        // Map mouse row to data index accordingly.
        let rel = row.saturating_sub(list.y.saturating_add(1)) as usize;
        let len = self.current_len();
        rel.min(len.saturating_sub(1))
    }

    fn update_hover_from_coords(&mut self, col: u16, row: u16, list: Rect) {
        // Only treat rows strictly below the header as list items
        if row >= list.y.saturating_add(1)
            && row < list.y.saturating_add(list.height)
            && col >= list.x
            && col < list.x + list.width
        {
            let idx = self.index_from_list_row(row, list);
            self.hovered = Some(idx);
        } else {
            self.hovered = None;
        }
    }
}

impl App {
    /// Reorder tasks within the Future (tomorrow) list using an insertion slot model.
    /// Returns the final index of the moved task in the Future list.
    fn move_future_index(&mut self, from: usize, to_slot: usize) -> usize {
        let len = self.tomorrow.len();
        if len == 0 || from >= len {
            return from;
        }
        let slot = to_slot.min(len);
        let dest_final = if from < slot { slot.saturating_sub(1) } else { slot };
        if dest_final == from {
            return from;
        }
        let item = self.tomorrow.remove(from);
        self.tomorrow.insert(dest_final, item);
        dest_final
    }
}

#[derive(Clone, Copy, Debug)]
struct LastClick {
    when: Instant,
    index: usize,
    button: MouseButton,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopupButton {
    Delete,
    Cancel,
    EstMinus,
    EstPlus,
    EstOk,
    EstCancel,
    InputAdd,
    InputCancel,
}

fn point_in_rect(x: u16, y: u16, r: Rect) -> bool {
    x >= r.x && x < r.x.saturating_add(r.width) && y >= r.y && y < r.y.saturating_add(r.height)
}
