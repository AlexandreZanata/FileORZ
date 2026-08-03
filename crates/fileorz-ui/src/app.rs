//! Blank main window skeleton — i18n title, token-styled shell.

use crate::theme::fileorz_theme;
use crate::tokens::{
    self, ACCENT, BORDER, FONT_BODY, FONT_TITLE, SPACE_2, SPACE_3, SURFACE, TEXT, TEXT_MUTED,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};
use fileorz_i18n::{normalize_locale, Localization};
use iced::widget::{column, container, horizontal_rule, text, Space};
use iced::{Background, Border, Color, Element, Length, Size, Task};

#[derive(Debug, Clone)]
pub enum Message {}

/// Minimal shell state (phase 13 — no settings yet).
pub struct ShellApp {
    window_title: String,
    brand: String,
    tagline: String,
    locale: String,
}

impl ShellApp {
    /// Build shell from locale tag (`en`, `pt-BR`, …).
    #[must_use]
    pub fn new(locale_tag: &str) -> Self {
        let locale = normalize_locale(locale_tag);
        let loc = Localization::embed(&locale)
            .unwrap_or_else(|_| Localization::embed("en").expect("en catalog must embed"));
        Self {
            window_title: loc.message("app-window-title"),
            brand: loc.message("app-title"),
            tagline: loc.message("app-tagline"),
            locale: loc.locale().to_string(),
        }
    }

    #[must_use]
    pub fn window_title(&self) -> &str {
        &self.window_title
    }

    #[must_use]
    pub fn locale(&self) -> &str {
        &self.locale
    }
}

fn title(app: &ShellApp) -> String {
    app.window_title.clone()
}

fn update(_app: &mut ShellApp, _message: Message) -> Task<Message> {
    Task::none()
}

fn view(app: &ShellApp) -> Element<'_, Message> {
    let header = column![
        text(&app.brand).size(FONT_TITLE).color(TEXT),
        text(&app.tagline).size(FONT_BODY).color(TEXT_MUTED),
    ]
    .spacing(SPACE_2 / 2.0);

    let body = column![
        header,
        horizontal_rule(1).style(|_| iced::widget::rule::Style {
            color: BORDER,
            width: 1,
            radius: 0.0.into(),
            fill_mode: iced::widget::rule::FillMode::Full,
        }),
        Space::with_height(SPACE_3),
        text(format!("locale · {}", app.locale))
            .size(FONT_BODY)
            .color(TEXT_MUTED),
        Space::with_height(SPACE_2),
        text("Main shell — settings & organizer in later phases")
            .size(FONT_BODY)
            .color(TEXT),
        Space::with_height(SPACE_3),
        container(text("Start organizing").size(FONT_BODY).color(Color::WHITE))
            .padding([SPACE_2 / 2.0, SPACE_2])
            .style(|_| iced::widget::container::Style {
                background: Some(Background::Color(ACCENT)),
                border: Border {
                    radius: 4.0.into(),
                    ..Border::default()
                },
                ..Default::default()
            }),
    ]
    .spacing(SPACE_2)
    .padding(SPACE_3)
    .width(Length::Fill);

    container(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style {
            background: Some(Background::Color(SURFACE)),
            ..Default::default()
        })
        .into()
}

/// Run the iced shell on Wayland/X11.
///
/// # Errors
/// Returns iced graphics / window errors.
pub fn run(locale_tag: &str) -> iced::Result {
    let locale = locale_tag.to_string();
    let position = if std::env::var_os("FILEORZ_UI_POS").is_some() {
        iced::window::Position::Specific(iced::Point::new(64.0, 64.0))
    } else {
        iced::window::Position::Centered
    };
    iced::application(title, update, view)
        .theme(|_| fileorz_theme())
        .window(iced::window::Settings {
            size: Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            position,
            ..iced::window::Settings::default()
        })
        .run_with(move || (ShellApp::new(&locale), Task::none()))
}

/// Expose window size tokens for tests / docs.
#[must_use]
pub fn window_size() -> (f32, f32) {
    (tokens::WINDOW_WIDTH, tokens::WINDOW_HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_changes_window_title() {
        let en = ShellApp::new("en");
        let pt = ShellApp::new("pt-BR");
        assert_eq!(en.locale(), "en");
        assert_eq!(pt.locale(), "pt-BR");
        assert_ne!(en.window_title(), pt.window_title());
        assert!(en.window_title().contains("Organize"));
        assert!(pt.window_title().contains("arquivos") || pt.window_title().contains("Organize"));
    }

    #[test]
    fn en_title_matches_catalog() {
        let app = ShellApp::new("en");
        assert_eq!(app.window_title(), "FileORZ — Organize your files");
    }
}
