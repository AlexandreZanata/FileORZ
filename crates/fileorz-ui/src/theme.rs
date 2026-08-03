//! iced [`Theme`] built from FileORZ tokens.

use crate::tokens::{ACCENT, BG, DANGER, SUCCESS, TEXT};
use iced::theme::{Palette, Theme};

/// Custom dark theme name (for `Theme::custom`).
pub const THEME_NAME: &str = "FileORZ Dark";

/// iced palette mapped from design tokens.
#[must_use]
pub fn palette() -> Palette {
    Palette {
        background: BG,
        text: TEXT,
        primary: ACCENT,
        success: SUCCESS,
        danger: DANGER,
    }
}

/// Full iced theme for the shell.
#[must_use]
pub fn fileorz_theme() -> Theme {
    Theme::custom(THEME_NAME.to_string(), palette())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_name_stable() {
        assert_eq!(THEME_NAME, "FileORZ Dark");
        let _ = fileorz_theme();
    }
}
