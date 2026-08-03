//! Compose main shell or settings subviews.

use crate::about;
use crate::message::Message;
use crate::settings::screen::SettingsScreen;
use crate::settings::{ad_view, adv_view, ext_view, hub};
use crate::shell::ShellApp;
use crate::tokens::BG;
use crate::view_body::view_body;
use crate::view_header::view_header;
use iced::widget::{column, container};
use iced::{Background, Color, Element, Length};

/// Full window content routed by [`SettingsScreen`].
pub fn view(app: &ShellApp) -> Element<'_, Message> {
    let content = match app.settings {
        SettingsScreen::Main => column![view_header(app), view_body(app)].width(Length::Fill),
        SettingsScreen::Hub => column![hub::view(app)].width(Length::Fill),
        SettingsScreen::Extensions => column![ext_view::view(app)].width(Length::Fill),
        SettingsScreen::Advanced => column![adv_view::view(app)].width(Length::Fill),
        SettingsScreen::AutoDelete => column![ad_view::view(app)].width(Length::Fill),
        SettingsScreen::About => column![about::view(app)].width(Length::Fill),
    };
    let mut bg = BG;
    // Subtle enter fade for non-main screens (motion cue #2).
    if app.settings != SettingsScreen::Main {
        bg = Color {
            a: 0.92 + 0.08 * app.motion.screen_t,
            ..BG
        };
    }
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(bg)),
            ..container::Style::default()
        })
        .into()
}
