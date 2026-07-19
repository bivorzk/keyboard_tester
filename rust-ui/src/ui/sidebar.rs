use eframe::egui::{RichText, Ui};

#[derive(Clone, Copy)]
pub struct KeyInfo {
    pub vk: i32,
}

pub fn show(ui: &mut Ui, key: Option<KeyInfo>) {
    ui.horizontal(|ui| {
        let Some(key) = key else {
            ui.label(RichText::new("Press any key to test it").weak());
            return;
        };

        let scan_code = scan_code(key.vk);
        ui.label(
            RichText::new(key_name(key.vk))
                .strong()
                .color(ui.visuals().hyperlink_color),
        );
        ui.label(RichText::new("is pressed").weak());
        ui.separator();
        ui.monospace(format!("VK 0x{:02X}  ·  Scan 0x{scan_code:02X}", key.vk));
    });
}

fn key_name(vk: i32) -> &'static str {
    match vk {
        0x08 => "Backspace",
        0x09 => "Tab",
        0x0D => "Enter",
        0x10 | 0xA0 | 0xA1 => "Shift",
        0x11 | 0xA2 | 0xA3 => "Ctrl",
        0x12 | 0xA4 | 0xA5 => "Alt",
        0x14 => "Caps Lock",
        0x1B => "Escape",
        0x20 => "Space",
        0x30..=0x39 | 0x41..=0x5A => char_name(vk),
        0x5B => "Left Windows",
        0x5C => "Right Windows",
        0x5D => "Menu",
        0xBA => ";",
        0xBB => "=",
        0xBC => ",",
        0xBD => "-",
        0xBE => ".",
        0xBF => "/",
        0xC0 => "`",
        0xDB => "[",
        0xDC => "\\",
        0xDD => "]",
        0xDE => "'",
        _ => "Unknown key",
    }
}

fn char_name(vk: i32) -> &'static str {
    const NAMES: [&str; 36] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
        "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    if vk <= 0x39 {
        NAMES[(vk - 0x30) as usize]
    } else {
        NAMES[(vk - 0x41 + 10) as usize]
    }
}

#[cfg(windows)]
fn scan_code(vk: i32) -> u32 {
    const MAPVK_VK_TO_VSC: u32 = 0;

    unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) }
}

#[cfg(not(windows))]
fn scan_code(_vk: i32) -> u32 {
    0
}

#[cfg(windows)]
#[link(name = "user32")]
unsafe extern "system" {
    fn MapVirtualKeyW(code: u32, map_type: u32) -> u32;
}
