//! Start / stop the organize loop from the shell.

use fileorz_core::advanced_pdf::{load_keywords, KeywordGroups};
use fileorz_core::config::AppConfig;
use fileorz_core::scheduler::{validate_root, OrganizerHandle, OrganizerOptions, TickError};
use fileorz_linux::trash::FreedesktopTrash;
use fileorz_linux::xdg::keywords_json_path;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

const STOP_TIMEOUT: Duration = Duration::from_secs(5);

/// Why start was rejected before spawning a worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartReject {
    /// No folder selected.
    MissingFolder,
    /// Path is not a writable directory.
    InvalidFolder,
}

/// Outcome of a start attempt.
pub enum StartOutcome {
    /// Worker is running.
    Started(OrganizerHandle),
    /// Validation failed.
    Rejected(StartReject),
}

/// Validate folder then spawn [`OrganizerHandle`].
#[must_use]
pub fn try_start(config: &AppConfig) -> StartOutcome {
    let Some(folder) = config.folder.as_ref() else {
        return StartOutcome::Rejected(StartReject::MissingFolder);
    };
    let root = PathBuf::from(folder);
    if let Err(err) = validate_root(&root) {
        return StartOutcome::Rejected(map_tick_err(err));
    }
    let interval = Duration::from_secs(u64::from(config.interval_minutes.max(1)) * 60);
    let keywords = keywords_for(config);
    let trash: Arc<dyn fileorz_core::autodelete::TrashSink> = Arc::new(FreedesktopTrash);
    let handle = OrganizerHandle::start(OrganizerOptions {
        root,
        config: config.clone(),
        keywords,
        interval,
        trash: Some(trash),
    });
    StartOutcome::Started(handle)
}

/// Cooperative stop; ignores join timeout (worker still flagged).
pub fn stop(handle: OrganizerHandle) {
    let _ = handle.stop(STOP_TIMEOUT);
}

fn map_tick_err(err: TickError) -> StartReject {
    match err {
        TickError::BadFolder(_) | TickError::NotWritable(_) => StartReject::InvalidFolder,
        _ => StartReject::InvalidFolder,
    }
}

fn keywords_for(config: &AppConfig) -> KeywordGroups {
    if !config.advanced_organize {
        return KeywordGroups::new();
    }
    load_keywords(Path::new(&keywords_json_path())).unwrap_or_else(|_| KeywordGroups::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fileorz_core::config::defaults;
    use tempfile::tempdir;

    #[test]
    fn missing_folder_rejected() {
        let cfg = defaults();
        assert!(matches!(
            try_start(&cfg),
            StartOutcome::Rejected(StartReject::MissingFolder)
        ));
    }

    #[test]
    fn invalid_folder_rejected() {
        let mut cfg = defaults();
        cfg.folder = Some("/no/such/fileorz/path".into());
        assert!(matches!(
            try_start(&cfg),
            StartOutcome::Rejected(StartReject::InvalidFolder)
        ));
    }

    #[test]
    fn valid_folder_starts_and_stops() {
        let dir = tempdir().expect("tempdir");
        let mut cfg = defaults();
        cfg.folder = Some(dir.path().display().to_string());
        cfg.interval_minutes = 1;
        match try_start(&cfg) {
            StartOutcome::Started(handle) => stop(handle),
            StartOutcome::Rejected(reason) => {
                panic!("expected start, got reject: {reason:?}")
            }
        }
    }
}
