//! Organize unit + golden parity tests.

use crate::config::{migrate_value, AppConfig};
use crate::organize::{apply_moves, plan_moves, PlannedMove};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures")
}

fn load_organize_config() -> AppConfig {
    let raw = fs::read_to_string(fixtures_root().join("configs/organize-basic.json")).unwrap();
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
    migrate_value(value).unwrap()
}

fn copy_tree(name: &str) -> (tempfile::TempDir, PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("tree");
    copy_dir_all(&fixtures_root().join("trees").join(name), &dest).unwrap();
    (tmp, dest)
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

#[derive(Debug, Deserialize)]
struct Golden {
    expected_actions: Vec<GoldenAction>,
    #[serde(default)]
    expected_root_remaining: Vec<String>,
    #[serde(default)]
    expected_present: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GoldenAction {
    from: String,
    to: String,
}

fn load_golden(name: &str) -> Golden {
    let raw = fs::read_to_string(fixtures_root().join("golden").join(name)).unwrap();
    serde_json::from_str(&raw).unwrap()
}

fn assert_plans_match_golden(plans: &[PlannedMove], golden: &Golden) {
    let got: BTreeSet<_> = plans
        .iter()
        .map(|p| (p.from_name.clone(), p.to_rel.clone()))
        .collect();
    let want: BTreeSet<_> = golden
        .expected_actions
        .iter()
        .map(|a| (a.from.clone(), a.to.clone()))
        .collect();
    assert_eq!(got, want);
}

#[test]
fn plan_outros_for_unknown_extension() {
    let cfg = load_organize_config();
    let (_tmp, root) = copy_tree("tiny-mixed");
    let plans = plan_moves(&root, &cfg).unwrap();
    let weird = plans.iter().find(|p| p.from_name == "weird.xyz").unwrap();
    assert_eq!(weird.to_rel, "OUTROS/XYZ/weird.xyz");
}

#[test]
fn plan_skips_dotfiles() {
    let cfg = load_organize_config();
    let (_tmp, root) = copy_tree("tiny-mixed");
    let plans = plan_moves(&root, &cfg).unwrap();
    assert!(!plans.iter().any(|p| p.from_name == ".hidden"));
}

#[test]
fn parity_tiny_mixed_plan_and_apply() {
    let cfg = load_organize_config();
    let golden = load_golden("tiny-mixed.json");
    let (_tmp, root) = copy_tree("tiny-mixed");
    let plans = plan_moves(&root, &cfg).unwrap();
    assert_plans_match_golden(&plans, &golden);
    apply_moves(&root, &plans).unwrap();
    for a in &golden.expected_actions {
        assert!(root.join(&a.to).is_file(), "missing {}", a.to);
        assert!(!root.join(&a.from).exists(), "source left {}", a.from);
    }
    let remaining: BTreeSet<_> = fs::read_dir(&root)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let want: BTreeSet<_> = golden.expected_root_remaining.iter().cloned().collect();
    assert_eq!(remaining, want);
}

#[test]
fn parity_collision_plan_and_apply() {
    let cfg = load_organize_config();
    let golden = load_golden("collision.json");
    let (_tmp, root) = copy_tree("collision");
    let plans = plan_moves(&root, &cfg).unwrap();
    assert_plans_match_golden(&plans, &golden);
    apply_moves(&root, &plans).unwrap();
    for path in &golden.expected_present {
        assert!(root.join(path).is_file(), "missing {path}");
    }
}
