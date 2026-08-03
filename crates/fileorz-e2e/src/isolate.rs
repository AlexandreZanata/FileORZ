//! Temporary HOME + XDG Base Directory isolation for child processes.

use crate::paths::ensure_dir;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Isolated home tree; apply to `Command` via [`IsolatedHome::apply`].
pub struct IsolatedHome {
    _tmp: TempDir,
    home: PathBuf,
    config: PathBuf,
    data: PathBuf,
    cache: PathBuf,
    state: PathBuf,
}

impl IsolatedHome {
    /// Create a fresh temp HOME with XDG subdirs.
    pub fn new() -> std::io::Result<Self> {
        let tmp = TempDir::new()?;
        let home = tmp.path().to_path_buf();
        let config = home.join(".config");
        let data = home.join(".local/share");
        let cache = home.join(".cache");
        let state = home.join(".local/state");
        for p in [&config, &data, &cache, &state] {
            ensure_dir(p)?;
        }
        ensure_dir(&config.join("fileorz"))?;
        Ok(Self {
            _tmp: tmp,
            home,
            config,
            data,
            cache,
            state,
        })
    }

    #[must_use]
    pub fn home(&self) -> &Path {
        &self.home
    }

    #[must_use]
    pub fn config_home(&self) -> &Path {
        &self.config
    }

    #[must_use]
    pub fn data_home(&self) -> &Path {
        &self.data
    }

    /// Config JSON path under this isolation (`…/fileorz/config.json`).
    #[must_use]
    pub fn config_json(&self) -> PathBuf {
        self.config.join("fileorz/config.json")
    }

    /// Apply HOME / XDG_* to a child process command.
    pub fn apply<'a>(&self, cmd: &'a mut std::process::Command) -> &'a mut std::process::Command {
        cmd.env("HOME", &self.home)
            .env("XDG_CONFIG_HOME", &self.config)
            .env("XDG_DATA_HOME", &self.data)
            .env("XDG_CACHE_HOME", &self.cache)
            .env("XDG_STATE_HOME", &self.state)
            .env_remove("FILEORZ_CONFIG")
    }
}
