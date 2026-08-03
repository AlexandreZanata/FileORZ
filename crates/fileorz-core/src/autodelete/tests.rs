//! Aged-files golden + mtime injection tests.

use crate::autodelete::{apply_deletes, plan_deletes, DeleteMode};
use crate::config::{migrate_value, AppConfig};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures")
}

fn load_autodelete_config() -> AppConfig {
    let raw = fs::read_to_string(fixtures_root().join("configs/autodelete-mtime.json")).unwrap();
    migrate_value(serde_json::from_str(&raw).unwrap()).unwrap()
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
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

fn set_mtime(path: &Path, days_ago: u64, now: SystemTime) {
    use std::time::UNIX_EPOCH;
    let stamp = now - Duration::from_secs(days_ago * 86_400);
    let secs = stamp.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let status = std::process::Command::new("touch")
        .args(["-d", &format!("@{secs}")])
        .arg(path)
        .status()
        .expect("touch must run");
    assert!(status.success(), "touch failed for {}", path.display());
}

#[derive(Deserialize)]
struct Golden {
    expected_actions: Vec<GoldenAction>,
    expected_present: Vec<String>,
    expected_absent: Vec<String>,
}

#[derive(Deserialize)]
struct GoldenAction {
    from: String,
    mode: String,
}

#[test]
fn parity_aged_files_permanent_mtime() {
    let cfg = load_autodelete_config();
    let golden: Golden = serde_json::from_str(
        &fs::read_to_string(fixtures_root().join("golden/aged-files.json")).unwrap(),
    )
    .unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("tree");
    copy_dir_all(&fixtures_root().join("trees/aged-files"), &root).unwrap();
    let now = SystemTime::now();
    set_mtime(&root.join("Documentos/TXT/old.txt"), 30, now);
    set_mtime(&root.join("Documentos/TXT/fresh.txt"), 1, now);
    let plans = plan_deletes(&root, &cfg, now).unwrap();
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].from_rel, golden.expected_actions[0].from);
    assert_eq!(plans[0].mode, DeleteMode::Permanent);
    assert_eq!(golden.expected_actions[0].mode, "permanent");
    apply_deletes(&root, &plans, None).unwrap();
    for p in &golden.expected_present {
        assert!(root.join(p).is_file(), "missing {p}");
    }
    for p in &golden.expected_absent {
        assert!(!root.join(p).exists(), "still present {p}");
    }
}

#[test]
fn root_loose_files_not_scanned() {
    let cfg = load_autodelete_config();
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("tree");
    fs::create_dir_all(root.join("Documentos/TXT")).unwrap();
    fs::write(root.join("loose.txt"), b"x").unwrap();
    fs::write(root.join("Documentos/TXT/old.txt"), b"x").unwrap();
    let now = SystemTime::now();
    set_mtime(&root.join("loose.txt"), 30, now);
    set_mtime(&root.join("Documentos/TXT/old.txt"), 30, now);
    let plans = plan_deletes(&root, &cfg, now).unwrap();
    assert!(plans.iter().all(|p| p.from_rel != "loose.txt"));
    assert!(plans.iter().any(|p| p.from_rel == "Documentos/TXT/old.txt"));
}
