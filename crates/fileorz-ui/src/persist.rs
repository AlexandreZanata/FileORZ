//! Load / save XDG config for folder, interval, autostart.

use fileorz_core::config::{atomic_write_json, defaults, load_config_file, AppConfig};
use fileorz_linux::xdg::config_json_path;
use std::path::{Path, PathBuf};

/// Allowed interval minutes on the main shell (parity with CTk dropdown).
pub const INTERVAL_CHOICES: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/// Load config from the default XDG path, or defaults if missing.
#[must_use]
pub fn load_or_default() -> AppConfig {
    load_from_path(&config_json_path())
}

/// Load from an explicit path (tests).
#[must_use]
pub fn load_from_path(path: &Path) -> AppConfig {
    if !path.is_file() {
        return defaults();
    }
    load_config_file(path)
        .map(|outcome| outcome.config)
        .unwrap_or_else(|_| defaults())
}

/// Persist config to the default XDG path.
pub fn save(config: &AppConfig) -> Result<(), String> {
    save_to_path(&config_json_path(), config)
}

/// Persist to an explicit path (tests).
pub fn save_to_path(path: &Path, config: &AppConfig) -> Result<(), String> {
    atomic_write_json(path, config).map_err(|e| e.to_string())
}

/// Clamp minutes into the shell dropdown set.
#[must_use]
pub fn clamp_interval(minutes: u32) -> u32 {
    if INTERVAL_CHOICES.contains(&minutes) {
        minutes
    } else {
        5
    }
}

/// Config path used by the live shell (docs / tests).
#[must_use]
pub fn config_path() -> PathBuf {
    config_json_path()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn clamp_unknown_interval_to_five() {
        assert_eq!(clamp_interval(5), 5);
        assert_eq!(clamp_interval(99), 5);
        assert_eq!(clamp_interval(1), 1);
    }

    #[test]
    fn roundtrip_folder_and_interval() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("config.json");
        let mut cfg = defaults();
        cfg.folder = Some("/tmp/demo".into());
        cfg.interval_minutes = 3;
        save_to_path(&path, &cfg).expect("save");
        let loaded = load_from_path(&path);
        assert_eq!(loaded.folder.as_deref(), Some("/tmp/demo"));
        assert_eq!(loaded.interval_minutes, 3);
    }
}
