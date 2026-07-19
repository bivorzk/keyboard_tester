use crate::settings::{KeyboardLayout, Settings, Theme};
use eframe::egui::{ComboBox, DragValue, Ui};

pub fn show(ui: &mut Ui, settings: &mut Settings) {
    ComboBox::from_label("Theme")
        .selected_text(match settings.theme {
            Theme::Default => "Default",
            Theme::Dark => "Dark",
            Theme::Light => "Light",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut settings.theme, Theme::Default, "Default");
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
