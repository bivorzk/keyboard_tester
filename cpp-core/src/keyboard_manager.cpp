#include "keyboard_manager.h"

#include <array>
#include <windows.h>
#include <vector>

namespace {
constexpr int kMinVirtualKey = 0x01;
constexpr int kMaxVirtualKey = 0xFE;

std::array<bool, 256> pressedKeys = {};

bool isValidVirtualKey(int virtual_key) {
    return virtual_key >= kMinVirtualKey && virtual_key <= kMaxVirtualKey;
}
}

bool KeyboardManager::init() {
    device_manager.enumerate();
    pressedKeys.fill(false);
    initialized_ = true;
    return true;
}

void KeyboardManager::shutdown() {
    pressedKeys.fill(false);
    raw_input_enabled_ = false;
    initialized_ = false;
}

int KeyboardManager::deviceCount() const {
    return device_manager.deviceCount();
}

bool KeyboardManager::deviceName(int index, std::wstring& name) const {
    return device_manager.deviceName(index, name);
}

void KeyboardManager::poll() {
    if (!initialized_ || raw_input_enabled_) {
        return;
    }

    for (int virtual_key = kMinVirtualKey; virtual_key <= kMaxVirtualKey; ++virtual_key) {
        pressedKeys[virtual_key] = (GetAsyncKeyState(virtual_key) & 0x8000) != 0;
    }
}

bool KeyboardManager::isKeyDown(int virtual_key) const {
    if (!initialized_ || !isValidVirtualKey(virtual_key)) {
        return false;
    }

    return pressedKeys[virtual_key];
}

bool KeyboardManager::registerRawInput(void* hwnd) {
    if (!initialized_ || !hwnd) {
        return false;
    }

    HWND target = reinterpret_cast<HWND>(hwnd);

    RAWINPUTDEVICE rid = {};
    rid.usUsagePage = 0x01;
    rid.usUsage = 0x06;
    rid.dwFlags = RIDEV_INPUTSINK;
    rid.hwndTarget = target;

    raw_input_enabled_ = RegisterRawInputDevices(&rid, 1, sizeof(rid)) != FALSE;
    if (!raw_input_enabled_) {
        return false;
    }

    return true;
}

void KeyboardManager::processRawInput(void* hrawinput) {
    if (!initialized_ || !raw_input_enabled_ || !hrawinput) {
        return;
    }

    HRAWINPUT raw_input = reinterpret_cast<HRAWINPUT>(hrawinput);

    UINT size = 0;
    GetRawInputData(raw_input, RID_INPUT, nullptr, &size, sizeof(RAWINPUTHEADER));
    
    if (size == 0) {
        return;
    }

    std::vector<BYTE> buffer(size);
    if (GetRawInputData(raw_input, RID_INPUT, buffer.data(), &size, sizeof(RAWINPUTHEADER)) != size) {
        return; 
    }

    const RAWINPUT* raw = reinterpret_cast<const RAWINPUT*>(buffer.data());
    if (raw->header.dwType != RIM_TYPEKEYBOARD) {
        return;
    }

    const RAWKEYBOARD& keyboard = raw->data.keyboard;
    device_manager.recordInput(raw->header.hDevice);

    int virtual_key = keyboard.VKey;
    const bool extended = (keyboard.Flags & RI_KEY_E0) != 0;

    switch (virtual_key) {
    case VK_SHIFT:
        virtual_key = MapVirtualKeyW(
            keyboard.MakeCode,
            MAPVK_VSC_TO_VK_EX
        );
        break;

    case VK_CONTROL:
        virtual_key = extended ? VK_RCONTROL : VK_LCONTROL;
        break;

    case VK_MENU:
        virtual_key = extended ? VK_RMENU : VK_LMENU;
        break;
    }

    if (!isValidVirtualKey(virtual_key)) {
        return;
    }

    const bool is_up = (keyboard.Flags & RI_KEY_BREAK) != 0;
    pressedKeys[virtual_key] = !is_up;
}

bool KeyboardManager::isRawInputEnabled() const {
    return raw_input_enabled_;
}
