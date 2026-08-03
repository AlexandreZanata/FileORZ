//! Apply planned deletes (permanent or via trash callback).

use crate::autodelete::mode::DeleteMode;
use crate::autodelete::plan::PlannedDelete;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplyError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("trash: {0}")]
    Trash(String),
    #[error("missing: {0}")]
    Missing(String),
}

/// Backend for FreeDesktop (or test) trash.
pub trait TrashSink: Send + Sync {
    /// # Errors
    /// Returns an error if the trash operation fails.
    fn trash_file(&self, path: &Path) -> Result<(), String>;
}

/// Apply deletes. `trash` is required when any plan uses [`DeleteMode::Trash`].
pub fn apply_deletes(
    root: &Path,
    deletes: &[PlannedDelete],
    trash: Option<&dyn TrashSink>,
) -> Result<(), ApplyError> {
    for item in deletes {
        let path = root.join(&item.from_rel);
        if !path.is_file() {
            return Err(ApplyError::Missing(item.from_rel.clone()));
        }
        match item.mode {
            DeleteMode::Permanent => fs::remove_file(&path)?,
            DeleteMode::Trash => {
                let sink = trash.ok_or_else(|| {
                    ApplyError::Trash("trash sink required for Trash mode".into())
                })?;
                sink.trash_file(&path).map_err(ApplyError::Trash)?;
            }
        }
    }
    Ok(())
}
