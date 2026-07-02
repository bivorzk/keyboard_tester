use crate::ffi;

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
}