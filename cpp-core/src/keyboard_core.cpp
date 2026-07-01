#include "../include/keyboard_core.h"
#include "keyboard_manager.h"


namespace {
KeyboardManager& keyboardManager() {
    static KeyboardManager manager;
    return manager;
}
}


bool kb_init() {
    return keyboardManager().init();
}

void kb_shutdown() {
    keyboardManager().shutdown();
}

void kb_poll() {
    keyboardManager().poll();
}

bool kb_is_key_down(int virtual_key) {
    return keyboardManager().isKeyDown(virtual_key);
}