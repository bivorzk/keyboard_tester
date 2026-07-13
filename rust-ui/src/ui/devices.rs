use crate::keyboard::Keyboard;
use eframe::egui::{self, RichText};

pub fn show(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("Connected keyboards")
        .open(open)
        .default_width(520.0)
        .show(ctx, |ui| {
            let count = Keyboard::device_count();
            ui.label(format!("Detected keyboards: {count}"));
            ui.separator();

            if count == 0 {
                ui.label(RichText::new("No keyboards detected.").italics());
                return;
            }

            for index in 0..count {
                ui.group(|ui| {
                    ui.label(RichText::new(format!("Keyboard {}", index + 1)).strong());
                    ui.monospace(
                        Keyboard::device_name(index)
                            .filter(|name| !name.is_empty())
                            .unwrap_or_else(|| "Unknown device".to_owned()),
                    );
                });
            }
        });
}
