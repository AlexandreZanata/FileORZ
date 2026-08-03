//! XDG autostart `.desktop` install / remove (ADR-0004).

use crate::xdg::autostart_desktop_path;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_EXEC: &str = "fileorz --tray";

/// Body of `fileorz.desktop` for FreeDesktop autostart.
#[must_use]
pub fn desktop_entry(exec_line: &str) -> String {
    format!(
        "\
[Desktop Entry]
Type=Application
Version=1.5
Name=FileORZ
Comment=File organizer
Exec={exec_line}
Terminal=false
Categories=Utility;Filesystem;
X-GNOME-Autostart-enabled=true
"
    )
}

/// Write `$XDG_CONFIG_HOME/autostart/fileorz.desktop` with `Exec=fileorz --tray`.
pub fn enable() -> io::Result<PathBuf> {
    enable_with_exec(DEFAULT_EXEC)
}

/// Write autostart entry using a custom `Exec=` line (tests / packaging).
pub fn enable_with_exec(exec_line: &str) -> io::Result<PathBuf> {
    let path = autostart_desktop_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, desktop_entry(exec_line))?;
    Ok(path)
}

/// Remove the autostart desktop file if present.
pub fn disable() -> io::Result<()> {
    let path = autostart_desktop_path();
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

/// Whether the autostart desktop file exists.
#[must_use]
pub fn is_enabled() -> bool {
    autostart_desktop_path().is_file()
}

/// Current autostart path (may not exist yet).
#[must_use]
pub fn path() -> PathBuf {
    autostart_desktop_path()
}

/// True if `path` looks like our FileORZ autostart entry.
#[must_use]
pub fn looks_like_fileorz_desktop(path: &Path) -> bool {
    fs::read_to_string(path)
        .map(|s| s.contains("Name=FileORZ") && s.contains("--tray"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env::lock_env;
    use std::env;

    #[test]
    fn enable_disable_under_temp_xdg() {
        let _guard = lock_env();
        let tmp = tempfile::tempdir().unwrap();
        env::set_var("XDG_CONFIG_HOME", tmp.path());
        assert!(!is_enabled());
        let written = enable().unwrap();
        assert_eq!(written, tmp.path().join("autostart/fileorz.desktop"));
        assert!(is_enabled());
        let body = fs::read_to_string(&written).unwrap();
        assert!(body.contains("Exec=fileorz --tray"));
        assert!(body.contains("Name=FileORZ"));
        assert!(looks_like_fileorz_desktop(&written));
        disable().unwrap();
        assert!(!is_enabled());
        disable().unwrap(); // idempotent
        env::remove_var("XDG_CONFIG_HOME");
    }

    #[test]
    fn custom_exec_line() {
        let _guard = lock_env();
        let tmp = tempfile::tempdir().unwrap();
        env::set_var("XDG_CONFIG_HOME", tmp.path());
        let path = enable_with_exec("/opt/fileorz/bin/fileorz --tray").unwrap();
        let body = fs::read_to_string(path).unwrap();
        assert!(body.contains("Exec=/opt/fileorz/bin/fileorz --tray"));
        env::remove_var("XDG_CONFIG_HOME");
    }
}
