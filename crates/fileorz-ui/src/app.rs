//! iced application entry — subscriptions + window lifecycle.

use crate::brand_icon;
use crate::message::Message;
use crate::settings::SettingsScreen;
use crate::shell::{LaunchOptions, ShellApp};
use crate::theme::fileorz_theme;
use crate::tokens::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::update::update;
use crate::view::view;
use iced::window::{self, Mode};
use iced::{Size, Subscription, Task};
use std::time::Duration;

/// Run visible main shell (default / `--ui`).
///
/// # Errors
/// Returns iced graphics / window errors.
pub fn run(locale_tag: &str) -> iced::Result {
    run_with(locale_tag, LaunchOptions::default())
}

/// Run shell hidden (for `--tray` parity: organizer may autostart).
///
/// # Errors
/// Returns iced graphics / window errors.
pub fn run_tray(locale_tag: &str) -> iced::Result {
    run_with(
        locale_tag,
        LaunchOptions {
            start_hidden: true,
            autostart_organizer: true,
        },
    )
}

/// Shared iced bootstrap.
///
/// # Errors
/// Returns iced graphics / window errors.
pub fn run_with(locale_tag: &str, opts: LaunchOptions) -> iced::Result {
    let locale = locale_tag.to_string();
    let position = if std::env::var_os("FILEORZ_UI_POS").is_some() {
        iced::window::Position::Specific(iced::Point::new(64.0, 64.0))
    } else {
        iced::window::Position::Centered
    };
    let start_hidden = opts.start_hidden;
    iced::application(title, update, view)
        .theme(|_| fileorz_theme())
        .subscription(subscription)
        .exit_on_close_request(false)
        .window(iced::window::Settings {
            size: Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            position,
            visible: !start_hidden,
            exit_on_close_request: false,
            icon: brand_icon::window_icon(),
            ..iced::window::Settings::default()
        })
        .run_with(move || {
            let mut app = ShellApp::new(&locale, opts);
            if std::env::var_os("FILEORZ_UI_OPEN_SETTINGS").is_some() {
                app.settings = SettingsScreen::Hub;
            }
            if std::env::var_os("FILEORZ_UI_OPEN_ABOUT").is_some() {
                app.settings = SettingsScreen::About;
            }
            (app, boot_tasks(start_hidden))
        })
}

fn boot_tasks(start_hidden: bool) -> Task<Message> {
    let scale = window::get_oldest().then(|id| match id {
        Some(id) => window::get_scale_factor(id).map(Message::ScaleFactor),
        None => Task::none(),
    });
    let mode = if start_hidden {
        window::get_oldest().then(|id| match id {
            Some(id) => window::change_mode(id, Mode::Hidden),
            None => Task::none(),
        })
    } else if std::env::var_os("FILEORZ_UI_OPEN_SETTINGS").is_some() {
        window::get_oldest().then(|id| match id {
            Some(id) => window::resize(
                id,
                Size::new(
                    crate::settings::screen::SETTINGS_WIDTH,
                    crate::settings::screen::SETTINGS_HEIGHT,
                ),
            ),
            None => Task::none(),
        })
    } else {
        Task::none()
    };
    Task::batch([scale, mode])
}

fn title(app: &ShellApp) -> String {
    app.window_title().to_string()
}

fn subscription(app: &ShellApp) -> Subscription<Message> {
    let close = window::close_requests().map(Message::CloseRequested);
    let tray = if app.tray.is_some() {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::TrayPoll)
    } else {
        Subscription::none()
    };
    let smoke = if app.smoke_path.is_some() {
        iced::time::every(Duration::from_millis(700)).map(|_| Message::SmokeTick)
    } else {
        Subscription::none()
    };
    let motion = if app.motion.needs_tick() {
        iced::time::every(Duration::from_millis(32)).map(|_| Message::MotionTick)
    } else {
        Subscription::none()
    };
    let keys = iced::keyboard::on_key_press(|key, _mods| match key {
        iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
            Some(Message::SettingsBack)
        }
        iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => Some(Message::EnterKey),
        _ => None,
    });
    Subscription::batch([close, tray, smoke, motion, keys])
}

/// Expose window size tokens for tests / docs.
#[must_use]
pub fn window_size() -> (f32, f32) {
    (WINDOW_WIDTH, WINDOW_HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::RunPhase;

    #[test]
    fn locale_changes_window_title() {
        let en = ShellApp::new("en", LaunchOptions::default());
        let pt = ShellApp::new("pt-BR", LaunchOptions::default());
        assert_eq!(en.locale(), "en");
        assert_eq!(pt.locale(), "pt-BR");
        assert_ne!(en.window_title(), pt.window_title());
    }

    #[test]
    fn apply_locale_persists_tag() {
        let mut app = ShellApp::new("en", LaunchOptions::default());
        app.apply_locale("pt-BR");
        assert_eq!(app.locale(), "pt-BR");
        assert_eq!(app.config.locale, "pt-BR");
        assert!(app.strings.settings.contains('ç') || app.strings.settings == "Configurações");
    }

    #[test]
    fn en_title_matches_catalog() {
        let app = ShellApp::new("en", LaunchOptions::default());
        assert_eq!(app.window_title(), "FileORZ — Organize your files");
        assert_eq!(app.phase, RunPhase::Idle);
    }

    #[test]
    fn window_size_parity() {
        assert_eq!(window_size(), (720.0, 460.0));
    }
}
