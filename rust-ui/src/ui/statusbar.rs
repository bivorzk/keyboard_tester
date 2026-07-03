use eframe::egui::{RichText, Ui};

pub fn show(
    ui: &mut Ui,
    polling_interval_ms: u64,
    pressed_key: Option<i32>,
    raw_input_enabled: bool,
) {
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("DLL: loaded");
        ui.separator();
        ui.label("Polling: active");
        ui.separator();
        ui.label(if raw_input_enabled { "Raw Input: on" } else { "Raw Input: off" });
        ui.separator();
        ui.label(format!("Interval: {polling_interval_ms} ms"));
        ui.separator();
        ui.label("Keyboards: 1");
        ui.separator();

        match pressed_key {
            Some(vk) => ui.label(format!("Last key: 0x{vk:02X}")),
            None => ui.label(RichText::new("Last key: none").weak()),
        };
    });
}
