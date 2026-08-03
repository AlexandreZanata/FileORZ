//! Filesystem helpers for e2e fixtures.

use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Recursively copy a directory tree.
pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

/// Write pretty JSON to `path` (creates parent dirs).
pub fn write_json<T: Serialize>(path: &Path, value: &T) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let raw = serde_json::to_string_pretty(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(path, raw)
}

/// Set file mtime to `now - days_ago` via `touch -d @epoch` (Linux).
pub fn set_mtime_days_ago(path: &Path, days_ago: u64, now: SystemTime) -> std::io::Result<()> {
    let stamp = now
        .checked_sub(Duration::from_secs(days_ago.saturating_mul(86_400)))
        .ok_or_else(|| std::io::Error::other("mtime underflow"))?;
    let secs = stamp
        .duration_since(UNIX_EPOCH)
        .map_err(|_| std::io::Error::other("mtime before epoch"))?
        .as_secs();
    let status = std::process::Command::new("touch")
        .args(["-d", &format!("@{secs}")])
        .arg(path)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "touch failed for {}",
            path.display()
        )))
    }
}
