struct AppState {
    keyboard_state: KeyboardState,
    keyboard_layout: KeyboardLayout,
    pressed_keys: Vec<u32>,
    settings: Settings,
    diagnostic_info: DiagnosticInfo,
}

