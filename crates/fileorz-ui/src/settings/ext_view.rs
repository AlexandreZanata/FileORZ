//! Extensions editor view.

use crate::message::Message;
use crate::settings::chrome;
use crate::settings::ext_logic::{category_ids, ext_keys};
use crate::settings::msg::SettingsMsg;
use crate::shell::ShellApp;
use crate::style::{accent_button, panel_style, secondary_button};
use crate::tokens::{FONT_BODY, FONT_BODY_SM, SPACE_1, SPACE_2, SPACE_3, TEXT, TEXT_MUTED};
use iced::widget::{button, checkbox, column, container, row, scrollable, text, Space};
use iced::{Element, Length};

/// Categories & extensions (Apply = Save).
pub fn view<'a>(app: &'a ShellApp) -> Element<'a, Message> {
    let s = &app.settings_strings;
    let mut cats = column![].spacing(SPACE_2);
    for cat in category_ids(&app.config) {
        cats = cats.push(category_block(app, &cat));
    }
    column![
        chrome::header(s, &s.ext_title, Some(&s.ext_subtitle)),
        scrollable(cats.padding([0.0, SPACE_3])).height(Length::Fill),
        row![
            Space::with_width(Length::Fill),
            button(text(&s.ext_save).size(FONT_BODY))
                .padding([10, 18])
                .style(|_, _| accent_button())
                .on_press(Message::Settings(SettingsMsg::ExtSave)),
        ]
        .padding(SPACE_3),
        feedback(app),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn category_block<'a>(app: &'a ShellApp, cat: &str) -> Element<'a, Message> {
    let s = &app.settings_strings;
    let label = s.category_label(cat);
    let Some(map) = app.config.categories.get(cat) else {
        return Space::with_height(0).into();
    };
    let cat_owned = cat.to_string();
    let mut checks = row![].spacing(SPACE_2);
    for ext in ext_keys(map) {
        let on = *map.get(&ext).unwrap_or(&false);
        let cat2 = cat_owned.clone();
        let ext2 = ext.clone();
        checks = checks.push(
            checkbox(ext, on)
                .text_size(FONT_BODY_SM)
                .on_toggle(move |enabled| {
                    Message::Settings(SettingsMsg::ExtToggle {
                        category: cat2.clone(),
                        ext: ext2.clone(),
                        enabled,
                    })
                }),
        );
    }
    container(
        column![
            row![
                text(label).size(FONT_BODY).color(TEXT),
                Space::with_width(Length::Fill),
                button(text(&s.ext_all).size(FONT_BODY_SM))
                    .padding([4, 10])
                    .style(|_, _| secondary_button())
                    .on_press(Message::Settings(SettingsMsg::ExtSetAll {
                        category: cat_owned.clone(),
                        enabled: true,
                    })),
                button(text(&s.ext_none).size(FONT_BODY_SM))
                    .padding([4, 10])
                    .style(|_, _| secondary_button())
                    .on_press(Message::Settings(SettingsMsg::ExtSetAll {
                        category: cat_owned,
                        enabled: false,
                    })),
            ]
            .align_y(iced::Alignment::Center)
            .spacing(SPACE_1),
            checks,
        ]
        .spacing(SPACE_1)
        .padding(SPACE_2),
    )
    .width(Length::Fill)
    .style(|_| panel_style())
    .into()
}

fn feedback(app: &ShellApp) -> Element<'_, Message> {
    match app.settings_feedback.as_deref() {
        Some(msg) => text(msg).size(FONT_BODY_SM).color(TEXT_MUTED).into(),
        None => Space::with_height(0).into(),
    }
}
