//! FreeDesktop trash helper (Trash Spec).

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Move `path` into `$XDG_DATA_HOME/Trash` (or `~/.local/share/Trash`).
pub fn move_to_trash(path: &Path) -> io::Result<()> {
    let trash_root = xdg_trash_root();
    let files = trash_root.join("files");
    let info = trash_root.join("info");
    fs::create_dir_all(&files)?;
    fs::create_dir_all(&info)?;
    let name = unique_name(&files, path.file_name().unwrap_or_default())?;
    let dest = files.join(&name);
    let abs = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    fs::rename(path, &dest)?;
    write_trashinfo(&info.join(format!("{name}.trashinfo")), &abs)?;
    Ok(())
}

fn xdg_trash_root() -> PathBuf {
    if let Ok(data) = std::env::var("XDG_DATA_HOME") {
        if !data.is_empty() {
            return PathBuf::from(data).join("Trash");
        }
    }
    PathBuf::from(std::env::var_os("HOME").expect("HOME must be set")).join(".local/share/Trash")
}

fn unique_name(files_dir: &Path, original: &std::ffi::OsStr) -> io::Result<String> {
    let base = original.to_string_lossy().into_owned();
    let mut candidate = base.clone();
    let mut n = 1u32;
    while files_dir.join(&candidate).exists() {
        candidate = format!("{base}.{n}");
        n += 1;
    }
    Ok(candidate)
}

fn write_trashinfo(info_path: &Path, absolute_original: &Path) -> io::Result<()> {
    let mut f = fs::File::create(info_path)?;
    writeln!(f, "[Trash Info]")?;
    writeln!(f, "Path={}", absolute_original.display())?;
    writeln!(f, "DeletionDate={}", deletion_stamp(SystemTime::now()))?;
    Ok(())
}

fn deletion_stamp(t: SystemTime) -> String {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, m, d, hh, mm, ss) = ymd_hms(secs);
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}")
}

fn ymd_hms(secs: u64) -> (i32, u32, u32, u64, u64, u64) {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let (y, m, d) = civil_from_unix_days(days);
    (y, m, d, rem / 3600, (rem % 3600) / 60, rem % 60)
}

/// Days since Unix epoch → Gregorian Y-M-D (Howard Hinnant).
fn civil_from_unix_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

/// Adapter for [`fileorz_core::autodelete::TrashSink`].
pub struct FreedesktopTrash;

impl fileorz_core::autodelete::TrashSink for FreedesktopTrash {
    fn trash_file(&self, path: &Path) -> Result<(), String> {
        move_to_trash(path).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env::lock_env;

    #[test]
    fn move_to_trash_creates_info() {
        let _guard = lock_env();
        let tmp = tempfile::tempdir().unwrap();
        std::env::set_var("XDG_DATA_HOME", tmp.path());
        let src = tmp.path().join("victim.txt");
        fs::write(&src, b"bye").unwrap();
        move_to_trash(&src).unwrap();
        assert!(!src.exists());
        let n = fs::read_dir(tmp.path().join("Trash/files"))
            .unwrap()
            .count();
        assert_eq!(n, 1);
        std::env::remove_var("XDG_DATA_HOME");
    }
}
