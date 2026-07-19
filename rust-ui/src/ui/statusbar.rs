use eframe::egui::{RichText, Ui};

pub fn show(ui: &mut Ui, polling_interval_ms: u64, raw_input_enabled: bool, device_count: usize) {
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Ready")
                .strong()
                .color(ui.visuals().hyperlink_color),
        );
        ui.separator();
        ui.label(if raw_input_enabled {
            "Raw Input"
        } else {
            "Fallback polling"
        });
        ui.separator();
        ui.label(format!("{polling_interval_ms} ms"));
        ui.separator();
        ui.label(format!(
            "{device_count} keyboard{}",
            if device_count == 1 { "" } else { "s" }
        ));
    });
}
