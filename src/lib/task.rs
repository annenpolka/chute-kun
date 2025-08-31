use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Category {
    #[default]
    General,
    Work,
    Home,
    Hobby,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub start_min: u16,
    #[serde(default)]
    pub end_min: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    Planned,
    Active,
    Paused,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub estimate_min: u16,
    pub actual_min: u16,
    /// Partially accumulated seconds (<60) toward `actual_min` for this task.
    #[serde(default)]
    pub actual_carry_sec: u16,
    /// Actual first start time in minutes since local midnight.
    /// Recorded when the task first transitions to Active.
    #[serde(default)]
    pub started_at_min: Option<u16>,
    /// Actual finish time in minutes since local midnight.
    /// Recorded when the task is marked Done.
    #[serde(default)]
    pub finished_at_min: Option<u16>,
    /// Full session log for this task (start/end pairs). The last session
    /// may have `end_min=None` while the task is Active.
    #[serde(default)]
    pub sessions: Vec<Session>,
    pub state: TaskState,
    /// Planned date for the task (YYYYMMDD). Defaults to today on creation.
    /// Used to support date selection on creation and postpone/bring operations.
    #[serde(default)]
    pub planned_ymd: u32,
    #[serde(default)]
    pub done_ymd: Option<u32>,
    /// User classification for the task (e.g., Work/Home/Hobby). Defaults to General.
    #[serde(default)]
    pub category: Category,
    /// Optional fixed planned start time in minutes since local midnight.
    /// When present, the visual schedule uses `max(cursor, fixed_start_min)` at this task
    /// and pushes the subsequent tasks based on estimates from there.
    #[serde(default)]
    pub fixed_start_min: Option<u16>,
}

impl Task {
    pub fn new(title: &str, estimate_min: u16) -> Self {
        Self {
            title: title.to_string(),
            estimate_min,
            actual_min: 0,
            actual_carry_sec: 0,
            started_at_min: None,
            finished_at_min: None,
            sessions: Vec::new(),
            state: TaskState::Planned,
            planned_ymd: crate::date::today_ymd(),
            done_ymd: None,
            category: Category::General,
            fixed_start_min: None,
        }
    }
}

impl Task {
    pub fn start_session(&mut self, now_min: u16) {
        let need_new = self.sessions.last().is_none_or(|s| s.end_min.is_some());
        if need_new {
            self.sessions.push(Session { start_min: now_min, end_min: None });
        }
    }
    pub fn end_session(&mut self, now_min: u16) {
        if let Some(last) = self.sessions.last_mut() {
            if last.end_min.is_none() {
                last.end_min = Some(now_min);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct DayPlan {
    pub tasks: Vec<Task>,
    active: Option<usize>,
}

impl DayPlan {
    pub fn new(tasks: Vec<Task>) -> Self {
        let active = tasks.iter().position(|t| matches!(t.state, TaskState::Active));
        Self { tasks, active }
    }

    pub fn active_index(&self) -> Option<usize> {
        self.active
    }

    pub fn add_task(&mut self, task: Task) -> usize {
        self.tasks.push(task);
        self.tasks.len() - 1
    }

    // start or activate a task at index, pausing any existing active task
    pub fn start(&mut self, index: usize) {
        if let Some(cur) = self.active {
            if cur != index {
                if let Some(t) = self.tasks.get_mut(cur) {
                    t.state = TaskState::Paused;
                }
            } else {
                // already active, nothing to do
                return;
            }
        }
        if let Some(t) = self.tasks.get_mut(index) {
            t.state = TaskState::Active;
            self.active = Some(index);
        }
    }

    pub fn pause_active(&mut self) {
        if let Some(cur) = self.active.take() {
            if let Some(t) = self.tasks.get_mut(cur) {
                t.state = TaskState::Paused;
            }
        }
    }

    /// Mark the task at `index` as Done with the given date (YYYYMMDD).
    /// - If it was the active task, clear active.
    pub fn finish_at(&mut self, index: usize, today_ymd: u32) {
        if index >= self.tasks.len() {
            return;
        }
        if self.active == Some(index) {
            self.active = None;
            if let Some(t) = self.tasks.get_mut(index) {
                t.state = TaskState::Done;
                t.done_ymd = Some(today_ymd);
            }
        } else if let Some(t) = self.tasks.get_mut(index) {
            t.state = TaskState::Done;
            t.done_ymd = Some(today_ymd);
        }
    }

    pub fn add_actual_to_active(&mut self, minutes: u16) {
        if let Some(cur) = self.active {
            if let Some(t) = self.tasks.get_mut(cur) {
                t.actual_min = t.actual_min.saturating_add(minutes);
            }
        }
    }

    pub fn remaining_total_min(&self) -> u16 {
        self.tasks
            .iter()
            .map(|t| match t.state {
                TaskState::Done => 0,
                _ => t.estimate_min.saturating_sub(t.actual_min),
            })
            .sum()
    }

    pub fn esd(&self, now_min: u16) -> u16 {
        // Sum estimates of non-done tasks (decoupled from progress)
        let est_sum: u16 = self
            .tasks
            .iter()
            .map(|t| match t.state {
                TaskState::Done => 0,
                _ => t.estimate_min,
            })
            .sum();
        // Base time: latest measured finish (task finish or session end) or now, whichever is later
        let base = match self.latest_actual_finish_min() {
            Some(last) => last.max(now_min),
            None => now_min,
        };
        esd_from(base, &[est_sum])
    }

    /// Latest actual finish minute across all tasks for today.
    fn latest_actual_finish_min(&self) -> Option<u16> {
        let mut max_end: Option<u16> = None;
        for t in &self.tasks {
            if let Some(f) = t.finished_at_min {
                max_end = Some(max_end.map_or(f, |m| m.max(f)));
            }
            if let Some(e) = t.sessions.iter().filter_map(|s| s.end_min).max() {
                max_end = Some(max_end.map_or(e, |m| m.max(e)));
            }
        }
        max_end
    }

    pub fn reorder_down(&mut self, index: usize) -> usize {
        if index + 1 >= self.tasks.len() {
            return index;
        }
        self.tasks.swap(index, index + 1);
        if let Some(a) = self.active.as_mut() {
            if *a == index {
                *a = index + 1;
            } else if *a == index + 1 {
                *a = index;
            }
        }
        index + 1
    }

    pub fn reorder_up(&mut self, index: usize) -> usize {
        if index == 0 || index >= self.tasks.len() {
            return index;
        }
        self.tasks.swap(index - 1, index);
        if let Some(a) = self.active.as_mut() {
            if *a == index {
                *a = index - 1;
            } else if *a == index - 1 {
                *a = index;
            }
        }
        index - 1
    }

    /// Move task from `from` index to logical position `to` (clamped to list bounds).
    /// Returns the new index of the moved task. Adjusts `active` pointer accordingly.
    pub fn move_index(&mut self, from: usize, to_slot: usize) -> usize {
        let len = self.tasks.len();
        if len == 0 || from >= len {
            return from;
        }
        // Allow insertion at end by accepting to_slot == len
        let slot = to_slot.min(len);
        // Desired final index after move in the resulting vector
        let dest_final = if from < slot { slot - 1 } else { slot };
        if dest_final == from {
            return from;
        }
        let item = self.tasks.remove(from);
        self.tasks.insert(dest_final, item);
        // Fix active pointer relative to move span
        if let Some(a) = self.active.as_mut() {
            if *a == from {
                *a = dest_final;
            } else if from < dest_final {
                // [from+1 ..= dest_final] shifted left by 1
                if *a > from && *a <= dest_final {
                    *a = a.saturating_sub(1);
                }
            } else {
                // [dest_final .. from-1] shifted right by 1
                if *a >= dest_final && *a < from {
                    *a = a.saturating_add(1);
                }
            }
        }
        dest_final
    }

    pub fn adjust_estimate(&mut self, index: usize, delta_min: i16) {
        if let Some(t) = self.tasks.get_mut(index) {
            let cur = t.estimate_min as i16 + delta_min;
            t.estimate_min = cur.max(0) as u16;
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Task> {
        if index >= self.tasks.len() {
            return None;
        }
        // fix active pointer
        if let Some(a) = self.active {
            if a == index {
                self.active = None;
            } else if a > index {
                self.active = Some(a - 1);
            }
        }
        Some(self.tasks.remove(index))
    }
}

// ESD(見込み終了時刻) = now(min) + 残分合計
pub fn esd_from(now_min: u16, remaining_mins: &[u16]) -> u16 {
    let sum: u16 = remaining_mins.iter().copied().sum();
    now_min.saturating_add(sum)
}

pub fn tc_log_line(task: &Task) -> String {
    let state = match task.state {
        TaskState::Planned => "Planned",
        TaskState::Active => "Active",
        TaskState::Paused => "Paused",
        TaskState::Done => "Done",
    };
    format!(
        "tc-log | {} | act:{}m | est:{}m | state:{}",
        task.title, task.actual_min, task.estimate_min, state
    )
}
