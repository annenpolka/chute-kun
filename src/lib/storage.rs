use crate::app::{App, View};
use crate::config::Config;
use crate::task::{DayPlan, Task};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Snapshot of user-visible task state.
/// - versioned for forwards/backwards compatibility
/// - lists are intentionally flat vectors (git‑friendly and easy to diff)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotV1 {
    pub version: u8,
    #[serde(default)]
    pub today: Vec<Task>,
    #[serde(default)]
    pub future: Vec<Task>,
    #[serde(default)]
    pub past: Vec<Task>,
}

impl Default for SnapshotV1 {
    fn default() -> Self {
        Self { version: 1, today: vec![], future: vec![], past: vec![] }
    }
}

impl SnapshotV1 {
    pub fn from_app(app: &App) -> Self {
        Self {
            version: 1,
            today: app.day.tasks.clone(),
            future: app.tomorrow_tasks().clone(),
            past: app.history_tasks().clone(),
        }
    }

    pub fn into_app(self, config: Config) -> App {
        let mut app = App::with_config(config);
        app.apply_snapshot(self.today, self.future, self.past);
        app
    }
}

/// Serialize current app state into a TOML string.
pub fn save_to_string(app: &App) -> Result<String> {
    let snap = SnapshotV1::from_app(app);
    toml::to_string_pretty(&snap).context("serialize snapshot to toml")
}

/// Deserialize from a TOML string into a new `App` using the provided config.
pub fn load_from_str(s: &str, config: Config) -> Result<App> {
    let snap: SnapshotV1 = toml::from_str(s).context("parse snapshot toml")?;
    Ok(snap.into_app(config))
}

/// Save to a file path, creating parent directories if missing.
pub fn save_to_path<P: AsRef<Path>>(app: &App, path: P) -> Result<()> {
    let s = save_to_string(app)?;
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(&path, s).context("write snapshot file")
}

/// Load from a file path; returns `Ok(None)` if the file does not exist.
pub fn load_from_path<P: AsRef<Path>>(path: P, config: Config) -> Result<Option<App>> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(None);
    }
    let s = fs::read_to_string(path).context("read snapshot file")?;
    let app = load_from_str(&s, config)?;
    Ok(Some(app))
}
