use std::path::Path;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Store {
    File,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Cookies,
    Identity,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Cookies => "cookies",
            Self::Identity => "identity",
        }
    }
}

#[derive(Clone, Copy)]
pub struct SnapshotRequest<'a> {
    pub store: Store,
    pub profile_dir: &'a Path,
    pub kind: Kind,
}

pub fn active() -> Store {
    static STORE: OnceLock<Store> = OnceLock::new();

    *STORE.get_or_init(|| Store::File)
}

pub fn load(_store: Store, dir: &Path, kind: Kind) -> Option<String> {
    std::fs::read_to_string(dir.join(kind.name())).ok()
}

pub fn save(
    _store: Store,
    dir: &Path,
    kind: Kind,
    contents: &str,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;

    let path = dir.join(kind.name());
    let temporary = dir.join(format!(".{}.tmp", kind.name()));

    std::fs::write(&temporary, contents)?;

    std::fs::set_permissions(
        &temporary,
        std::os::unix::fs::PermissionsExt::from_mode(0o600),
    )?;

    std::fs::rename(temporary, path)
}

pub fn erase(_store: Store, dir: &Path, kind: Kind) -> std::io::Result<()> {
    match std::fs::remove_file(dir.join(kind.name())) {
        Ok(()) | Err(_) => Ok(()),
    }
}

pub fn where_kept(_store: Store, dir: &Path, kind: Kind) -> String {
    dir.join(kind.name()).display().to_string()
}

pub fn usable() -> Result<(), String> {
    Ok(())
}

pub fn snapshot(_request: SnapshotRequest<'_>) -> Option<String> {
    None
}
