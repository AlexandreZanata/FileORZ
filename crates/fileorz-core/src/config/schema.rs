//! Stable config schema (v1) — see `docs/CONFIG-KEY-MAP.md`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Extension toggle map (keys include leading `.`).
pub type ExtMap = BTreeMap<String, bool>;

/// Categories keyed by on-disk folder id (e.g. `documentos`).
pub type CategoryMap = BTreeMap<String, ExtMap>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    /// Organize root; `None` means unset (legacy placeholder stripped).
    #[serde(default)]
    pub folder: Option<String>,
    #[serde(default = "default_interval")]
    pub interval_minutes: u32,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default)]
    pub advanced_organize: bool,
    #[serde(default)]
    pub auto_delete: AutoDeleteConfig,
    #[serde(default)]
    pub folder_delete: FolderDeleteConfig,
    #[serde(default)]
    pub categories: CategoryMap,
}

fn default_interval() -> u32 {
    5
}

fn default_locale() -> String {
    "en".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AutoDeleteConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub to_trash: bool,
    #[serde(default)]
    pub permanent: bool,
    #[serde(default)]
    pub by_created: bool,
    #[serde(default)]
    pub by_modified: bool,
    #[serde(default = "default_max_age")]
    pub max_age_days: u32,
}

fn default_max_age() -> u32 {
    15
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FolderDeleteConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub to_trash: bool,
    #[serde(default)]
    pub permanent: bool,
    #[serde(default)]
    pub orz_folders_only: bool,
    #[serde(default)]
    pub everything: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        defaults()
    }
}

/// Upstream-aligned defaults for Linux (categories subset of AutoBuild).
#[must_use]
pub fn defaults() -> AppConfig {
    serde_json::from_str(include_str!("../../defaults/config.v1.json"))
        .expect("bundled config.v1.json must parse")
}
