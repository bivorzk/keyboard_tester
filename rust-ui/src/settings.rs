use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, time::Duration};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub theme: Theme,
    pub keyboard_layout: KeyboardLayout,
    pub window_size: WindowSize,
    pub polling_interval_ms: u64,
}

impl Settings {
    pub fn load() -> Self {
        let path = settings_path();

        fs::read_to_string(path)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = settings_path();

        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn polling_interval(&self) -> Duration {
        Duration::from_millis(self.polling_interval_ms)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            keyboard_layout: KeyboardLayout::Ansi,
            window_size: WindowSize {
                width: 1000.0,
                height: 520.0,
            },
            polling_interval_ms: 16,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum KeyboardLayout {
    Ansi,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

pub fn settings_path() -> PathBuf {
    PathBuf::from("settings.json")
}
