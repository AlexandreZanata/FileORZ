//! Characterization / golden parity helpers + matrix lock tests.
//!
//! Fixtures: `tests/fixtures/` — see `docs/CHARACTERIZATION.md`.
//! Matrix: `docs/PARITY-REPORT.md`.

use std::path::{Path, PathBuf};

/// Crate package name (smoke helper for workspace wiring).
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// Repository root (workspace).
#[must_use]
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Characterization fixtures root.
#[must_use]
pub fn fixtures_root() -> PathBuf {
    repo_root().join("tests/fixtures")
}

/// Behavior IDs that must appear in `docs/PARITY-REPORT.md`.
pub const BEHAVIOR_IDS: &[&str] = &[
    "B-01", "B-02", "B-03", "B-04", "B-05", "B-10", "B-11", "B-12", "B-13", "B-14", "B-20", "B-21",
    "B-22", "B-23", "B-30", "B-31", "B-32", "B-33", "B-34",
];

/// Required golden JSON basenames under `tests/fixtures/golden/`.
pub const GOLDEN_CASES: &[&str] = &[
    "tiny-mixed.json",
    "collision.json",
    "pdf-keywords.json",
    "aged-files.json",
];

/// True when `path` is an existing file.
#[must_use]
pub fn is_file(path: &Path) -> bool {
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn crate_name_matches() {
        assert_eq!(crate_name(), "fileorz-parity");
    }

    #[test]
    fn golden_fixtures_exist() {
        let root = fixtures_root().join("golden");
        for name in GOLDEN_CASES {
            let path = root.join(name);
            assert!(is_file(&path), "missing golden {}", path.display());
        }
    }

    #[test]
    fn parity_report_exists() {
        let path = repo_root().join("docs/PARITY-REPORT.md");
        assert!(is_file(&path), "missing {}", path.display());
    }

    /// Optional lock: every catalog ID appears in the committed parity report.
    /// Run: `cargo test -p fileorz-parity -- --include-ignored`
    #[test]
    #[ignore = "optional matrix lock — include-ignored"]
    fn ignored_parity_report_lists_all_behavior_ids() {
        let path = repo_root().join("docs/PARITY-REPORT.md");
        let text = fs::read_to_string(&path).expect("read PARITY-REPORT");
        for id in BEHAVIOR_IDS {
            assert!(
                text.contains(id),
                "PARITY-REPORT.md missing behavior id {id}"
            );
        }
        assert!(
            text.contains("Intentional differences"),
            "missing intentional differences section"
        );
        assert!(
            text.contains("Python out of the release path"),
            "missing Python release-path decision"
        );
    }
}
