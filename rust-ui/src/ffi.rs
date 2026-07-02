use std::os::raw::c_int;

#[link(name = "keyboard_core")] 
unsafe extern "C" {
    pub unsafe fn kb_init() -> bool;
    pub unsafe fn kb_shutdown();
    pub unsafe fn kb_poll();
    pub unsafe fn kb_is_key_down(virtual_key: c_int) -> bool;
}
