use std::os::raw::{c_char, c_int, c_void};

#[link(name = "keyboard_core")]
unsafe extern "C" {
    pub unsafe fn kb_init() -> bool;
    pub unsafe fn kb_shutdown();
    pub unsafe fn kb_poll();
    pub unsafe fn kb_is_key_down(virtual_key: c_int) -> bool;
    pub unsafe fn kb_register_raw_input(hwnd: *mut c_void) -> bool;
    pub unsafe fn kb_process_raw_input(hrawinput: *mut c_void);
    pub unsafe fn kb_raw_input_enabled() -> bool;
    pub unsafe fn kb_device_count() -> c_int;
    pub unsafe fn kb_device_name(index: c_int, buffer: *mut c_char, buffer_size: c_int) -> c_int;
}
