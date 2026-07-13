#include "device_manager.h"

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

    name = keyboard_devices[index].device_name;
    return true;
}

