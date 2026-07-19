# Keyboard tester (W.I.P)

- Recently my keyboard had some issues and I couldn't find a normal keyboard tester so I made one
- Currently only works on Windows but I will expand it to Linux as well 

## Tech stack

- C++ WIN32 API
- Rust for the UI

## Usage

- Go to the Releases and download the file or compile yourself


# Setup

## Why no CMake?

- I really hate working with if you for some reason modify the cpp-core part here is a build command 

```powershell
g++ -std=c++17 -shared -static-libgcc -static-libstdc++ -Icpp-core/include cpp-core/src/keyboard_core.cpp cpp-core/src/keyboard_manager.cpp cpp-core/src/device_manager.cpp -o cpp-core/build/keyboard_core.dll "-Wl,--out-implib,cpp-core/build/libkeyboard_core.a" -luser32 -lhid
```