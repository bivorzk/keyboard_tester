#pragma once

class KeyboardManager {
public:
    bool init();
    void shutdown();
    void poll();
    bool isKeyDown(int virtual_key) const;
    bool registerRawInput(void* hwnd);
    void processRawInput(void* hrawinput);
    bool isRawInputEnabled() const;

private:
    bool initialized_ = false;
    bool raw_input_enabled_ = false;
};
