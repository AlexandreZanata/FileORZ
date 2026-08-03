//! Auto-delete UI mutations — mutual exclusion (parity with radio groups).

use fileorz_core::config::AutoDeleteConfig;

/// Allowed deadline days (upstream dropdown).
pub const AGE_DAY_CHOICES: &[u32] = &[5, 10, 15, 20, 25, 30, 60, 120, 180, 240, 300, 360];

/// Select trash destination; clears permanent.
pub fn set_to_trash(cfg: &mut AutoDeleteConfig) {
    cfg.to_trash = true;
    cfg.permanent = false;
}

/// Select permanent delete; clears trash.
pub fn set_permanent(cfg: &mut AutoDeleteConfig) {
    cfg.permanent = true;
    cfg.to_trash = false;
}

/// Age by creation date; clears modified.
pub fn set_by_created(cfg: &mut AutoDeleteConfig) {
    cfg.by_created = true;
    cfg.by_modified = false;
}

/// Age by modification date; clears created.
pub fn set_by_modified(cfg: &mut AutoDeleteConfig) {
    cfg.by_modified = true;
    cfg.by_created = false;
}

/// Clamp days into the dropdown set (default 15).
#[must_use]
pub fn clamp_days(days: u32) -> u32 {
    if AGE_DAY_CHOICES.contains(&days) {
        days
    } else {
        15
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fileorz_core::autodelete::resolve_delete_mode;
    use fileorz_core::autodelete::DeleteMode;

    #[test]
    fn trash_and_permanent_are_mutex() {
        let mut cfg = AutoDeleteConfig {
            enabled: true,
            ..AutoDeleteConfig::default()
        };
        set_to_trash(&mut cfg);
        assert!(cfg.to_trash && !cfg.permanent);
        assert_eq!(resolve_delete_mode(&cfg), Some(DeleteMode::Trash));
        set_permanent(&mut cfg);
        assert!(cfg.permanent && !cfg.to_trash);
        assert_eq!(resolve_delete_mode(&cfg), Some(DeleteMode::Permanent));
    }

    #[test]
    fn created_and_modified_are_mutex() {
        let mut cfg = AutoDeleteConfig::default();
        set_by_created(&mut cfg);
        assert!(cfg.by_created && !cfg.by_modified);
        set_by_modified(&mut cfg);
        assert!(cfg.by_modified && !cfg.by_created);
    }

    #[test]
    fn clamp_unknown_days() {
        assert_eq!(clamp_days(15), 15);
        assert_eq!(clamp_days(7), 15);
    }
}
