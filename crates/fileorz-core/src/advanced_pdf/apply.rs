//! Apply planned PDF keyword moves (skips are no-ops).

use crate::advanced_pdf::plan::PdfAction;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplyError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("missing source: {0}")]
    MissingSource(String),
}

/// `mkdir -p` destination and `rename` each [`PdfAction::Move`].
pub fn apply_pdf_actions(root: &Path, actions: &[PdfAction]) -> Result<(), ApplyError> {
    for action in actions {
        let PdfAction::Move { from_name, to_rel } = action else {
            continue;
        };
        let from = root.join(from_name);
        if !from.is_file() {
            return Err(ApplyError::MissingSource(from_name.clone()));
        }
        let to = root.join(to_rel);
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&from, &to)?;
    }
    Ok(())
}
