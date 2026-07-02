use eframe::egui::{ComboBox, DragValue, Ui};
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
        fs::read_to_string(settings_path())
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(settings_path(), json);
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum KeyboardLayout {
    Ansi,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

pub fn show(ui: &mut Ui, settings: &mut Settings) {
    ui.heading("Settings");
    ui.separator();

    ComboBox::from_label("Theme")
        .selected_text(match settings.theme {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut settings.theme, Theme::Dark, "Dark");
            ui.selectable_value(&mut settings.theme, Theme::Light, "Light");
        });

    ComboBox::from_label("Keyboard layout")
        .selected_text("ANSI")
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut settings.keyboard_layout, KeyboardLayout::Ansi, "ANSI");
        });

    ui.horizontal(|ui| {
        ui.label("Polling interval");
        ui.add(
            DragValue::new(&mut settings.polling_interval_ms)
                .range(1..=1000)
                .suffix(" ms"),
        );
    });
}

pub fn settings_path() -> PathBuf {
    PathBuf::from("settings.json")
}
