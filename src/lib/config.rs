//! App configuration loaded from a config file or defaults.
//! - Supports day start time (HH:MM) and key bindings.
//! - Defaults: day start 09:00 and built-in keymap compatible with current tests.

use anyhow::{anyhow, Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub day_start_minutes: u16,
    pub keys: KeyMap,
}

impl Default for Config {
    fn default() -> Self {
        Self { day_start_minutes: 9 * 60, keys: KeyMap::default() }
    }
}

#[derive(Debug, Clone)]
pub struct KeyMap {
    pub quit: Vec<KeySpec>,
    pub add_task: Vec<KeySpec>,
    pub add_interrupt: Vec<KeySpec>,
    pub start_or_resume: Vec<KeySpec>,
    pub finish_active: Vec<KeySpec>,
    pub pause: Vec<KeySpec>,
    pub reorder_up: Vec<KeySpec>,
    pub reorder_down: Vec<KeySpec>,
    pub estimate_plus: Vec<KeySpec>,
    pub postpone: Vec<KeySpec>,
    pub view_next: Vec<KeySpec>,
    pub view_prev: Vec<KeySpec>,
    pub select_up: Vec<KeySpec>,
    pub select_down: Vec<KeySpec>,
}

impl Default for KeyMap {
    fn default() -> Self {
        use KeySpec as K;
        let k = |s| K::parse(s).expect("valid default key spec");
        KeyMap {
            quit: vec![k("q")],
            add_task: vec![k("i")],
            add_interrupt: vec![k("I")],
            start_or_resume: vec![k("Enter")],
            finish_active: vec![k("Shift+Enter")],
            pause: vec![k("Space")],
            reorder_up: vec![k("[")],
            reorder_down: vec![k("]")],
            estimate_plus: vec![k("e")],
            postpone: vec![k("p")],
            view_next: vec![k("Tab")],
            view_prev: vec![k("BackTab")],
            select_up: vec![k("Up"), k("k")],
            select_down: vec![k("Down"), k("j")],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    AddTask,
    AddInterrupt,
    StartOrResume,
    FinishActive,
    Pause,
    ReorderUp,
    ReorderDown,
    EstimatePlus,
    Postpone,
    ViewNext,
    ViewPrev,
    SelectUp,
    SelectDown,
}

impl KeyMap {
    pub fn action_for(&self, ev: &KeyEvent) -> Option<Action> {
        let matches = |list: &Vec<KeySpec>| list.iter().any(|k| k.matches(ev));
        if matches(&self.quit) {
            Some(Action::Quit)
        } else if matches(&self.add_task) {
            Some(Action::AddTask)
        } else if matches(&self.add_interrupt) {
            Some(Action::AddInterrupt)
        } else if matches(&self.start_or_resume) {
            Some(Action::StartOrResume)
        } else if matches(&self.finish_active) {
            Some(Action::FinishActive)
        } else if matches(&self.pause) {
            Some(Action::Pause)
        } else if matches(&self.reorder_up) {
            Some(Action::ReorderUp)
        } else if matches(&self.reorder_down) {
            Some(Action::ReorderDown)
        } else if matches(&self.estimate_plus) {
            Some(Action::EstimatePlus)
        } else if matches(&self.postpone) {
            Some(Action::Postpone)
        } else if matches(&self.view_next) {
            Some(Action::ViewNext)
        } else if matches(&self.view_prev) {
            Some(Action::ViewPrev)
        } else if matches(&self.select_up) {
            Some(Action::SelectUp)
        } else if matches(&self.select_down) {
            Some(Action::SelectDown)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeySpec {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeySpec {
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        if s.is_empty() {
            return Err(anyhow!("empty key spec"));
        }
        let mut parts = s.split('+').map(str::trim).collect::<Vec<_>>();
        let key_str = parts.pop().unwrap();
        let mut mods = KeyModifiers::empty();
        for m in parts {
            match m.to_ascii_lowercase().as_str() {
                "shift" => mods |= KeyModifiers::SHIFT,
                "ctrl" | "control" => mods |= KeyModifiers::CONTROL,
                "alt" => mods |= KeyModifiers::ALT,
                other => return Err(anyhow!("unsupported modifier: {}", other)),
            }
        }
        let code = match key_str {
            "Enter" => KeyCode::Enter,
            "Space" => KeyCode::Char(' '),
            "Tab" => KeyCode::Tab,
            "BackTab" => KeyCode::BackTab,
            "Up" => KeyCode::Up,
            "Down" => KeyCode::Down,
            // common punctuation and single char letters
            s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
            other => return Err(anyhow!("unsupported key: {}", other)),
        };
        Ok(KeySpec { code, modifiers: mods })
    }

    pub fn matches(&self, ev: &KeyEvent) -> bool {
        ev.code == self.code && ev.modifiers == self.modifiers
    }
}

// ----- Loading / Parsing -----

#[derive(Debug, Deserialize)]
struct RawConfig {
    #[serde(default)]
    day_start: Option<String>,
    #[serde(default)]
    keys: Option<RawKeys>,
}

#[derive(Debug, Deserialize, Default)]
struct RawKeys {
    quit: Option<OneOrMany>,
    add_task: Option<OneOrMany>,
    add_interrupt: Option<OneOrMany>,
    start_or_resume: Option<OneOrMany>,
    finish_active: Option<OneOrMany>,
    pause: Option<OneOrMany>,
    reorder_up: Option<OneOrMany>,
    reorder_down: Option<OneOrMany>,
    estimate_plus: Option<OneOrMany>,
    postpone: Option<OneOrMany>,
    view_next: Option<OneOrMany>,
    view_prev: Option<OneOrMany>,
    select_up: Option<OneOrMany>,
    select_down: Option<OneOrMany>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

impl OneOrMany {
    fn to_vec(self) -> Vec<String> {
        match self {
            OneOrMany::One(s) => vec![s],
            OneOrMany::Many(v) => v,
        }
    }
}

fn parse_hhmm_to_minutes(s: &str) -> Result<u16> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow!("invalid time format, expected HH:MM: {}", s));
    }
    let h: u16 = parts[0].parse().context("invalid hour")?;
    let m: u16 = parts[1].parse().context("invalid minute")?;
    Ok((h % 24) * 60 + (m % 60))
}

impl Config {
    pub fn from_toml_str(s: &str) -> Result<Self> {
        let raw: RawConfig = toml::from_str(s).context("parse config toml")?;
        let mut cfg = Config::default();
        if let Some(ds) = raw.day_start {
            cfg.day_start_minutes = parse_hhmm_to_minutes(&ds)?;
        }
        if let Some(keys) = raw.keys {
            let mut km = KeyMap::default();
            let apply = |dst: &mut Vec<KeySpec>, src: OneOrMany| -> Result<()> {
                *dst = src
                    .to_vec()
                    .into_iter()
                    .map(|s| KeySpec::parse(&s))
                    .collect::<Result<Vec<_>>>()?;
                Ok(())
            };
            if let Some(v) = keys.quit { apply(&mut km.quit, v)?; }
            if let Some(v) = keys.add_task { apply(&mut km.add_task, v)?; }
            if let Some(v) = keys.add_interrupt { apply(&mut km.add_interrupt, v)?; }
            if let Some(v) = keys.start_or_resume { apply(&mut km.start_or_resume, v)?; }
            if let Some(v) = keys.finish_active { apply(&mut km.finish_active, v)?; }
            if let Some(v) = keys.pause { apply(&mut km.pause, v)?; }
            if let Some(v) = keys.reorder_up { apply(&mut km.reorder_up, v)?; }
            if let Some(v) = keys.reorder_down { apply(&mut km.reorder_down, v)?; }
            if let Some(v) = keys.estimate_plus { apply(&mut km.estimate_plus, v)?; }
            if let Some(v) = keys.postpone { apply(&mut km.postpone, v)?; }
            if let Some(v) = keys.view_next { apply(&mut km.view_next, v)?; }
            if let Some(v) = keys.view_prev { apply(&mut km.view_prev, v)?; }
            if let Some(v) = keys.select_up { apply(&mut km.select_up, v)?; }
            if let Some(v) = keys.select_down { apply(&mut km.select_down, v)?; }
            cfg.keys = km;
        }
        Ok(cfg)
    }

    pub fn load() -> Self {
        if let Ok(path) = std::env::var("CHUTE_KUN_CONFIG") {
            if let Ok(s) = fs::read_to_string(&path) {
                if let Ok(cfg) = Self::from_toml_str(&s) {
                    return cfg;
                }
            }
        }
        let path = default_config_path();
        if let Some(path) = path {
            if path.exists() {
                if let Ok(s) = fs::read_to_string(&path) {
                    if let Ok(cfg) = Self::from_toml_str(&s) {
                        return cfg;
                    }
                }
            }
        }
        Config::default()
    }

    /// Render a default TOML string users can customize.
    pub fn default_toml() -> String {
        // Keep keys aligned with KeyMap::default()
        r#"# Chute_kun configuration
# 設定ファイルの場所: $XDG_CONFIG_HOME/chute_kun/config.toml （なければ ~/.config/chute_kun/config.toml）

# 1日の開始時刻（固定表示）。"HH:MM" 形式。既定は 09:00。
day_start = "09:00"

[keys]
# 既定のキーバインド。必要なものだけ上書きできます。
quit = "q"
add_task = "i"
add_interrupt = "I"
start_or_resume = "Enter"
finish_active = "Shift+Enter"
pause = "Space"
reorder_up = "["
reorder_down = "]"
estimate_plus = "e"
postpone = "p"
view_next = "Tab"
view_prev = "BackTab"
select_up = ["Up", "k"]
select_down = ["Down", "j"]
"#.to_string()
    }

    /// Write a default config file to the resolved path.
    /// - If `CHUTE_KUN_CONFIG` is set, writes there; otherwise XDG default.
    /// - Creates parent directories when必要.
    /// - If file already exists, leaves it as-is and returns Ok(path).
    pub fn write_default_file() -> Result<std::path::PathBuf> {
        let path = if let Ok(p) = std::env::var("CHUTE_KUN_CONFIG") {
            std::path::PathBuf::from(p)
        } else {
            default_config_path().ok_or_else(|| anyhow!("could not resolve config path"))?
        };
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent).ok(); }
        if !path.exists() {
            std::fs::write(&path, Self::default_toml()).context("write default config")?;
        }
        Ok(path)
    }
}

pub fn default_config_path() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg).join("chute_kun").join("config.toml"));
    }
    // macOS: prefer ~/.config over ~/Library/Application Support to keep cross‑platform consistency
    if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            return Some(PathBuf::from(home).join(".config").join("chute_kun").join("config.toml"));
        }
    }
    // Fallback to OS default (Linux uses ~/.config; Windows gets %APPDATA%)
    dirs::config_dir().map(|b| b.join("chute_kun").join("config.toml"))
}
