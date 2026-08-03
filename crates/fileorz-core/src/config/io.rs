//! Load / save config with atomic replace and legacy backup.

use crate::config::migrate::{looks_legacy, migrate_value};
use crate::config::schema::AppConfig;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("migrate: {0}")]
    Migrate(String),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

/// Outcome of loading a config file (migration may rewrite the file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadOutcome {
    pub config: AppConfig,
    pub migrated: bool,
    pub backup_path: Option<PathBuf>,
}

/// Parse JSON bytes (legacy or stable) into [`AppConfig`].
pub fn parse_config_json(bytes: &[u8]) -> Result<AppConfig> {
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    migrate_value(value).map_err(ConfigError::Migrate)
}

/// Load from `path`. If legacy, backup once and rewrite stable JSON atomically.
pub fn load_config_file(path: &Path) -> Result<LoadOutcome> {
    let bytes = fs::read(path)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
    if !looks_legacy(&value) {
        let config = serde_json::from_value(value)?;
        return Ok(LoadOutcome {
            config,
            migrated: false,
            backup_path: None,
        });
    }
    let config = migrate_value(value).map_err(ConfigError::Migrate)?;
    let backup = backup_legacy_once(path, &bytes)?;
    atomic_write_json(path, &config)?;
    Ok(LoadOutcome {
        config,
        migrated: true,
        backup_path: Some(backup),
    })
}

/// Write stable config via temp file + `sync_all` + rename.
pub fn atomic_write_json(path: &Path, config: &AppConfig) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    {
        let mut file = fs::File::create(&tmp)?;
        let body = serde_json::to_vec_pretty(config)?;
        file.write_all(&body)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

fn backup_legacy_once(path: &Path, original: &[u8]) -> Result<PathBuf> {
    let bak = legacy_backup_path(path);
    if !bak.exists() {
        if let Some(parent) = bak.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&bak, original)?;
    }
    Ok(bak)
}

#[must_use]
pub fn legacy_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("config.json.bak-legacy")
}
