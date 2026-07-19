#include "device_manager.h"
#include <hidsdi.h>

#ifdef _MSC_VER
#pragma comment(lib, "hid.lib")
#endif

namespace {
std::wstring getFriendlyName(const std::wstring& device_path) {
    HANDLE handle = CreateFileW(
        device_path.c_str(),
        0,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        nullptr,
        OPEN_EXISTING,
        0,
        nullptr
    );

    if (handle == INVALID_HANDLE_VALUE) {
        return L"";
    }

    wchar_t buffer[256] = {};
    std::wstring result;
    if (HidD_GetProductString(handle, buffer, sizeof(buffer))) {
        result = buffer;
    }
    CloseHandle(handle);
    return result;
}
}

void DeviceManager::enumerate() {
    keyboard_devices.clear();

    UINT device_count = 0;
    if (GetRawInputDeviceList(nullptr, &device_count, sizeof(RAWINPUTDEVICELIST)) != 0 || device_count == 0) {
        return;
    }

    std::vector<RAWINPUTDEVICELIST> raw_devices(device_count);
    const UINT result = GetRawInputDeviceList(
        raw_devices.data(),
        &device_count,
        sizeof(RAWINPUTDEVICELIST)
    );

    if (result == static_cast<UINT>(-1)) {
        return;
    }
    for (UINT index = 0; index < result; ++index) {
        const RAWINPUTDEVICELIST& raw_device = raw_devices[index];
        if (raw_device.dwType != RIM_TYPEKEYBOARD) {
            continue;
        }

        KeyboardDevice device;
        device.device_handle = raw_device.hDevice;
        device.is_connected = true;

        UINT name_length = 0;
        GetRawInputDeviceInfoW(raw_device.hDevice, RIDI_DEVICENAME, nullptr, &name_length);

        if (name_length > 0) {
            std::wstring name(name_length, L'\0');
            if (GetRawInputDeviceInfoW(
                    raw_device.hDevice,
                    RIDI_DEVICENAME,
                    name.data(),
                    &name_length
                ) != static_cast<UINT>(-1)) {
                if (!name.empty() && name.back() == L'\0') {
                    name.pop_back();
                }
                device.device_name = std::move(name);
            }
        }

        if (device.device_name.find(L"RDP_KBD") != std::wstring::npos) {
            continue;
        }

        device.friendly_name = getFriendlyName(device.device_name);
        if (device.friendly_name.empty()) {
            device.friendly_name = device.device_name;
        }

        keyboard_devices.push_back(std::move(device));
    }
}

bool DeviceManager::recordInput(HANDLE device_handle) {
    for (KeyboardDevice& device : keyboard_devices) {
        if (device.device_handle == device_handle) {
            device.last_input_time = std::chrono::steady_clock::now();
            return true;
        }
    }

    return false;
}

int DeviceManager::deviceCount() const {
    return static_cast<int>(keyboard_devices.size());
}

bool DeviceManager::deviceName(int index, std::wstring& name) const {
    if (index < 0 || static_cast<std::size_t>(index) >= keyboard_devices.size()) {
        return false;
    }

    name = keyboard_devices[index].friendly_name;
    return true;
}
