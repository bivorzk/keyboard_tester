use crate::ui::sidebar::KeyInfo;
use eframe::egui;

pub fn show(
    ctx: &egui::Context,
    open: &mut bool,
    polling_interval_ms: u64,
    pressed_key: Option<KeyInfo>,
    raw_input_enabled: bool,
) {
    egui::Window::new("Diagnostics").open(open).show(ctx, |ui| {
        row(ui, "DLL status", "Loaded");
        row(ui, "Polling status", "Active");
        row(ui, "Polling interval", format!("{polling_interval_ms} ms"));
        row(ui, "Connected keyboards", "1");
        row(ui, "Raw Input", if raw_input_enabled { "Enabled" } else { "Fallback polling" });

        let last_key = pressed_key
            .map(|key| format!("0x{:02X}", key.vk))
            .unwrap_or_else(|| "None".to_owned());
        row(ui, "Last pressed key", last_key);
    });
}

fn row(ui: &mut egui::Ui, label: &str, value: impl ToString) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.separator();
        ui.label(value.to_string());
    });
}
