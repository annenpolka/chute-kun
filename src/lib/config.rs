use anyhow::{anyhow, Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub const APP_DIR_NAME: &str = "chute_kun";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeySpec {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl From<KeyEvent> for KeySpec {
    fn from(value: KeyEvent) -> Self {
        KeySpec { code: value.code, modifiers: value.modifiers }
    }
}

impl Hash for KeySpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // KeyCode doesn't implement Hash, so convert to a string-ish discriminant
        use KeyCode::*;
        let disc: u8 = match self.code {
            Backspace => 1,
            Enter => 2,
            Left => 3,
            Right => 4,
            Up => 5,
            Down => 6,
            Home => 7,
            End => 8,
            PageUp => 9,
            PageDown => 10,
            Tab => 11,
            BackTab => 12,
            Delete => 13,
            Insert => 14,
            Null => 15,
            Esc => 16,
            CapsLock => 17,
            ScrollLock => 18,
            NumLock => 19,
            PrintScreen => 20,
            Pause => 21,
            Menu => 22,
            KeypadBegin => 23,
            Media(_) => 24,
            Modifier(_) => 25,
            Char(_) => 26,
            F(n) => 30 + (n as u8),
        };
        disc.hash(state);
        match self.code {
            Char(c) => {
                (c as u32).hash(state);
            }
            _ => {}
        }
        // KeyModifiers is bitflags and implements Hash via bits
        self.modifiers.bits().hash(state);
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigToml {
    #[serde(default)]
    pub start_of_day: Option<String>,
    #[serde(default)]
    pub keys: Option<HashMap<String, String>>, // action -> key string
    #[serde(default)]
    pub esd_base: Option<String>, // "now" | "start_of_day"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ESDBase {
    Now,
    StartOfDay,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub start_of_day_min: Option<u16>,
    pub keys: HashMap<String, KeySpec>,
    pub esd_base: ESDBase,
}

impl Default for Config {
    fn default() -> Self {
        Self { start_of_day_min: None, keys: HashMap::new(), esd_base: ESDBase::Now }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_file_path()?;
        if !path.exists() {
            return Ok(Config::default());
        }
        let s = std::fs::read_to_string(&path)
            .with_context(|| format!("reading config file: {}", path.display()))?;
        let raw: ConfigToml = toml::from_str(&s).context("parsing TOML")?;
        let mut cfg = Config::default();
        if let Some(s) = raw.start_of_day {
            cfg.start_of_day_min = parse_hhmm_to_min(&s);
        }
        if let Some(map) = raw.keys {
            for (action, keystr) in map.into_iter() {
                if let Some(spec) = parse_key_spec(&keystr) {
                    cfg.keys.insert(action, spec);
                }
            }
        }
        if let Some(esd) = raw.esd_base.as_deref() {
            if let Some(v) = parse_esd_base(esd) {
                cfg.esd_base = v;
            }
        }
        Ok(cfg)
    }

    pub fn key_for(&self, action: &str) -> Option<KeySpec> {
        self.keys.get(action).copied()
    }
}

pub fn config_file_path() -> Result<PathBuf> {
    let base = xdg_config_home()?;
    Ok(base.join(APP_DIR_NAME).join("config.toml"))
}

fn xdg_config_home() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("XDG_CONFIG_HOME") {
        if !p.is_empty() {
            return Ok(PathBuf::from(p));
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return Ok(Path::new(&home).join(".config"));
        }
    }
    Err(anyhow!("cannot determine config directory: set XDG_CONFIG_HOME or HOME"))
}

fn parse_hhmm_to_min(s: &str) -> Option<u16> {
    let s = s.trim();
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let h: u16 = parts[0].parse().ok()?;
    let m: u16 = parts[1].parse().ok()?;
    if h < 24 && m < 60 {
        Some(h * 60 + m)
    } else {
        None
    }
}

fn parse_key_spec(s: &str) -> Option<KeySpec> {
    // Accept forms: "q", "]", "space", "enter", "tab", "backtab", "up", "down", "shift+enter", "shift+q"
    let mut s = s.trim().to_string();
    if s.starts_with('"') && s.ends_with('"') {
        s = s.trim_matches('"').to_string();
    }
    if s.starts_with('\'') && s.ends_with('\'') {
        s = s.trim_matches('\'').to_string();
    }
    let mut modifiers = KeyModifiers::empty();
    let mut name = s.as_str();
    if let Some(rest) = s.strip_prefix("shift+") {
        modifiers.insert(KeyModifiers::SHIFT);
        name = rest;
    }
    let name_l = name.to_lowercase();
    let code = match name_l.as_str() {
        "enter" => KeyCode::Enter,
        "space" => KeyCode::Char(' '),
        "tab" => KeyCode::Tab,
        "backtab" => KeyCode::BackTab,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        _ => {
            // single character
            let mut chars = name.chars();
            let ch = chars.next()?;
            if chars.next().is_some() {
                return None;
            }
            KeyCode::Char(ch)
        }
    };
    Some(KeySpec { code, modifiers })
}

fn parse_esd_base(s: &str) -> Option<ESDBase> {
    match s.trim().to_lowercase().as_str() {
        "now" => Some(ESDBase::Now),
        "start_of_day" | "start-of-day" => Some(ESDBase::StartOfDay),
        _ => None,
    }
}
