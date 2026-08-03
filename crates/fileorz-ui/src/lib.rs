//! iced UI — theme tokens + main shell (ADR-0002, phase 13).

pub mod app;
pub mod theme;
pub mod tokens;

pub use app::{run, window_size, ShellApp};
pub use theme::{fileorz_theme, palette, THEME_NAME};
pub use tokens::{ACCENT, BG, DANGER, SURFACE, TEXT, TEXT_MUTED, WINDOW_HEIGHT, WINDOW_WIDTH};

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
        assert_eq!(crate_name(), "fileorz-ui");
    }
}
