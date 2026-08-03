//! Settings hub — three navigation cards.

use crate::message::Message;
use crate::settings::chrome;
use crate::settings::msg::SettingsMsg;
use crate::settings::strings::SettingsStrings;
use crate::style::{accent_button, panel_style, secondary_button};
use crate::tokens::{FONT_BODY, FONT_BODY_SM, SPACE_2, SPACE_3, TEXT, TEXT_MUTED};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length};

/// Three-card hub matching upstream `ui/config.py`.
pub fn view(s: &SettingsStrings) -> Element<'_, Message> {
    column![
        chrome::header(s, &s.hub_title, Some(&s.hub_subtitle)),
        row![
            card(
                &s.card_ext_title,
                &s.card_ext_body,
                &s.configure,
                SettingsMsg::OpenExtensions
            ),
            card(
                &s.card_adv_title,
                &s.card_adv_body,
                &s.configure,
                SettingsMsg::OpenAdvanced
            ),
            card(
                &s.card_ad_title,
                &s.card_ad_body,
                &s.configure,
                SettingsMsg::OpenAutoDelete
            ),
        ]
        .spacing(SPACE_2)
        .padding(iced::Padding {
            top: 0.0,
            right: SPACE_3,
            bottom: SPACE_3,
            left: SPACE_3,
        }),
    ]
    .width(Length::Fill)
    .into()
}

fn card<'a>(
    title: &'a str,
    body: &'a str,
    configure: &'a str,
    msg: SettingsMsg,
) -> Element<'a, Message> {
    container(
        column![
            text(title).size(FONT_BODY).color(TEXT),
            text(body).size(FONT_BODY_SM).color(TEXT_MUTED),
            Space::with_height(SPACE_2),
            button(text(configure).size(FONT_BODY_SM))
                .padding([8, 14])
                .style(|_, status| {
                    if matches!(status, iced::widget::button::Status::Pressed) {
                        secondary_button()
                    } else {
                        accent_button()
                    }
                })
                .on_press(Message::Settings(msg)),
        ]
        .spacing(SPACE_2 / 2.0)
        .padding(SPACE_2),
    )
    .width(Length::Fill)
    .style(|_| panel_style())
    .into()
}
