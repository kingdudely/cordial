use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub enum Source {
    Explicit,
    Cache,
    Fetched,
    Nothing(String),
}

impl std::fmt::Display for Source {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Explicit => write!(formatter, "explicit"),
            Self::Cache => write!(formatter, "cache"),
            Self::Fetched => write!(formatter, "fetched"),
            Self::Nothing(error) => write!(formatter, "nothing: {error}"),
        }
    }
}

const URL: &str =
    "https://clientsettingscdn.roblox.com/v2/settings-compressed/application/GoogleAndroidApp.zst";

fn cache_path() -> PathBuf {
    crate::profile::active().join("clientsettings.json")
}

fn fresh(path: &Path) -> Option<String> {
    let metadata = std::fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let age = SystemTime::now().duration_since(modified).ok()?;

    (age < Duration::from_secs(6 * 60 * 60))
        .then(|| std::fs::read_to_string(path).ok())
        .flatten()
}

fn fetch() -> Result<String, String> {
    let mut response = ureq::get(URL)
        .call()
        .map_err(|error| error.to_string())?;

    let bytes = response
        .body_mut()
        .read_to_vec()
        .map_err(|error| error.to_string())?;

    let decompressed = zstd::stream::decode_all(std::io::Cursor::new(bytes))
        .map_err(|error| error.to_string())?;

    String::from_utf8(decompressed).map_err(|error| error.to_string())
}

pub fn load(path: Option<&str>) -> Option<String> {
    load_reporting(path).0
}

pub fn load_reporting(path: Option<&str>) -> (Option<String>, Source) {
    if let Some(path) = path {
        return match std::fs::read_to_string(path) {
            Ok(contents) => (Some(contents), Source::Explicit),
            Err(error) => (None, Source::Nothing(error.to_string())),
        };
    }

    let cache = cache_path();

    if let Some(contents) = fresh(&cache) {
        return (Some(contents), Source::Cache);
    }

    match fetch() {
        Ok(contents) => {
            if let Some(parent) = cache.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            let _ = std::fs::write(&cache, &contents);

            (Some(contents), Source::Fetched)
        }
        Err(error) => (None, Source::Nothing(error)),
    }
}
