use crate::keyboard::Keyboard;
use crate::settings::{Settings, Theme};
use crate::ui;

pub struct MyApp {
    settings: Settings,
    show_settings: bool,
    show_about: bool,
    show_devices: bool,
    show_diagnostics: bool,
}

impl MyApp {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            show_settings: false,
            show_about: false,
            show_devices: false,
            show_diagnostics: false,
        }
    }
}

fn visuals(theme: Theme) -> egui::Visuals {
    match theme {
        Theme::Dark => return egui::Visuals::dark(),
        Theme::Light => return egui::Visuals::light(),
        Theme::Default => {}
    }

    let bg = egui::Color32::from_rgb(7, 11, 22);
    let panel = egui::Color32::from_rgb(14, 20, 37);
    let line = egui::Color32::from_rgb(38, 59, 104);
    let text = egui::Color32::from_rgb(237, 247, 255);
    let muted = egui::Color32::from_rgb(169, 182, 203);
    let accent = egui::Color32::from_rgb(101, 217, 255);

    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(text);
    visuals.panel_fill = bg;
    visuals.window_fill = panel;
    visuals.extreme_bg_color = bg;
    visuals.faint_bg_color = panel;
    visuals.hyperlink_color = accent;
    visuals.selection.bg_fill = line;
    visuals.selection.stroke = egui::Stroke::new(1.5, accent);
    visuals.window_stroke = egui::Stroke::new(1.0, line);
    visuals.widgets.noninteractive.bg_fill = panel;
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, muted);
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, line);
    visuals.widgets.inactive.bg_fill = panel;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, line);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(20, 40, 67);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, accent);
    visuals.widgets.active.bg_fill = line;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5, accent);
    visuals
}

fn pressed_key() -> Option<ui::sidebar::KeyInfo> {
    (0x01..=0xFE).find_map(|vk| {
        let pressed = Keyboard::is_key_down(vk);
        pressed.then_some(ui::sidebar::KeyInfo { vk })
    })
}

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Keyboard::poll();
        ctx.input(|input| {
            if let Some(rect) = input.viewport().inner_rect {
                self.settings.window_size.width = rect.width();
                self.settings.window_size.height = rect.height();
            }
        });
        ctx.request_repaint_after(self.settings.polling_interval());
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(visuals(self.settings.theme));
        ui.painter()
            .rect_filled(ui.max_rect(), 0.0, ui.visuals().panel_fill);

        egui::Frame::central_panel(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Keyboard Tester");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                    ui.toggle_value(&mut self.show_diagnostics, "Diagnostics");
                    ui.toggle_value(&mut self.show_devices, "Devices");
                    ui.toggle_value(&mut self.show_settings, "Settings");
                });
            });
            ui.add_space(12.0);

            egui::Frame::group(ui.style())
                .inner_margin(8.0)
                .show(ui, |ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui::keyboard::show(
                            ui,
                            self.settings.keyboard_layout,
                            self.settings.theme,
                            Keyboard::is_key_down,
                        );
                    });
                });

            ui.add_space(8.0);
            ui::sidebar::show(ui, pressed_key());
            ui.add_space(8.0);
            ui::statusbar::show(
                ui,
                self.settings.polling_interval_ms,
                Keyboard::raw_input_enabled(),
                Keyboard::device_count(),
            );
        });

        egui::Window::new("Settings")
            .open(&mut self.show_settings)
            .resizable(false)
            .show(ui.ctx(), |ui| ui::settings::show(ui, &mut self.settings));
        ui::devices::show(ui.ctx(), &mut self.show_devices);
        ui::diagnostics::show(
            ui.ctx(),
            &mut self.show_diagnostics,
            self.settings.polling_interval_ms,
            pressed_key(),
            Keyboard::raw_input_enabled(),
            Keyboard::device_count(),
        );
        ui::about::show(ui.ctx(), &mut self.show_about);
    }
}

impl Drop for MyApp {
    fn drop(&mut self) {
        self.settings.save();
    }
}
