//! XDG base-dir helpers for FileORZ (ADR-0004).

use std::env;
use std::path::PathBuf;

const APP: &str = "fileorz";

fn home_dir() -> PathBuf {
    env::var_os("HOME")
        .map(PathBuf::from)
        .expect("HOME must be set on Linux")
}

/// `$XDG_CONFIG_HOME/fileorz` or `~/.config/fileorz`.
#[must_use]
pub fn config_dir() -> PathBuf {
    xdg_dir("XDG_CONFIG_HOME", ".config").join(APP)
}

/// `$XDG_DATA_HOME/fileorz` or `~/.local/share/fileorz`.
#[must_use]
pub fn data_dir() -> PathBuf {
    xdg_dir("XDG_DATA_HOME", ".local/share").join(APP)
}

#[must_use]
pub fn config_json_path() -> PathBuf {
    config_dir().join("config.json")
}

#[must_use]
pub fn keywords_json_path() -> PathBuf {
    config_dir().join("keywords.json")
}

#[must_use]
pub fn autostart_desktop_path() -> PathBuf {
    xdg_dir("XDG_CONFIG_HOME", ".config")
        .join("autostart")
        .join("fileorz.desktop")
}

fn xdg_dir(env_key: &str, fallback_under_home: &str) -> PathBuf {
    match env::var_os(env_key) {
        Some(val) if !val.is_empty() => PathBuf::from(val),
        _ => home_dir().join(fallback_under_home),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn config_dir_respects_xdg_config_home() {
        let _guard = ENV_LOCK.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        env::set_var("XDG_CONFIG_HOME", tmp.path());
        assert_eq!(config_dir(), tmp.path().join("fileorz"));
        assert_eq!(config_json_path(), tmp.path().join("fileorz/config.json"));
        env::remove_var("XDG_CONFIG_HOME");
    }

    #[test]
    fn data_dir_respects_xdg_data_home() {
        let _guard = ENV_LOCK.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        env::set_var("XDG_DATA_HOME", tmp.path());
        assert_eq!(data_dir(), tmp.path().join("fileorz"));
        env::remove_var("XDG_DATA_HOME");
    }

    #[test]
    fn keywords_and_autostart_paths() {
        let _guard = ENV_LOCK.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        env::set_var("XDG_CONFIG_HOME", tmp.path());
        assert_eq!(
            keywords_json_path(),
            tmp.path().join("fileorz/keywords.json")
        );
        assert_eq!(
            autostart_desktop_path(),
            tmp.path().join("autostart/fileorz.desktop")
        );
        env::remove_var("XDG_CONFIG_HOME");
    }
}
