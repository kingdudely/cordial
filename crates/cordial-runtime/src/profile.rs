use std::path::PathBuf;
use std::sync::OnceLock;

pub const DEFAULT_NAME: &str = "default";

static ACTIVE: OnceLock<PathBuf> = OnceLock::new();

pub fn root() -> PathBuf {
    std::env::var_os("CORDIAL_PROFILE_ROOT")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("XDG_DATA_HOME")
                .map(|path| PathBuf::from(path).join("roblox"))
        })
        .or_else(|| {
            std::env::var_os("HOME")
                .map(|path| PathBuf::from(path).join(".local/share/roblox"))
        })
        .unwrap_or_else(|| std::env::temp_dir().join("roblox"))
}

pub fn set_active(dir: PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(&dir).map_err(|error| error.to_string())?;

    ACTIVE
        .set(dir)
        .map_err(|_| "profile already selected".into())
}

pub fn active() -> PathBuf {
    ACTIVE
        .get()
        .cloned()
        .unwrap_or_else(|| root().join(DEFAULT_NAME))
}

pub fn is_valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
}

pub fn dir(name: &str) -> Result<PathBuf, String> {
    if !is_valid_name(name) {
        Err(format!("invalid profile name: {name}"))
    } else {
        Ok(root().join(name))
    }
}

pub fn migrate_legacy_layout() -> Option<PathBuf> {
    None
}

pub fn list() -> Vec<String> {
    std::fs::read_dir(root())
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
        .collect()
}
