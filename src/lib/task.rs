use serde::{Deserialize, Serialize};

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
    pub state: TaskState,
}

impl Task {
    pub fn new(title: &str, estimate_min: u16) -> Self {
        Self {
            title: title.to_string(),
            estimate_min,
            actual_min: 0,
            actual_carry_sec: 0,
            state: TaskState::Planned,
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

    pub fn finish_active(&mut self) {
        if let Some(cur) = self.active.take() {
            if let Some(t) = self.tasks.get_mut(cur) {
                t.state = TaskState::Done;
            }
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
        esd_from(now_min, &[self.remaining_total_min()])
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
