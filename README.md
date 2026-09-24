# Cordial Roblox Runner

This fork is a standalone Linux Roblox runner implemented in C++.

## Standalone runner

The normal launch path is:

    ./roblox

The defaults are `./libroblox.so` and `./assets`. Override them with:

    ./roblox --libroblox /path/to/libroblox.so --assets /path/to/assets

Optional window sizing:

    ./roblox --width 1920 --height 1080

The runtime creates an X11 host window, supplies Cordial's native Android/JNI compatibility layer, loads Roblox through the vendored Bionic linker, and forwards keyboard/mouse input through the existing C++ GameActivity bridge.

The runtime layout can be:

    build/
    ├── roblox
    ├── libroblox.so
    └── assets/
        ├── content/
        ├── ssl/
        ├── android/
        └── ...

It does not download an APK at launch. It uses the supplied `libroblox.so` and `assets/` directory directly.

## Build

Install a C++17 compiler, CMake, X11 development headers/libraries, and the native dependencies required by the bundled Bionic linker/JNI/audio layers.

Configure and build:

    cmake -S . -B build
    cmake --build build -j$(nproc)

The executable is `build/roblox`.

The project contains no Rust workspace or Cargo build step. The required third-party runtime components remain vendored C++ projects (`mcpelauncher-linker` and `libjnivm`).

## Browser URI handler

The existing XDG helper can still register the two Roblox URI schemes:

    bash tools/install-roblox-handler.sh ./roblox

The current C++ launcher accepts `--libroblox`, `--assets`, `--width`, and `--height`.
