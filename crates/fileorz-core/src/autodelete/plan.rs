//! Plan auto-delete candidates under category/EXT folders.

use crate::autodelete::age::{exceeds_max_age, read_birthtime, read_mtime};
use crate::autodelete::mode::{resolve_delete_mode, DeleteMode};
use crate::config::AppConfig;
use crate::organize::capitalize_category;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedDelete {
    pub from_rel: String,
    pub mode: DeleteMode,
}

#[derive(Debug, Error)]
pub enum PlanError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Scan `Category/SUB` only; skip when master off or no destination mode.
pub fn plan_deletes(
    root: &Path,
    config: &AppConfig,
    now: SystemTime,
) -> Result<Vec<PlannedDelete>, PlanError> {
    let Some(mode) = resolve_delete_mode(&config.auto_delete) else {
        return Ok(Vec::new());
    };
    let ad = &config.auto_delete;
    if !ad.by_created && !ad.by_modified {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for category_id in config.categories.keys() {
        plan_category(root, category_id, ad, mode, now, &mut out)?;
    }
    out.sort_by(|a, b| a.from_rel.cmp(&b.from_rel));
    Ok(out)
}

fn plan_category(
    root: &Path,
    category_id: &str,
    ad: &crate::config::AutoDeleteConfig,
    mode: DeleteMode,
    now: SystemTime,
    out: &mut Vec<PlannedDelete>,
) -> Result<(), PlanError> {
    let cat_name = capitalize_category(category_id);
    let cat_dir = root.join(&cat_name);
    if !cat_dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(&cat_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let sub_norm = entry
            .file_name()
            .to_string_lossy()
            .to_uppercase()
            .replace('.', "");
        plan_folder(&entry.path(), &cat_name, &sub_norm, ad, mode, now, out)?;
    }
    Ok(())
}

fn plan_folder(
    folder: &Path,
    cat_name: &str,
    sub_norm: &str,
    ad: &crate::config::AutoDeleteConfig,
    mode: DeleteMode,
    now: SystemTime,
    out: &mut Vec<PlannedDelete>,
) -> Result<(), PlanError> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let path = entry.path();
        if !file_is_stale(&path, ad, now)? {
            continue;
        }
        let rel = PathBuf::from(cat_name)
            .join(sub_norm)
            .join(entry.file_name())
            .to_string_lossy()
            .replace('\\', "/");
        out.push(PlannedDelete {
            from_rel: rel,
            mode,
        });
    }
    Ok(())
}

fn file_is_stale(
    path: &Path,
    ad: &crate::config::AutoDeleteConfig,
    now: SystemTime,
) -> Result<bool, PlanError> {
    if ad.by_modified {
        let mtime = read_mtime(path)?;
        if exceeds_max_age(now, mtime, ad.max_age_days) {
            return Ok(true);
        }
    }
    if ad.by_created {
        if let Some(birth) = read_birthtime(path)? {
            if exceeds_max_age(now, birth, ad.max_age_days) {
                return Ok(true);
            }
        }
        // Birthtime missing → skip create check (ADR-0005).
    }
    Ok(false)
}
