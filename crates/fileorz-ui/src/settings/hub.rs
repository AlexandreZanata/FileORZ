//! Settings hub — navigation cards + language picker.

use crate::locale_pick::LOCALE_CHOICES;
use crate::message::Message;
use crate::settings::chrome;
use crate::settings::msg::SettingsMsg;
use crate::shell::ShellApp;
use crate::style::{accent_button, panel_style};
use crate::tokens::{FONT_BODY, FONT_BODY_SM, SPACE_2, SPACE_3, TEXT, TEXT_MUTED};
use iced::widget::{button, column, container, pick_list, row, text, Space};
use iced::{Element, Length};

/// Hub cards + language row (Windows-parity settings entry).
pub fn view(app: &ShellApp) -> Element<'_, Message> {
    let s = &app.settings_strings;
    column![
        chrome::header(s, &s.hub_title, Some(&s.hub_subtitle)),
        row![
            nav_card(
                &s.card_ext_title,
                &s.card_ext_body,
                &s.configure,
                SettingsMsg::OpenExtensions
            ),
            nav_card(
                &s.card_adv_title,
                &s.card_adv_body,
                &s.configure,
                SettingsMsg::OpenAdvanced
            ),
            nav_card(
                &s.card_ad_title,
                &s.card_ad_body,
                &s.configure,
                SettingsMsg::OpenAutoDelete
            ),
        ]
        .spacing(SPACE_2)
        .padding(hub_pad()),
        language_card(app),
    ]
    .width(Length::Fill)
    .into()
}

fn hub_pad() -> iced::Padding {
    iced::Padding {
        top: 0.0,
        right: SPACE_3,
        bottom: SPACE_2,
        left: SPACE_3,
    }
}

fn nav_card<'a>(
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
                .style(|_, status| accent_button(status))
                .on_press(Message::Settings(msg)),
        ]
        .spacing(SPACE_2 / 2.0)
        .padding(SPACE_2),
    )
    .width(Length::Fill)
    .style(|_| panel_style())
    .into()
}

fn language_card(app: &ShellApp) -> Element<'_, Message> {
    let s = &app.settings_strings;
    container(
        container(
            row![
                column![
                    text(&s.card_lang_title).size(FONT_BODY).color(TEXT),
                    text(&s.card_lang_body).size(FONT_BODY_SM).color(TEXT_MUTED),
                ]
                .spacing(SPACE_2 / 2.0)
                .width(Length::Fill),
                pick_list(LOCALE_CHOICES, Some(app.locale.as_str()), |tag: &str| {
                    Message::LocaleChanged(tag.to_string())
                },)
                .text_size(FONT_BODY)
                .padding([8, 14]),
            ]
            .align_y(iced::Alignment::Center)
            .spacing(SPACE_2)
            .padding(SPACE_2),
        )
        .width(Length::Fill)
        .style(|_| panel_style()),
    )
    .padding(iced::Padding {
        top: 0.0,
        right: SPACE_3,
        bottom: SPACE_3,
        left: SPACE_3,
    })
    .into()
}
