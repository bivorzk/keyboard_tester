use crate::ffi;
use std::ffi::c_void;

pub struct Keyboard;

impl Keyboard {
    pub fn init() -> bool {
        unsafe { ffi::kb_init() }
    }

    pub fn shutdown() {
        unsafe { ffi::kb_shutdown() };
    }

    pub fn poll() {
        unsafe { ffi::kb_poll() };
    }

    pub fn is_key_down(virtual_key: i32) -> bool {
        unsafe { ffi::kb_is_key_down(virtual_key) }
    }

    #[allow(dead_code)]
    pub fn register_raw_input(hwnd: *mut c_void) -> bool {
        unsafe { ffi::kb_register_raw_input(hwnd) }
    }

    #[allow(dead_code)]
    pub fn process_raw_input(hrawinput: *mut c_void) {
        unsafe { ffi::kb_process_raw_input(hrawinput) };
    }

    pub fn raw_input_enabled() -> bool {
        unsafe { ffi::kb_raw_input_enabled() }
    }
}
