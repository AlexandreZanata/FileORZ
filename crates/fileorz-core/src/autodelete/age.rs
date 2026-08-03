//! Age rules — mtime / birthtime (ADR-0005).

use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Whole days between `earlier` and `now` (floor), matching Python `.days`.
#[must_use]
pub fn age_days(now: SystemTime, earlier: SystemTime) -> u64 {
    now.duration_since(earlier)
        .map(|d| d.as_secs() / 86_400)
        .unwrap_or(0)
}

/// True when age is **strictly greater** than `max_age_days`.
#[must_use]
pub fn exceeds_max_age(now: SystemTime, stamp: SystemTime, max_age_days: u32) -> bool {
    age_days(now, stamp) > u64::from(max_age_days)
}

/// File mtime (modification time).
pub fn read_mtime(path: &Path) -> std::io::Result<SystemTime> {
    fs::metadata(path)?.modified()
}

/// Birth/creation time when the platform exposes it; `None` if unavailable.
/// Never falls back to ctime (ADR-0005).
pub fn read_birthtime(path: &Path) -> std::io::Result<Option<SystemTime>> {
    match fs::metadata(path)?.created() {
        Ok(t) => Ok(Some(t)),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod age_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn exceeds_uses_strict_greater_than() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(100 * 86_400);
        let stamp = now - Duration::from_secs(15 * 86_400);
        assert!(!exceeds_max_age(now, stamp, 15));
        let older = now - Duration::from_secs(16 * 86_400);
        assert!(exceeds_max_age(now, older, 15));
    }
}
