//! iced UI — theme tokens + main shell + settings + a11y polish (phases 13–16).

pub mod about;
pub mod app;
pub mod brand_icon;
pub mod contrast;
pub mod links;
pub mod locale_pick;
pub mod message;
pub mod motion;
pub mod organizer;
pub mod persist;
pub mod settings;
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
pub mod window_ops;

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
