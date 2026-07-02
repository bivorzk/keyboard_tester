#include "keyboard_manager.h"

#include <windows.h>
// unused due to refactor #include <vector>
#include <array>

std::array<bool, 256> pressedKeys; // refactored from std::vector<bool> pressedKeys(256, false);


namespace {
constexpr int kMinVirtualKey = 0x01;
constexpr int kMaxVirtualKey = 0xFE;

    bool isValidVirtualKey(int virtual_key) {
        return virtual_key >= kMinVirtualKey && virtual_key <= kMaxVirtualKey;
    }
}

bool KeyboardManager::init() {
    initialized_ = true; 
    return true;
}


void KeyboardManager::shutdown() {
    initialized_ = false;
}

void KeyboardManager::poll() {
    if (!initialized_) {
        return;
    }

    for (int virtual_key = kMinVirtualKey; virtual_key <= kMaxVirtualKey; ++virtual_key) {
        if (isValidVirtualKey(virtual_key)) {
            bool is_down = (GetAsyncKeyState(virtual_key) & 0x8000) != 0;
            pressedKeys[virtual_key] = is_down;
        }
    }
}


bool KeyboardManager::isKeyDown(int virtual_key) const {
    if (!initialized_ || !isValidVirtualKey(virtual_key)) {
        return false;
    }

    return pressedKeys[virtual_key]; // 0x8000 
}
