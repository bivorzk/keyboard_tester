mod ffi;
mod ui {
    pub mod keyboard;
    pub mod settings;
    pub mod sidebar;
}

use ui::settings::{Settings, Theme};

struct MyApp {
    settings: Settings,
    show_settings: bool,
}

impl MyApp {
    fn new(settings: Settings) -> Self {
        Self {
            settings,
            show_settings: false,
        }
    }
}

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        unsafe { ffi::kb_poll() };
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
            });
            ui.add_space(12.0);
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui::keyboard::show(ui, |vk| unsafe { ffi::kb_is_key_down(vk) });
                });

                ui.separator();

                ui.set_width(260.0);
                if self.show_settings {
                    ui::settings::show(ui, &mut self.settings);
                    ui.add_space(16.0);
                }
                ui::sidebar::show(ui, pressed_key());
            });
        });
    }
}

impl Drop for MyApp {
    fn drop(&mut self) {
        self.settings.save();
    }
}

fn pressed_key() -> Option<ui::sidebar::KeyInfo> {
    (0x01..=0xFE).find_map(|vk| {
        let pressed = unsafe { ffi::kb_is_key_down(vk) };
        pressed.then_some(ui::sidebar::KeyInfo { vk, pressed })
    })
}

fn main() {
    if !unsafe { ffi::kb_init() } {
        eprintln!("Failed to initialize keyboard core.");
        return;
    }

    println!("Keyboard core initialized successfully.");

    let settings = Settings::load();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([
            settings.window_size.width,
            settings.window_size.height,
        ]),
        ..Default::default()
    };

    if let Err(err) = eframe::run_native(
        "Keyboard Tester",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(settings)))),
    ) {
        eprintln!("Failed to run UI: {err}");
    }

    unsafe { ffi::kb_shutdown() };
}
