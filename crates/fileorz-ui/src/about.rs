//! About dialog — version, GPL-3.0, upstream + fork notices.

use crate::links::{FORK_URL, GITHUB_URL};
use crate::message::Message;
use crate::settings::chrome;
use crate::shell::ShellApp;
use crate::style::{accent_button, panel_style, secondary_button};
use crate::tokens::{FONT_BODY, FONT_BODY_SM, SPACE_1, SPACE_2, SPACE_3, TEXT, TEXT_MUTED};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length};

/// About content (opened from header).
pub fn view(app: &ShellApp) -> Element<'_, Message> {
    let s = &app.strings;
    let version = format!("{} {}", s.about_version_label, env!("CARGO_PKG_VERSION"));
    let scale = format!("HiDPI scale · {:.2}", app.scale_factor);
    column![
        chrome::header(&app.settings_strings, &s.about, None),
        container(
            column![
                text(version).size(FONT_BODY).color(TEXT),
                text(&s.about_license).size(FONT_BODY_SM).color(TEXT_MUTED),
                Space::with_height(SPACE_2),
                row![
                    button(text(&s.about_upstream).size(FONT_BODY_SM))
                        .padding([8, 14])
                        .style(|_, status| accent_button(status))
                        .on_press(Message::OpenUpstream),
                    Space::with_width(SPACE_1),
                    button(text(&s.about_fork).size(FONT_BODY_SM))
                        .padding([8, 14])
                        .style(|_, status| secondary_button(status))
                        .on_press(Message::OpenFork),
                    Space::with_width(SPACE_1),
                    button(text(&s.about_notices).size(FONT_BODY_SM))
                        .padding([8, 14])
                        .style(|_, status| secondary_button(status))
                        .on_press(Message::OpenNotices),
                ]
                .spacing(SPACE_1),
                Space::with_height(SPACE_2),
                text(scale).size(FONT_BODY_SM).color(TEXT_MUTED),
                text(format!("Upstream · {GITHUB_URL}"))
                    .size(FONT_BODY_SM)
                    .color(TEXT_MUTED),
                text(format!("Fork · {FORK_URL}"))
                    .size(FONT_BODY_SM)
                    .color(TEXT_MUTED),
            ]
            .spacing(SPACE_1)
            .padding(SPACE_3),
        )
        .width(Length::Fill)
        .style(|_| panel_style())
        .padding([0.0, SPACE_3]),
        Space::with_height(Length::Fill),
        row![
            Space::with_width(Length::Fill),
            button(text(&s.about_close).size(FONT_BODY))
                .padding([10, 18])
                .style(|_, status| accent_button(status))
                .on_press(Message::SettingsBack),
        ]
        .padding(SPACE_3),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
