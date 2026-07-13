#pragma once
#include "device_manager.h"

class KeyboardManager {
public:
    bool init();
    void shutdown();
    void poll();
    bool isKeyDown(int virtual_key) const;
    bool registerRawInput(void* hwnd);
    void processRawInput(void* hrawinput);
    bool isRawInputEnabled() const;
    int deviceCount() const;
    bool deviceName(int index, std::wstring& name) const;

private:
    bool initialized_ = false;
    bool raw_input_enabled_ = false;
    DeviceManager device_manager;
};
