use std::os::raw::c_int;

#[link(name = "keyboard_core")] 
extern "C" {
    pub fn kb_init() -> bool;
    pub fn kb_shutdown();
    pub fn kb_poll();
    pub fn kb_is_key_down(virtual_key: c_int) -> bool;
}
