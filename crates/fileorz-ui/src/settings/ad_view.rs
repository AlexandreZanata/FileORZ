//! Auto-delete editor view (autosave + mutex radios).

use crate::message::Message;
use crate::settings::chrome;
use crate::settings::msg::SettingsMsg;
use crate::settings::mutex::AGE_DAY_CHOICES;
use crate::shell::ShellApp;
use crate::style::{accent_button, panel_style, secondary_button};
use crate::tokens::{FONT_BODY, FONT_BODY_SM, SPACE_1, SPACE_2, SPACE_3, TEXT, TEXT_MUTED};
use iced::widget::{button, column, container, pick_list, row, text, toggler, Space};
use iced::{Element, Length};

/// Auto-delete flags + days (every change persists).
pub fn view<'a>(app: &'a ShellApp) -> Element<'a, Message> {
    let s = &app.settings_strings;
    let ad = &app.config.auto_delete;
    let enabled = ad.enabled;
    let toggle_label = if enabled { &s.ad_on } else { &s.ad_off };
    column![
        chrome::header(s, &s.ad_title, None),
        column![
            toggler(enabled)
                .label(toggle_label.clone())
                .text_size(FONT_BODY_SM)
                .on_toggle(|v| Message::Settings(SettingsMsg::AdEnabled(v))),
            section(
                &s.ad_filters,
                row![
                    choice(&s.ad_by_created, ad.by_created, SettingsMsg::AdByCreated),
                    choice(&s.ad_by_modified, ad.by_modified, SettingsMsg::AdByModified),
                ]
                .spacing(SPACE_2),
            ),
            section(
                &s.ad_deadline,
                row![
                    pick_list(AGE_DAY_CHOICES, Some(ad.max_age_days), |d| {
                        Message::Settings(SettingsMsg::AdDays(d))
                    },)
                    .text_size(FONT_BODY_SM)
                    .padding([4, 10]),
                    text(&s.ad_deadline_help)
                        .size(FONT_BODY_SM)
                        .color(TEXT_MUTED),
                ]
                .spacing(SPACE_2)
                .align_y(iced::Alignment::Center),
            ),
            section(
                &s.ad_type,
                row![
                    choice(&s.ad_trash, ad.to_trash, SettingsMsg::AdTrash),
                    choice(&s.ad_permanent, ad.permanent, SettingsMsg::AdPermanent),
                ]
                .spacing(SPACE_2),
            ),
        ]
        .spacing(SPACE_2)
        .padding(SPACE_3),
        Space::with_height(Length::Fill),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn section<'a>(title: &'a str, body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(
        column![
            text(title).size(FONT_BODY).color(TEXT),
            Space::with_height(SPACE_1),
            body.into(),
        ]
        .padding(SPACE_2),
    )
    .width(Length::Fill)
    .style(|_| panel_style())
    .into()
}

fn choice<'a>(label: &'a str, selected: bool, msg: SettingsMsg) -> Element<'a, Message> {
    button(text(label).size(FONT_BODY_SM))
        .padding([8, 14])
        .style(move |_, status| {
            if selected {
                accent_button(status)
            } else {
                secondary_button(status)
            }
        })
        .on_press(Message::Settings(msg))
        .into()
}
