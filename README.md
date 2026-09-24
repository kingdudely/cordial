# Cordial Roblox Runner

This fork is a standalone Linux Roblox runner.

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
