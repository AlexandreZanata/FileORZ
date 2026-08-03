//! Failure artifacts (screenshots) under `.local/tmp/e2e/`.

use crate::paths::{artifact_dir, ensure_dir};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Capture X11 root window to a PNG when `DISPLAY` is set (ImageMagick `import`).
pub fn capture_root_screenshot(label: &str) -> Option<PathBuf> {
    let display = std::env::var_os("DISPLAY")?;
    if display.is_empty() {
        return None;
    }
    let dir = artifact_dir();
    if ensure_dir(&dir).is_err() {
        return None;
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let safe: String = label
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let path = dir.join(format!("fail-{safe}-{ts}.png"));
    let status = std::process::Command::new("import")
        .args(["-window", "root"])
        .arg(&path)
        .status()
        .ok()?;
    if status.success() && path.is_file() {
        eprintln!("[e2e] screenshot → {}", path.display());
        Some(path)
    } else {
        None
    }
}

/// Drop guard that captures a screenshot if the current thread is panicking.
pub struct ArtifactGuard {
    label: String,
}

/// Create a failure screenshot guard for scenario `label`.
#[must_use]
pub fn failure_guard(label: &str) -> ArtifactGuard {
    ArtifactGuard {
        label: label.into(),
    }
}

impl Drop for ArtifactGuard {
    fn drop(&mut self) {
        if std::thread::panicking() {
            let _ = capture_root_screenshot(&self.label);
        }
    }
}
