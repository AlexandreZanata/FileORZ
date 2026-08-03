//! Locale load and message lookup (scaffold).
//!
//! Fluent catalogs currently live in repo-root `i18n/`; move here in phase 10.

/// Crate package name (smoke helper for workspace wiring).
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn crate_name_matches() {
        assert_eq!(crate_name(), "fileorz-i18n");
    }
}
