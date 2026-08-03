//! Apply planned moves on the filesystem.

use crate::organize::plan::PlannedMove;
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

/// Create destination dirs and `rename` each planned move under `root`.
pub fn apply_moves(root: &Path, moves: &[PlannedMove]) -> Result<(), ApplyError> {
    for mv in moves {
        let from = root.join(&mv.from_name);
        if !from.is_file() {
            return Err(ApplyError::MissingSource(mv.from_name.clone()));
        }
        let to = root.join(&mv.to_rel);
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&from, &to)?;
    }
    Ok(())
}
