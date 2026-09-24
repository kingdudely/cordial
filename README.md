# Cordial Roblox Runner

This fork is a standalone Linux Roblox runner.

## Standalone runner usage

The fork's normal launch path now uses the configuration that previously required --host-libc --game-activity by default:

    ./roblox

The default files are ./libroblox.so and ./assets. Override them with --libroblox <path> and --assets <path> when needed. A normal launch stays open until the window is closed; --run 0 is therefore no longer necessary. The old --host-libc, --jni-onload, and --game-activity switches remain accepted for compatibility.

Roblox browser launch links can also be passed directly as the first argument. Both the desktop roblox-player: form and the roblox:// form are accepted:

    ./roblox 'roblox-player:1+launchmode:play+...'
    ./roblox 'roblox://experiences/start?placeId=1818'

To make the browser's Play button launch the same binary, register both XDG URI schemes once:

    bash tools/install-roblox-handler.sh ./roblox

That writes a desktop handler for x-scheme-handler/roblox and x-scheme-handler/roblox-player and points both at the supplied executable.

The runtime expects this exact layout:

```
build/
├── roblox
├── libroblox.so
└── assets/
    ├── content/
    ├── ssl/
    ├── android/
    └── ...
```

It does not download an APK at launch. It uses the supplied `libroblox.so` and reads the supplied `assets/` directory directly.

Build:

```bash
cargo build --release --bin roblox
```
