//! iced UI — theme tokens + main shell (ADR-0002, phases 13–14).

pub mod app;
pub mod links;
pub mod message;
pub mod organizer;
pub mod persist;
pub mod shell;
pub mod smoke;
pub mod state;
pub mod strings;
pub mod style;
pub mod theme;
pub mod tokens;
pub mod update;
pub mod view;
pub mod view_body;
pub mod view_header;

pub use app::{run, run_tray, run_with, window_size};
pub use shell::{LaunchOptions, ShellApp};
pub use state::RunPhase;
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
