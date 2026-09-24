//! Standalone Linux Roblox runtime.
#![allow(unsafe_code)]

pub fn window_title(backend: &str) -> String {
    format!("Roblox ({backend})")
}

pub mod android;
pub mod battery;
pub mod bionic;
pub mod client_settings;
pub mod cookies;
pub mod deeplink;
pub mod elf;
pub mod ffi_util;
pub mod flags;
pub mod graphics;
pub mod headless;
pub mod host_window;
pub mod identity;
pub mod linking;
pub mod mimalloc_lib;
pub mod permissions;
pub mod profile;
pub mod refresh;
pub mod secrets;
pub mod storage;
pub mod stubs;
pub mod symtab;
pub mod unimplemented;
pub mod webview;
