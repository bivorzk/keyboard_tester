#pragma once

class KeyboardManager {
public:
    bool init();
    void shutdown();
    void poll();
    bool isKeyDown(int virtual_key) const;

private:
    bool initialized_ = false;
};

