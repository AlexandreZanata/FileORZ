//! Shared settings chrome (title + Back).

use crate::message::Message;
use crate::settings::strings::SettingsStrings;
use crate::style::secondary_button;
use crate::tokens::{FONT_BODY, FONT_BODY_SM, FONT_TITLE, SPACE_2, SPACE_3, TEXT, TEXT_MUTED};
use iced::widget::{button, column, row, text, Space};
use iced::{Element, Length};

/// Top bar: Back + title + optional subtitle.
pub fn header<'a>(
    s: &'a SettingsStrings,
    title: &'a str,
    subtitle: Option<&'a str>,
) -> Element<'a, Message> {
    let title_col = match subtitle {
        Some(sub) => column![
            text(title).size(FONT_TITLE).color(TEXT),
            text(sub).size(FONT_BODY_SM).color(TEXT_MUTED),
        ]
        .spacing(2),
        None => column![text(title).size(FONT_TITLE).color(TEXT)],
    };
    row![
        button(text(&s.back).size(FONT_BODY))
            .padding([8, 14])
            .style(|_, _| secondary_button())
            .on_press(Message::SettingsBack),
        Space::with_width(SPACE_2),
        title_col,
        Space::with_width(Length::Fill),
    ]
    .align_y(iced::Alignment::Center)
    .padding([SPACE_2, SPACE_3])
    .into()
}
