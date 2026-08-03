//! Linux e2e helpers — isolated HOME/XDG, fixture copy, binary runner, artifacts.

mod artifact;
mod assert_fs;
mod fsutil;
mod isolate;
mod paths;
mod run;

pub use artifact::{capture_root_screenshot, failure_guard, ArtifactGuard};
pub use assert_fs::{assert_absent, assert_present, assert_root_remaining};
pub use fsutil::{copy_dir_all, set_mtime_days_ago, write_json};
pub use isolate::IsolatedHome;
pub use paths::{artifact_dir, fileorz_bin, fixtures_root, repo_root};
pub use run::{run_fileorz, run_fileorz_capture, run_fileorz_env, ui_smoke_ok, CommandOutput};

/// Returns true when `FILEORZ_E2E=1` (set by `scripts/e2e-linux.sh`).
#[must_use]
pub fn e2e_enabled() -> bool {
    matches!(
        std::env::var("FILEORZ_E2E").ok().as_deref(),
        Some("1") | Some("true") | Some("yes")
    )
}

/// Skip helper for integration tests when the e2e harness is not active.
#[must_use]
pub fn skip_unless_e2e(label: &str) -> bool {
    if e2e_enabled() {
        return false;
    }
    eprintln!("[e2e] skip {label} — run via ./scripts/e2e-linux.sh");
    true
}

/// Crate package name (smoke helper for workspace wiring).
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn crate_name_matches() {
        assert_eq!(crate_name(), "fileorz-e2e");
    }
}
