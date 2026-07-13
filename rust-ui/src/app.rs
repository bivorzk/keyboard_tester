use crate::keyboard::Keyboard;
use crate::settings::{Settings, Theme};
use crate::ui;

pub struct MyApp {
    settings: Settings,
    show_settings: bool,
    show_about: bool,
    show_diagnostics: bool,
}

impl MyApp {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            show_settings: false,
            show_about: false,
            show_diagnostics: false,
        }
    }
}


fn pressed_key() -> Option<ui::sidebar::KeyInfo> {
    (0x01..=0xFE).find_map(|vk| {
        let pressed = Keyboard::is_key_down(vk);
        pressed.then_some(ui::sidebar::KeyInfo { vk, pressed })
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
        ui.ctx().set_visuals(match self.settings.theme {
            Theme::Dark => egui::Visuals::dark(),
            Theme::Light => egui::Visuals::light(),
        });

        egui::Frame::central_panel(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Keyboard Tester");
                ui.toggle_value(&mut self.show_settings, "Settings");
                ui.toggle_value(&mut self.show_diagnostics, "Diagnostics");
                if ui.button("About").clicked() {
                    self.show_about = true;
                }
            });
            ui.add_space(12.0);
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui::keyboard::show(ui, self.settings.keyboard_layout, Keyboard::is_key_down);
                });

                ui.separator();

                ui.set_width(260.0);
                if self.show_settings {
                    ui::settings::show(ui, &mut self.settings);
                    ui.add_space(16.0);
                }
                ui::sidebar::show(ui, pressed_key());
            });

            ui.add_space(12.0);
            ui::statusbar::show(
                ui,
                self.settings.polling_interval_ms,
                pressed_key().map(|key| key.vk),
                Keyboard::raw_input_enabled(),
            );
        });

        ui::diagnostics::show(
            ui.ctx(),
            &mut self.show_diagnostics,
            self.settings.polling_interval_ms,
            pressed_key(),
            Keyboard::raw_input_enabled(),
        );
        ui::about::show(ui.ctx(), &mut self.show_about);
    }
}

impl Drop for MyApp {
    fn drop(&mut self) {
        self.settings.save();
    }
}
