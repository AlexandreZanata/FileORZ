//! Advanced keywords editor view.

use crate::message::Message;
use crate::settings::chrome;
use crate::settings::msg::SettingsMsg;
use crate::settings::strings::SettingsStrings;
use crate::shell::ShellApp;
use crate::style::{accent_button, danger_button, panel_style, secondary_button};
use crate::tokens::{FONT_BODY_SM, SPACE_1, SPACE_2, SPACE_3, TEXT_MUTED};
use iced::widget::{button, column, container, row, scrollable, text, text_input, toggler, Space};
use iced::{Element, Length};

/// Advanced PDF keyword groups (Save per card; enable autosaves).
pub fn view<'a>(app: &'a ShellApp) -> Element<'a, Message> {
    let s = &app.settings_strings;
    let enabled = app.config.advanced_organize;
    let toggle_label = if enabled { &s.adv_on } else { &s.adv_off };
    let body = if app.keyword_rows.is_empty() {
        column![text(&s.adv_empty).size(FONT_BODY_SM).color(TEXT_MUTED)]
    } else {
        let mut list = column![].spacing(SPACE_2);
        for (i, row) in app.keyword_rows.iter().enumerate() {
            list = list.push(group_card(s, i, &row.name, &row.phrases));
        }
        list
    };
    column![
        chrome::header(s, &s.adv_title, None),
        column![
            toggler(enabled)
                .label(toggle_label.clone())
                .text_size(FONT_BODY_SM)
                .on_toggle(|v| Message::Settings(SettingsMsg::AdvEnabled(v))),
            text(&s.adv_help).size(FONT_BODY_SM).color(TEXT_MUTED),
            button(text(&s.adv_add).size(FONT_BODY_SM))
                .padding([6, 12])
                .style(|_, _| secondary_button())
                .on_press(Message::Settings(SettingsMsg::AdvAddGroup)),
        ]
        .spacing(SPACE_1)
        .padding([0.0, SPACE_3]),
        scrollable(body.padding([SPACE_2, SPACE_3])).height(Length::Fill),
    ]
    .spacing(SPACE_1)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn group_card<'a>(
    s: &'a SettingsStrings,
    index: usize,
    name: &'a str,
    phrases: &'a str,
) -> Element<'a, Message> {
    container(
        column![
            text_input(&s.adv_name_ph, name)
                .on_input(move |v| Message::Settings(SettingsMsg::AdvName(index, v)))
                .padding(8)
                .size(FONT_BODY_SM),
            text_input(&s.adv_kw_ph, phrases)
                .on_input(move |v| Message::Settings(SettingsMsg::AdvPhrases(index, v)))
                .padding(8)
                .size(FONT_BODY_SM),
            row![
                button(text(&s.adv_save).size(FONT_BODY_SM))
                    .padding([6, 12])
                    .style(|_, _| accent_button())
                    .on_press(Message::Settings(SettingsMsg::AdvSaveGroup(index))),
                Space::with_width(SPACE_1),
                button(text(&s.adv_delete).size(FONT_BODY_SM))
                    .padding([6, 12])
                    .style(|_, _| danger_button())
                    .on_press(Message::Settings(SettingsMsg::AdvDeleteGroup(index))),
            ],
        ]
        .spacing(SPACE_1)
        .padding(SPACE_2),
    )
    .width(Length::Fill)
    .style(|_| panel_style())
    .into()
}
