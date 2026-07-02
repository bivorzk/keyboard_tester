use eframe::egui::{self, Button, Color32, RichText, Ui, Vec2};

const KEY_H: f32 = 44.0;
const GAP: f32 = 6.0;
const UNIT: f32 = 44.0;

pub fn show(ui: &mut Ui, is_pressed: impl Fn(i32) -> bool) {
    ui.spacing_mut().item_spacing = Vec2::splat(GAP);

    for row in ANSI_LAYOUT {
        ui.horizontal(|ui| {
            for key in *row {
                draw_key(ui, key, is_pressed(key.vk));
            }
        });
    }
}

fn draw_key(ui: &mut Ui, key: &Key, pressed: bool) {
    let fill = if pressed {
        Color32::from_rgb(64, 150, 110)
    } else {
        Color32::from_rgb(42, 45, 50)
    };

    let stroke = if pressed {
        egui::Stroke::new(2.0, Color32::from_rgb(180, 255, 215))
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

const ANSI_LAYOUT: &[&[Key]] = &[
    &[
        key("Esc", 0x1B),
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
    ],
    &[
        wide("Shift", 0xA0, 2.25),
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
        wide("Shift", 0xA1, 2.75),
    ],
    &[
        wide("Ctrl", 0xA2, 1.5),
        wide("Win", 0x5B, 1.25),
        wide("Alt", 0xA4, 1.25),
        wide("Space", 0x20, 6.25),
        wide("Alt", 0xA5, 1.25),
        wide("Fn", 0x00, 1.25),
        wide("Menu", 0x5D, 1.25),
        wide("Ctrl", 0xA3, 1.5),
    ],
];
