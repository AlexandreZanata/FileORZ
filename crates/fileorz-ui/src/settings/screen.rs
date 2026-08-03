//! Settings navigation screens (in-app subviews).

/// Which settings surface is visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsScreen {
    /// Main shell (folder / interval / start).
    #[default]
    Main,
    /// Three-card hub.
    Hub,
    /// Categories & extensions editor.
    Extensions,
    /// Advanced PDF keywords editor.
    Advanced,
    /// Auto-delete editor.
    AutoDelete,
    /// About / license dialog.
    About,
}

impl SettingsScreen {
    /// Pop one level toward Main (Esc / Back).
    #[must_use]
    pub fn back(self) -> Self {
        match self {
            Self::Main => Self::Main,
            Self::Hub | Self::About => Self::Main,
            Self::Extensions | Self::Advanced | Self::AutoDelete => Self::Hub,
        }
    }

    /// True when Esc should leave settings rather than no-op on main.
    #[must_use]
    pub fn is_settings(self) -> bool {
        !matches!(self, Self::Main)
    }
}

/// Settings window size (parity with CustomTkinter hub).
pub const SETTINGS_WIDTH: f32 = 900.0;
pub const SETTINGS_HEIGHT: f32 = 520.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_from_editor_to_hub_to_main() {
        assert_eq!(SettingsScreen::Extensions.back(), SettingsScreen::Hub);
        assert_eq!(SettingsScreen::Hub.back(), SettingsScreen::Main);
        assert_eq!(SettingsScreen::About.back(), SettingsScreen::Main);
        assert_eq!(SettingsScreen::Main.back(), SettingsScreen::Main);
    }
}
