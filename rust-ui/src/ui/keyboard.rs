use crate::settings::{KeyboardLayout, Theme};
use eframe::egui::{self, Button, Color32, RichText, Ui, Vec2};

const KEY_H: f32 = 44.0;
const GAP: f32 = 6.0;
const UNIT: f32 = 44.0;

pub fn show(ui: &mut Ui, layout: KeyboardLayout, theme: Theme, is_pressed: impl Fn(i32) -> bool) {
    ui.spacing_mut().item_spacing = Vec2::splat(GAP);

    let rows = match layout {
        KeyboardLayout::Ansi => ANSI_LAYOUT,
    };

    for row in rows {
        ui.horizontal(|ui| {
            for key in *row {
                if key.vk == 0 {
                    ui.add_space(UNIT * key.w);
                } else {
                    draw_key(ui, key, is_pressed(key.vk), theme);
                }
            }
        });
    }
}

fn draw_key(ui: &mut Ui, key: &Key, pressed: bool, theme: Theme) {
    let fill = if pressed && theme == Theme::Default {
        Color32::from_rgb(38, 59, 104)
    } else if pressed {
        Color32::from_rgb(64, 150, 110)
    } else if theme == Theme::Default {
        Color32::from_rgb(14, 20, 37)
    } else {
        Color32::from_rgb(42, 45, 50)
    };

    let stroke = if pressed && theme == Theme::Default {
        egui::Stroke::new(2.0, Color32::from_rgb(101, 217, 255))
    } else if pressed {
        egui::Stroke::new(2.0, Color32::from_rgb(180, 255, 215))
    } else if theme == Theme::Default {
        egui::Stroke::new(1.0, Color32::from_rgb(38, 59, 104))
    } else {
        egui::Stroke::new(1.0, Color32::from_rgb(85, 90, 98))
    };

    ui.add_sized(
        [UNIT * key.w + GAP * (key.w - 1.0), KEY_H],
        Button::new(RichText::new(key.label).color(Color32::WHITE))
            .fill(fill)
            .stroke(stroke),
    );
}

struct Key {
    label: &'static str,
    vk: i32,
    w: f32,
}

const fn key(label: &'static str, vk: i32) -> Key {
    Key { label, vk, w: 1.0 }
}

const fn wide(label: &'static str, vk: i32, w: f32) -> Key {
    Key { label, vk, w }
}

const fn space(w: f32) -> Key {
    Key {
        label: "",
        vk: 0,
        w,
    }
}

const ANSI_LAYOUT: &[&[Key]] = &[
    &[
        key("Esc", 0x1B),
        space(0.5),
        key("F1", 0x70),
        key("F2", 0x71),
        key("F3", 0x72),
        key("F4", 0x73),
        space(0.5),
        key("F5", 0x74),
        key("F6", 0x75),
        key("F7", 0x76),
        key("F8", 0x77),
        space(0.5),
        key("F9", 0x78),
        key("F10", 0x79),
        key("F11", 0x7A),
        key("F12", 0x7B),
        space(0.5),
        key("Prt", 0x2C),
        key("Scr", 0x91),
        key("Pause", 0x13),
    ],
    &[
        key("`", 0xC0),
        key("1", 0x31),
        key("2", 0x32),
        key("3", 0x33),
        key("4", 0x34),
        key("5", 0x35),
        key("6", 0x36),
        key("7", 0x37),
        key("8", 0x38),
        key("9", 0x39),
        key("0", 0x30),
        key("-", 0xBD),
        key("=", 0xBB),
        wide("Backspace", 0x08, 2.0),
        space(0.5),
        key("Ins", 0x2D),
        key("Home", 0x24),
        key("PgUp", 0x21),
    ],
    &[
        wide("Tab", 0x09, 1.5),
        key("Q", 0x51),
        key("W", 0x57),
        key("E", 0x45),
        key("R", 0x52),
        key("T", 0x54),
        key("Y", 0x59),
        key("U", 0x55),
        key("I", 0x49),
        key("O", 0x4F),
        key("P", 0x50),
        key("[", 0xDB),
        key("]", 0xDD),
        wide("\\", 0xDC, 1.5),
        space(0.5),
        key("Del", 0x2E),
        key("End", 0x23),
        key("PgDn", 0x22),
    ],
    &[
        wide("Caps", 0x14, 1.75),
        key("A", 0x41),
        key("S", 0x53),
        key("D", 0x44),
        key("F", 0x46),
        key("G", 0x47),
        key("H", 0x48),
        key("J", 0x4A),
        key("K", 0x4B),
        key("L", 0x4C),
        key(";", 0xBA),
        key("'", 0xDE),
        wide("Enter", 0x0D, 2.25),
        space(4.0),
    ],
    &[
        wide("Left Shift", 0xA0, 2.25),
        key("Z", 0x5A),
        key("X", 0x58),
        key("C", 0x43),
        key("V", 0x56),
        key("B", 0x42),
        key("N", 0x4E),
        key("M", 0x4D),
        key(",", 0xBC),
        key(".", 0xBE),
        key("/", 0xBF),
        wide("Right Shift", 0xA1, 2.75),
        space(1.625),
        key("Up", 0x26),
    ],
    &[
        wide("Ctrl", 0xA2, 1.25),
        wide("Win", 0x5B, 1.25),
        wide("Alt", 0xA4, 1.25),
        wide("Space", 0x20, 6.25),
        wide("Alt", 0xA5, 1.25),
        wide("Win", 0x5C, 1.25),
        wide("Menu", 0x5D, 1.25),
        wide("Ctrl", 0xA3, 1.25),
        space(0.5),
        key("Left", 0x25),
        key("Down", 0x28),
        key("Right", 0x27),
    ],
];
