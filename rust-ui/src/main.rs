mod ffi;
mod keyboard;
mod app;
mod settings;
mod ui {
    pub mod about;
    pub mod diagnostics;
    pub mod keyboard;
    pub mod settings;
    pub mod sidebar;
    pub mod statusbar;
}

use crate::settings::Settings;

fn load_icon(path: &str) -> Option<egui::IconData> {
    let image = image::open(path).ok()?.into_rgba8();
    let (width, height) = image.dimensions();

    Some(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}

#[cfg(windows)]
fn install_raw_input_hook(options: &mut eframe::NativeOptions) {
    use std::ffi::c_void;
    use windows_sys::Win32::UI::WindowsAndMessaging::{MSG, WM_INPUT};
    use winit::platform::windows::EventLoopBuilderExtWindows;

    options.event_loop_builder = Some(Box::new(|builder| {
        let mut registered = false;

        builder.with_msg_hook(move |message| {
            let message = unsafe { &*message.cast::<MSG>() };

            if !registered && !message.hwnd.is_null() {
                registered = keyboard::Keyboard::register_raw_input(
                    message.hwnd as *mut c_void,
                );
            }

            if registered && message.message == WM_INPUT {
                keyboard::Keyboard::process_raw_input(
                    message.lParam as *mut c_void,
                );
            }

            false
        });
    }));
}

fn main() {
    if !keyboard::Keyboard::init() {
        eprintln!("Failed to initialize keyboard core.");
        keyboard::Keyboard::shutdown();
        return;
    }

    println!("Keyboard core initialized successfully.");



    let settings = Settings::load();
    let mut viewport = egui::ViewportBuilder::default().with_inner_size([
        settings.window_size.width,
        settings.window_size.height,
    ]);

    if let Some(icon) = load_icon("assets/keyboard_logo.png") {
        viewport = viewport.with_icon(icon);
    }
    let mut options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    #[cfg(windows)]
    install_raw_input_hook(&mut options);

    if let Err(err) = eframe::run_native(
        "Keyboard Tester",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::new(settings)))),
    ) {
        eprintln!("Failed to run UI: {err}");
        keyboard::Keyboard::shutdown();
        return;
    }

    keyboard::Keyboard::shutdown();
}
