//! Repo / fixture / binary path resolution.

use std::path::{Path, PathBuf};

/// Workspace repository root (`…/FileORZ`).
#[must_use]
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Characterization fixtures root.
#[must_use]
pub fn fixtures_root() -> PathBuf {
    repo_root().join("tests/fixtures")
}

/// Built `fileorz` binary (env override or debug target).
#[must_use]
pub fn fileorz_bin() -> PathBuf {
    if let Ok(p) = std::env::var("FILEORZ_BIN") {
        return PathBuf::from(p);
    }
    let release = repo_root().join("target/release/fileorz");
    if release.is_file() {
        return release;
    }
    repo_root().join("target/debug/fileorz")
}

/// Failure artifact directory (gitignored under `.local/tmp/e2e`).
#[must_use]
pub fn artifact_dir() -> PathBuf {
    if let Ok(p) = std::env::var("FILEORZ_E2E_ARTIFACT_DIR") {
        return PathBuf::from(p);
    }
    repo_root().join(".local/tmp/e2e")
}

/// Ensure `path` exists as a directory.
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}
