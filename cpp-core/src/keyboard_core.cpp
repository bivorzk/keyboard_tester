#include "../include/keyboard_core.h"
#include "keyboard_manager.h"

#include <string>
#include <windows.h>


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

bool kb_register_raw_input(void* hwnd) {
    return keyboardManager().registerRawInput(hwnd);
}

void kb_process_raw_input(void* hrawinput) {
    keyboardManager().processRawInput(hrawinput);
}

bool kb_raw_input_enabled() {
    return keyboardManager().isRawInputEnabled();
}

int kb_device_count() {
    return keyboardManager().deviceCount();
}

int kb_device_name(int index, char* buffer, int buffer_size) {
    std::wstring wide_name;

    if (!keyboardManager().deviceName(index, wide_name)) {
        return 0;
    }

    const int required_size = WideCharToMultiByte(
        CP_UTF8,
        0,
        wide_name.c_str(),
        -1,
        nullptr,
        0,
        nullptr,
        nullptr
    );

    if (required_size == 0) {
        return 0;
    }

    if (buffer == nullptr && buffer_size == 0) {
        return required_size;
    }

    if (buffer == nullptr || buffer_size < required_size) {
        return 0;
    }

    return WideCharToMultiByte(
        CP_UTF8,
        0,
        wide_name.c_str(),
        -1,
        buffer,
        buffer_size,
        nullptr,
        nullptr
    );
}
