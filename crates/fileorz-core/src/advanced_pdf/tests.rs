//! Unit + pdf-keywords golden parity (advanced_pdf then extension).

use crate::advanced_pdf::{
    apply_pdf_actions, last_page_haystack, load_keywords, plan_pdf_actions, PdfAction, SkipReason,
};
use crate::config::{migrate_value, AppConfig};
use crate::organize::{apply_moves, plan_moves};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures")
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

fn load_organize_advanced() -> AppConfig {
    let raw = fs::read_to_string(fixtures_root().join("configs/organize-advanced.json")).unwrap();
    migrate_value(serde_json::from_str(&raw).unwrap()).unwrap()
}

#[derive(Deserialize)]
struct Golden {
    expected_actions: Vec<GoldenAction>,
    expected_present: Vec<String>,
    expected_root_remaining: Vec<String>,
}

#[derive(Deserialize)]
struct GoldenAction {
    action: String,
    from: String,
    to: String,
    #[serde(default)]
    reason: Option<String>,
}

#[test]
fn extract_invoice_contains_nota_fiscal() {
    let path = fixtures_root().join("trees/pdf-keywords/invoice.pdf");
    let text = last_page_haystack(&path).unwrap();
    assert!(text.contains("NOTA FISCAL"));
}

#[test]
fn plan_skips_when_destination_exists() {
    let keywords = load_keywords(&fixtures_root().join("configs/keywords-fixture.json")).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("tree");
    copy_dir_all(&fixtures_root().join("trees/pdf-keywords"), &root).unwrap();
    let actions = plan_pdf_actions(&root, &keywords).unwrap();
    let skip = actions.iter().find(|a| match a {
        PdfAction::Skip { from_name, .. } => from_name == "dup.pdf",
        _ => false,
    });
    assert_eq!(
        skip,
        Some(&PdfAction::Skip {
            from_name: "dup.pdf".into(),
            to_rel: "Nota Fiscal/dup.pdf".into(),
            reason: SkipReason::DestinationExists,
        })
    );
}

#[test]
fn parity_pdf_keywords_pipeline() {
    let golden: Golden = serde_json::from_str(
        &fs::read_to_string(fixtures_root().join("golden/pdf-keywords.json")).unwrap(),
    )
    .unwrap();
    let keywords = load_keywords(&fixtures_root().join("configs/keywords-fixture.json")).unwrap();
    let cfg = load_organize_advanced();
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("tree");
    copy_dir_all(&fixtures_root().join("trees/pdf-keywords"), &root).unwrap();

    let pdf_actions = plan_pdf_actions(&root, &keywords).unwrap();
    apply_pdf_actions(&root, &pdf_actions).unwrap();
    let ext_plans = plan_moves(&root, &cfg).unwrap();
    apply_moves(&root, &ext_plans).unwrap();

    let mut got = BTreeSet::new();
    for a in &pdf_actions {
        match a {
            PdfAction::Move { from_name, to_rel } => {
                got.insert(("move".into(), from_name.clone(), to_rel.clone(), None));
            }
            PdfAction::Skip {
                from_name,
                to_rel,
                reason: _,
            } => {
                got.insert((
                    "skip".into(),
                    from_name.clone(),
                    to_rel.clone(),
                    Some("destination_exists".into()),
                ));
            }
        }
    }
    for p in &ext_plans {
        got.insert(("move".into(), p.from_name.clone(), p.to_rel.clone(), None));
    }
    let want: BTreeSet<_> = golden
        .expected_actions
        .iter()
        .map(|a| {
            (
                a.action.clone(),
                a.from.clone(),
                a.to.clone(),
                a.reason.clone(),
            )
        })
        .collect();
    assert_eq!(got, want);

    for path in &golden.expected_present {
        assert!(root.join(path).is_file(), "missing {path}");
    }
    let remaining: BTreeSet<_> = fs::read_dir(&root)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let want_root: BTreeSet<_> = golden.expected_root_remaining.iter().cloned().collect();
    assert_eq!(remaining, want_root);
}
