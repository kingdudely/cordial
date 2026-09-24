use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum Source {
    User,
    System,
}

impl Source {
    pub fn describe(&self) -> String {
        match self {
            Self::User => "user".into(),
            Self::System => "system".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Resolved {
    pub value: String,
    pub source: Source,
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub source: Source,
    pub values: BTreeMap<String, String>,
}

pub fn user_path_in(dir: &Path) -> PathBuf {
    dir.join("flags.json")
}

pub fn user_path() -> PathBuf {
    user_path_in(&crate::profile::active())
}

pub fn parse_user_text(text: &str) -> Result<BTreeMap<String, String>, String> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|error| error.to_string())?;

    let object = value
        .as_object()
        .ok_or("flags.json must be an object")?;

    Ok(object
        .iter()
        .map(|(key, value)| {
            let value = match value {
                serde_json::Value::String(string) => string.clone(),
                _ => value.to_string(),
            };

            (key.clone(), value)
        })
        .collect())
}

pub fn read_layer(path: &Path, source: Source) -> Option<Layer> {
    let text = std::fs::read_to_string(path).ok()?;

    Some(Layer {
        source,
        values: parse_user_text(&text).ok()?,
    })
}

pub fn resolve(layers: &[Layer]) -> BTreeMap<String, Resolved> {
    let mut resolved = BTreeMap::new();

    for layer in layers {
        for (key, value) in &layer.values {
            resolved.insert(
                key.clone(),
                Resolved {
                    value: value.clone(),
                    source: layer.source.clone(),
                },
            );
        }
    }

    resolved
}

pub fn collect() -> Vec<Layer> {
    read_layer(&user_path(), Source::User)
        .into_iter()
        .collect()
}

pub fn write_user_layer(
    dir: &Path,
    values: &BTreeMap<String, String>,
) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|error| error.to_string())?;

    let mut object = serde_json::Map::new();

    for (key, value) in values {
        object.insert(
            key.clone(),
            serde_json::Value::String(value.clone()),
        );
    }

    let text =
        serde_json::to_string_pretty(&object).map_err(|error| error.to_string())?;

    std::fs::write(user_path_in(dir), text)
        .map_err(|error| error.to_string())
}

pub fn report(_resolved: &BTreeMap<String, Resolved>) {}
