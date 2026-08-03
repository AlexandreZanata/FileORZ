//! Single organize tick — B-10 pipeline order.

use crate::advanced_pdf::{apply_pdf_actions, plan_pdf_actions, KeywordGroups};
use crate::autodelete::{apply_deletes, plan_deletes, TrashSink};
use crate::config::AppConfig;
use crate::organize::{apply_moves, plan_moves};
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TickReport {
    pub deletes: usize,
    pub pdf_moves: usize,
    pub pdf_skips: usize,
    pub ext_moves: usize,
}

#[derive(Debug, Error)]
pub enum TickError {
    #[error("folder missing or not a directory: {0}")]
    BadFolder(String),
    #[error("folder not writable: {0}")]
    NotWritable(String),
    #[error("autodelete: {0}")]
    AutoDelete(String),
    #[error("advanced pdf: {0}")]
    AdvancedPdf(String),
    #[error("organize: {0}")]
    Organize(String),
}

/// Validate organize root exists, is a directory, and is writable.
pub fn validate_root(root: &Path) -> Result<(), TickError> {
    if !root.is_dir() {
        return Err(TickError::BadFolder(root.display().to_string()));
    }
    let probe = root.join(".fileorz-write-probe");
    match fs::write(&probe, b"ok") {
        Ok(()) => {
            let _ = fs::remove_file(&probe);
            Ok(())
        }
        Err(_) => Err(TickError::NotWritable(root.display().to_string())),
    }
}

/// Run one tick: auto-delete → advanced PDF → extension organize.
pub fn run_tick(
    root: &Path,
    config: &AppConfig,
    keywords: &KeywordGroups,
    now: SystemTime,
    trash: Option<&dyn TrashSink>,
) -> Result<TickReport, TickError> {
    validate_root(root)?;
    let mut report = TickReport::default();

    let deletes =
        plan_deletes(root, config, now).map_err(|e| TickError::AutoDelete(e.to_string()))?;
    apply_deletes(root, &deletes, trash).map_err(|e| TickError::AutoDelete(e.to_string()))?;
    report.deletes = deletes.len();

    if config.advanced_organize && !keywords.is_empty() {
        let actions =
            plan_pdf_actions(root, keywords).map_err(|e| TickError::AdvancedPdf(e.to_string()))?;
        apply_pdf_actions(root, &actions).map_err(|e| TickError::AdvancedPdf(e.to_string()))?;
        report.pdf_moves = actions
            .iter()
            .filter(|a| matches!(a, crate::advanced_pdf::PdfAction::Move { .. }))
            .count();
        report.pdf_skips = actions.len() - report.pdf_moves;
    }

    let moves = plan_moves(root, config).map_err(|e| TickError::Organize(e.to_string()))?;
    apply_moves(root, &moves).map_err(|e| TickError::Organize(e.to_string()))?;
    report.ext_moves = moves.len();
    Ok(report)
}
