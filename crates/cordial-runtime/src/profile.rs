use std::path::PathBuf;
use std::sync::OnceLock;
pub const DEFAULT_NAME:&str="default";
static ACTIVE:OnceLock<PathBuf>=OnceLock::new();
pub fn root()->PathBuf{std::env::var_os("CORDIAL_PROFILE_ROOT").map(PathBuf::from).or_else(||std::env::var_os("XDG_DATA_HOME").map(|p|PathBuf::from(p).join("roblox"))).or_else(||std::env::var_os("HOME").map(|p|PathBuf::from(p).join(".local/share/roblox"))).unwrap_or_else(||std::env::temp_dir().join("roblox"))}
pub fn set_active(dir:PathBuf)->Result<(),String>{std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;ACTIVE.set(dir).map_err(|_|"profile already selected".into())}
pub fn active()->PathBuf{ACTIVE.get().cloned().unwrap_or_else(||root().join(DEFAULT_NAME))}
pub fn is_valid_name(n:&str)->bool{!n.is_empty()&&n.len()<=64&&n.chars().all(|c|c.is_ascii_alphanumeric()||c=='-'||c=='_')}
pub fn dir(n:&str)->Result<PathBuf,String>{if !is_valid_name(n){Err(format!("invalid profile name: {n}"))}else{Ok(root().join(n))}}
pub fn migrate_legacy_layout()->Option<PathBuf>{None}
pub fn list()->Vec<String>{std::fs::read_dir(root()).ok().into_iter().flat_map(|it|it.filter_map(Result::ok)).filter(|e|e.path().is_dir()).filter_map(|e|e.file_name().to_str().map(str::to_string)).collect()}
