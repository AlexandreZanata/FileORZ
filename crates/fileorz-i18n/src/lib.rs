//! Fluent locale load and message lookup (ADR-0003).

mod bundle;
mod catalog;
mod locale;

pub use bundle::{I18nError, Localization};
pub use catalog::{embedded_ftl, FILES, LOCALES};
pub use locale::{normalize_locale, resolve_locale, resolve_locale_from_env};

/// Convenience alias: `t!(localization, "message-id")`.
#[macro_export]
macro_rules! t {
    ($loc:expr, $id:expr) => {
        $loc.message($id)
    };
}

/// Crate package name (smoke helper for workspace wiring).
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests;
