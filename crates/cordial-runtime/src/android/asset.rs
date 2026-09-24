use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

struct Asset {
    bytes: &'static [u8],
}

struct Manager {
    root: PathBuf,
    cache: Mutex<HashMap<String, &'static [u8]>>,
}

static MANAGER: OnceLock<Manager> = OnceLock::new();
static REQUESTED: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn set_asset_dir(path: &Path) -> Result<(), String> {
    let root = path
        .canonicalize()
        .map_err(|error| format!("{}: {error}", path.display()))?;

    if !root.is_dir() {
        return Err(format!("{} is not a directory", root.display()));
    }

    MANAGER
        .set(Manager {
            root,
            cache: Mutex::new(HashMap::new()),
        })
        .map_err(|_| "asset directory already configured".to_string())
}

pub fn set_asset_root(path: &Path) {
    let _ = set_asset_dir(path);
}

pub fn set_apk(_path: &Path) -> Result<(), String> {
    Err("APK input is disabled; provide assets/ beside roblox".into())
}

pub fn is_configured() -> bool {
    MANAGER.get().is_some()
}

fn valid_asset_name(name: &str) -> bool {
    let path = Path::new(name);

    !path.is_absolute()
        && !path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
}

fn read(name: &str) -> Option<&'static [u8]> {
    if !valid_asset_name(name) {
        return None;
    }

    let manager = MANAGER.get()?;

    if let Ok(cache) = manager.cache.lock() {
        if let Some(bytes) = cache.get(name) {
            return Some(bytes);
        }
    }

    let bytes = std::fs::read(manager.root.join(name)).ok()?;
    let bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());

    if let Ok(mut cache) = manager.cache.lock() {
        cache.insert(name.to_string(), bytes);
    }

    if let Ok(mut requested) = REQUESTED.lock() {
        requested.push(name.to_string());
    }

    Some(bytes)
}

extern "C" fn from_java(_env: *mut c_void, _obj: *mut c_void) -> *mut c_void {
    MANAGER
        .get()
        .map_or(std::ptr::null_mut(), |manager| {
            manager as *const Manager as *mut c_void
        })
}

extern "C" fn open(_manager: *mut c_void, path: *const c_char, _mode: c_int) -> *mut c_void {
    let Some(path) = (unsafe { (!path.is_null()).then(|| CStr::from_ptr(path)) })
        .and_then(|value| value.to_str().ok())
    else {
        return std::ptr::null_mut();
    };

    read(path).map_or(std::ptr::null_mut(), |bytes| {
        Box::into_raw(Box::new(Asset { bytes })) as *mut c_void
    })
}

extern "C" fn len(asset: *mut c_void) -> i64 {
    if asset.is_null() {
        0
    } else {
        unsafe { (*(asset as *const Asset)).bytes.len() as i64 }
    }
}

extern "C" fn buf(asset: *mut c_void) -> *const c_void {
    if asset.is_null() {
        std::ptr::null()
    } else {
        unsafe { (*(asset as *const Asset)).bytes.as_ptr() as *const c_void }
    }
}

extern "C" fn close(asset: *mut c_void) {
    if !asset.is_null() {
        unsafe {
            drop(Box::from_raw(asset as *mut Asset));
        }
    }
}

extern "C" fn fd(
    asset: *mut c_void,
    start: *mut i64,
    out_len: *mut i64,
) -> c_int {
    if asset.is_null() {
        return -1;
    }

    extern "C" {
        fn memfd_create(name: *const c_char, flags: u32) -> c_int;
        fn write(fd: c_int, buf: *const c_void, len: usize) -> isize;
        fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    }

    let bytes = unsafe { (*(asset as *const Asset)).bytes };

    let file = unsafe { memfd_create(c"roblox-asset".as_ptr(), 0) };
    if file < 0 {
        return -1;
    }

    let written = unsafe { write(file, bytes.as_ptr() as *const c_void, bytes.len()) };
    if written < 0 {
        return -1;
    }

    unsafe {
        lseek(file, 0, 0);
    }

    if !start.is_null() {
        unsafe {
            *start = 0;
        }
    }

    if !out_len.is_null() {
        unsafe {
            *out_len = bytes.len() as i64;
        }
    }

    file
}

pub fn overrides() -> Vec<(&'static str, *mut c_void)> {
    macro_rules! function {
        ($name:literal, $function:expr) => {
            ($name, $function as *const () as *mut c_void)
        };
    }

    vec![
        function!("AAssetManager_fromJava", from_java),
        function!("AAssetManager_open", open),
        function!("AAsset_getBuffer", buf),
        function!("AAsset_getLength", len),
        function!("AAsset_close", close),
        function!("AAsset_openFileDescriptor", fd),
    ]
}

pub fn read_asset(name: &str) -> Option<&'static [u8]> {
    read(name)
}

pub fn probe(name: &str) -> Result<usize, String> {
    read(name)
        .map(|bytes| bytes.len())
        .ok_or_else(|| format!("asset not found: {name}"))
}

pub fn trace_path() -> PathBuf {
    crate::profile::active().join("asset-trace.log")
}

pub fn write_trace(path: &Path) -> std::io::Result<usize> {
    let mut requested = REQUESTED.lock().unwrap();
    requested.sort();
    requested.dedup();

    std::fs::write(path, requested.join("\n"))?;
    Ok(requested.len())
}

pub fn shadow_report() -> Vec<String> {
    Vec::new()
}

pub fn start_watcher() -> bool {
    false
}

pub fn index() -> Index {
    Index
}

pub struct Index;

impl Index {
    pub fn len(&self) -> usize {
        0
    }

    pub fn is_empty(&self) -> bool {
        true
    }
}
