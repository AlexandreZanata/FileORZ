//! Filesystem outcome assertions for golden-style e2e checks.

use std::fs;
use std::path::Path;

/// Assert each relative path exists as a file under `root`.
pub fn assert_present(root: &Path, rels: &[&str]) {
    for rel in rels {
        let p = root.join(rel);
        assert!(
            p.is_file(),
            "expected file missing: {} (root={})",
            rel,
            root.display()
        );
    }
}

/// Assert each relative path does not exist under `root`.
pub fn assert_absent(root: &Path, rels: &[&str]) {
    for rel in rels {
        let p = root.join(rel);
        assert!(
            !p.exists(),
            "expected absent still present: {} (root={})",
            rel,
            root.display()
        );
    }
}

/// Assert top-level *file* names under `root` match `expected` (sorted).
/// Category directories created by organize are ignored.
pub fn assert_root_remaining(root: &Path, expected: &[&str]) {
    let mut names: Vec<String> = fs::read_dir(root)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", root.display()))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let mut want: Vec<&str> = expected.to_vec();
    want.sort_unstable();
    assert_eq!(
        names,
        want,
        "root remaining files mismatch under {}",
        root.display()
    );
}
