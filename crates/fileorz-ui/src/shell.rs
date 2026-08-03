//! Shell application state (config, phase, tray, organizer, settings).

use crate::organizer;
use crate::persist::{self, clamp_interval};
use crate::settings::keywords_logic::KeywordRow;
use crate::settings::{SettingsScreen, SettingsStrings};
use crate::state::RunPhase;
use crate::strings::UiStrings;
use fileorz_core::config::AppConfig;
use fileorz_core::scheduler::OrganizerHandle;
use fileorz_i18n::{normalize_locale, Localization};
use fileorz_linux::autostart;
use fileorz_linux::tray::{TrayLabels, TrayService};

/// Launch mode for the iced shell.
#[derive(Debug, Clone, Copy, Default)]
pub struct LaunchOptions {
    /// Start with [`iced::window::Mode::Hidden`] (tray / `--tray`).
    pub start_hidden: bool,
    /// If config has a valid folder, start organizer immediately.
    pub autostart_organizer: bool,
}

/// Main shell state.
pub struct ShellApp {
    pub strings: UiStrings,
    pub settings_strings: SettingsStrings,
    pub locale: String,
    pub config: AppConfig,
    pub phase: RunPhase,
    pub feedback: Option<String>,
    pub settings_feedback: Option<String>,
    pub settings: SettingsScreen,
    pub keyword_rows: Vec<KeywordRow>,
    pub organizer: Option<OrganizerHandle>,
    pub tray: Option<TrayService>,
    pub start_hidden: bool,
    /// When set, first smoke tick writes a capture and quits.
    pub smoke_path: Option<std::path::PathBuf>,
}

impl ShellApp {
    /// Build shell from locale + launch options.
    #[must_use]
    pub fn new(locale_tag: &str, opts: LaunchOptions) -> Self {
        let locale = normalize_locale(locale_tag);
        let loc = Localization::embed(&locale)
            .unwrap_or_else(|_| Localization::embed("en").expect("en catalog"));
        let strings = UiStrings::from_localization(&loc);
        let settings_strings = SettingsStrings::from_localization(&loc);
        let mut config = persist::load_or_default();
        config.interval_minutes = clamp_interval(config.interval_minutes);
        config.autostart = autostart::is_enabled();
        let tray = spawn_tray(&loc);
        let mut app = Self {
            strings,
            settings_strings,
            locale: loc.locale().to_string(),
            config,
            phase: RunPhase::Idle,
            feedback: None,
            settings_feedback: None,
            settings: SettingsScreen::Main,
            keyword_rows: Vec::new(),
            organizer: None,
            tray,
            start_hidden: opts.start_hidden,
            smoke_path: std::env::var_os("FILEORZ_UI_SMOKE").map(std::path::PathBuf::from),
        };
        if opts.autostart_organizer {
            app.try_autostart_organizer();
        }
        app
    }

    #[must_use]
    pub fn window_title(&self) -> &str {
        let s = &self.settings_strings;
        match self.settings {
            SettingsScreen::Main => &self.strings.window_title,
            SettingsScreen::Hub => &s.hub_window,
            SettingsScreen::Extensions => &s.ext_title,
            SettingsScreen::Advanced => &s.adv_window,
            SettingsScreen::AutoDelete => &s.ad_window,
        }
    }

    #[must_use]
    pub fn locale(&self) -> &str {
        &self.locale
    }

    fn try_autostart_organizer(&mut self) {
        match organizer::try_start(&self.config) {
            organizer::StartOutcome::Started(handle) => {
                self.organizer = Some(handle);
                self.phase = RunPhase::Running;
            }
            organizer::StartOutcome::Rejected(_) => {}
        }
    }
}

fn spawn_tray(loc: &Localization) -> Option<TrayService> {
    if cfg!(test) || std::env::var_os("FILEORZ_UI_NO_TRAY").is_some() {
        return None;
    }
    let labels = TrayLabels::from_messages(
        &loc.message("tray-tooltip"),
        &loc.message("tray-open"),
        &loc.message("tray-quit"),
    );
    match TrayService::spawn(labels) {
        Ok(svc) => Some(svc),
        Err(err) => {
            eprintln!("tray unavailable: {err}");
            None
        }
    }
}
