#pragma once

#include <chrono>
#include <string>
#include <vector>
#include <windows.h>

struct KeyboardDevice {
    HANDLE device_handle = nullptr;
    std::wstring device_name;
    std::wstring friendly_name;
    bool is_connected = false;
    std::chrono::steady_clock::time_point last_input_time{};
};

class DeviceManager {

public:
    void enumerate();
    bool recordInput(HANDLE device_handle);
    int deviceCount() const;
    bool deviceName(int index, std::wstring& name) const;

private:
    std::vector<KeyboardDevice> keyboard_devices;
};
