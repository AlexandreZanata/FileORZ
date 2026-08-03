//! Localized shell copy loaded once from Fluent.

use fileorz_i18n::Localization;

/// Bundle of main-shell strings (all via fileorz-i18n).
#[derive(Debug, Clone)]
pub struct UiStrings {
    pub window_title: String,
    pub brand: String,
    pub tagline: String,
    pub autostart: String,
    pub changelog: String,
    pub github: String,
    pub about: String,
    pub folder_label: String,
    pub folder_button: String,
    pub folder_dialog: String,
    pub folder_empty: String,
    pub interval_label: String,
    pub interval_help: String,
    pub settings: String,
    pub start: String,
    pub stop: String,
    pub err_folder_missing: String,
    pub err_folder_invalid: String,
    pub err_start_failed: String,
    pub feedback_started: String,
    pub about_body: String,
}

impl UiStrings {
    /// Resolve every shell ID from an embedded catalog.
    #[must_use]
    pub fn from_localization(loc: &Localization) -> Self {
        Self {
            window_title: loc.message("app-window-title"),
            brand: loc.message("app-title"),
            tagline: loc.message("app-tagline"),
            autostart: loc.message("header-autostart"),
            changelog: loc.message("header-changelog"),
            github: loc.message("header-github"),
            about: loc.message("about-title"),
            folder_label: loc.message("folder-pick-label"),
            folder_button: loc.message("folder-pick-button"),
            folder_dialog: loc.message("folder-pick-dialog"),
            folder_empty: loc.message("folder-pick-empty"),
            interval_label: loc.message("interval-label"),
            interval_help: loc.message("interval-help"),
            settings: loc.message("main-btn-settings"),
            start: loc.message("main-btn-start"),
            stop: loc.message("main-btn-stop"),
            err_folder_missing: loc.message("error-folder-missing"),
            err_folder_invalid: loc.message("error-folder-invalid"),
            err_start_failed: loc.message("error-organizer-start-failed"),
            feedback_started: loc.message("feedback-organize-started"),
            about_body: format!(
                "{} — {} · {}",
                loc.message("about-title"),
                loc.message("about-license"),
                loc.message("about-upstream")
            ),
        }
    }
}
