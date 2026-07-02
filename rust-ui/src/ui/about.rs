use eframe::egui;

pub fn show(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("About")
        .open(open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Keyboard Tester");
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            ui.separator();
            ui.label("A small desktop utility for testing keyboard input.");
            ui.label("Stack: Rust, egui/eframe, C++, Win32 API");
            ui.label("Author: bivorzk");
        });
}
