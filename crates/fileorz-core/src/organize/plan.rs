//! Pure-ish move planning (reads tree; does not rename).

use crate::config::AppConfig;
use crate::organize::map::extension_map_from_config;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedMove {
    pub from_name: String,
    /// Destination relative to organize root (`Category/EXT/file`).
    pub to_rel: String,
}

#[derive(Debug, Error)]
pub enum PlanError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Plan moves for top-level files under `root` (no filesystem writes).
pub fn plan_moves(root: &Path, config: &AppConfig) -> Result<Vec<PlannedMove>, PlanError> {
    let mapping = extension_map_from_config(config);
    let mut occupied = initial_occupied(root)?;
    let mut plans = Vec::new();
    for name in list_top_level_files(root)? {
        if name.starts_with('.') {
            continue;
        }
        let (stem, ext) = split_name(&name);
        let ext_lower = ext.to_lowercase();
        let category = mapping
            .get(&ext_lower)
            .cloned()
            .unwrap_or_else(|| "OUTROS".into());
        let sub = subfolder_for_ext(&ext);
        let dest_name = unique_name(&stem, &ext, &category, &sub, &mut occupied);
        let to_rel = format!("{category}/{sub}/{dest_name}");
        occupied.insert(PathBuf::from(&to_rel));
        plans.push(PlannedMove {
            from_name: name,
            to_rel,
        });
    }
    Ok(plans)
}

fn list_top_level_files(root: &Path) -> Result<Vec<String>, PlanError> {
    let mut names = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}

fn initial_occupied(root: &Path) -> Result<HashSet<PathBuf>, PlanError> {
    let mut set = HashSet::new();
    collect_rel_paths(root, root, &mut set)?;
    Ok(set)
}

fn collect_rel_paths(root: &Path, dir: &Path, out: &mut HashSet<PathBuf>) -> Result<(), PlanError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
        if entry.file_type()?.is_dir() {
            collect_rel_paths(root, &path, out)?;
        } else if entry.file_type()?.is_file() {
            out.insert(rel);
        }
    }
    Ok(())
}

fn split_name(name: &str) -> (String, String) {
    match name.rfind('.') {
        Some(0) | None => (name.to_string(), String::new()),
        Some(i) => (name[..i].to_string(), name[i..].to_string()),
    }
}

fn subfolder_for_ext(ext: &str) -> String {
    if ext.len() > 1 {
        ext[1..].to_uppercase()
    } else {
        "OUTROS".into()
    }
}

fn unique_name(
    stem: &str,
    ext: &str,
    category: &str,
    sub: &str,
    occupied: &mut HashSet<PathBuf>,
) -> String {
    let mut candidate = format!("{stem}{ext}");
    let mut counter = 1u32;
    loop {
        let rel = PathBuf::from(format!("{category}/{sub}/{candidate}"));
        if !occupied.contains(&rel) {
            return candidate;
        }
        candidate = format!("{stem}_{counter}{ext}");
        counter += 1;
    }
}
