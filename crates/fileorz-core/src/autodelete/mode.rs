//! Auto-delete destination mode (B-22).

use crate::config::AutoDeleteConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteMode {
    Trash,
    Permanent,
}

/// Resolve delete destination. If both flags set, **trash wins**.
/// Returns `None` when disabled or neither destination is selected.
#[must_use]
pub fn resolve_delete_mode(cfg: &AutoDeleteConfig) -> Option<DeleteMode> {
    if !cfg.enabled {
        return None;
    }
    if cfg.to_trash {
        return Some(DeleteMode::Trash);
    }
    if cfg.permanent {
        return Some(DeleteMode::Permanent);
    }
    None
}

#[cfg(test)]
mod mode_tests {
    use super::*;
    use crate::config::AutoDeleteConfig;

    #[test]
    fn trash_wins_when_both_true() {
        let cfg = AutoDeleteConfig {
            enabled: true,
            to_trash: true,
            permanent: true,
            ..AutoDeleteConfig::default()
        };
        assert_eq!(resolve_delete_mode(&cfg), Some(DeleteMode::Trash));
    }

    #[test]
    fn disabled_yields_none() {
        let cfg = AutoDeleteConfig {
            enabled: false,
            to_trash: true,
            permanent: false,
            ..AutoDeleteConfig::default()
        };
        assert_eq!(resolve_delete_mode(&cfg), None);
    }
}
