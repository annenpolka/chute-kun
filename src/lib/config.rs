use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub start_of_day_min: u16,
    pub keys: Keybindings,
}

impl Default for Config {
    fn default() -> Self {
        Self { start_of_day_min: 9 * 60, keys: Keybindings::default() }
    }
}

#[derive(Debug, Clone)]
pub struct Keybindings {
    pub quit: char,
    pub interrupt: char,
    pub pause: char,
    pub inc_estimate: char,
    pub postpone: char,
    pub select_up: char,
    pub select_down: char,
    pub reorder_up: char,
    pub reorder_down: char,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: 'q',
            interrupt: 'i',
            pause: ' ',
            inc_estimate: 'e',
            postpone: 'p',
            select_up: 'k',
            select_down: 'j',
            reorder_up: '[',
            reorder_down: ']',
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct RawConfig {
    #[serde(default)]
    start_of_day: Option<String>,
    #[serde(default)]
    keybindings: Option<RawKeybindings>,
}

#[derive(Debug, Deserialize, Default)]
struct RawKeybindings {
    quit: Option<String>,
    interrupt: Option<String>,
    pause: Option<String>,
    inc_estimate: Option<String>,
    postpone: Option<String>,
    select_up: Option<String>,
    select_down: Option<String>,
    reorder_up: Option<String>,
    reorder_down: Option<String>,
}

fn parse_hhmm_to_min(s: &str) -> Option<u16> {
    let parts: Vec<&str> = s.trim().split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let h: u16 = parts[0].parse().ok()?;
    let m: u16 = parts[1].parse().ok()?;
    if h < 24 && m < 60 { Some(h * 60 + m) } else { None }
}

fn parse_key_char(s: &str) -> Option<char> {
    let t = s.trim();
    match t.to_lowercase().as_str() {
        "space" | "spc" => Some(' '),
        _ => t.chars().next(),
    }
}

fn config_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("CHUTE_KUN_CONFIG_DIR") {
        return Some(PathBuf::from(dir));
    }
    // XDG base dir fallback: $XDG_CONFIG_HOME/chute_kun or $HOME/.config/chute_kun
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg).join("chute_kun"));
    }
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home).join(".config").join("chute_kun"));
    }
    None
}

fn config_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join("config.toml"))
}

pub fn load() -> Config {
    let mut cfg = Config::default();
    if let Some(path) = config_path() {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(raw) = toml::from_str::<RawConfig>(&data) {
                if let Some(s) = raw.start_of_day.and_then(|v| parse_hhmm_to_min(&v)) {
                    cfg.start_of_day_min = s;
                }
                if let Some(k) = raw.keybindings {
                    let mut keys = Keybindings::default();
                    if let Some(v) = k.quit.and_then(|v| parse_key_char(&v)) {
                        keys.quit = v;
                    }
                    if let Some(v) = k.interrupt.and_then(|v| parse_key_char(&v)) {
                        keys.interrupt = v;
                    }
                    if let Some(v) = k.pause.and_then(|v| parse_key_char(&v)) {
                        keys.pause = v;
                    }
                    if let Some(v) = k.inc_estimate.and_then(|v| parse_key_char(&v)) {
                        keys.inc_estimate = v;
                    }
                    if let Some(v) = k.postpone.and_then(|v| parse_key_char(&v)) {
                        keys.postpone = v;
                    }
                    if let Some(v) = k.select_up.and_then(|v| parse_key_char(&v)) {
                        keys.select_up = v;
                    }
                    if let Some(v) = k.select_down.and_then(|v| parse_key_char(&v)) {
                        keys.select_down = v;
                    }
                    if let Some(v) = k.reorder_up.and_then(|v| parse_key_char(&v)) {
                        keys.reorder_up = v;
                    }
                    if let Some(v) = k.reorder_down.and_then(|v| parse_key_char(&v)) {
                        keys.reorder_down = v;
                    }
                    cfg.keys = keys;
                }
            }
        }
    }
    cfg
}
