//! Header: brand icon, autostart, GitHub / Changelog (Windows-parity layout).

use crate::message::Message;
use crate::shell::ShellApp;
use crate::style::{header_style, secondary_button};
use crate::tokens::{FONT_BODY_SM, FONT_TITLE, SPACE_1, SPACE_2, SPACE_3, TEXT, TEXT_MUTED};
use iced::widget::{button, column, container, image, row, text, toggler, Space};
use iced::{Element, Length};

/// Top strip matching upstream header jobs (no language picker — Settings hub).
pub fn view_header(app: &ShellApp) -> Element<'_, Message> {
    let brand = row![
        image(app.brand_icon.clone())
            .width(Length::Fixed(32.0))
            .height(Length::Fixed(32.0)),
        Space::with_width(SPACE_2),
        column![
            text(&app.strings.brand).size(FONT_TITLE).color(TEXT),
            text(&app.strings.tagline)
                .size(FONT_BODY_SM)
                .color(TEXT_MUTED),
        ]
        .spacing(2),
    ]
    .align_y(iced::Alignment::Center);

    let controls = row![
        toggler(app.config.autostart)
            .label(app.strings.autostart.clone())
            .text_size(FONT_BODY_SM)
            .on_toggle(Message::AutostartToggled),
        Space::with_width(SPACE_2),
        link_btn(&app.strings.github, Message::OpenGithub),
        Space::with_width(SPACE_1),
        link_btn(&app.strings.changelog, Message::OpenChangelog),
        Space::with_width(SPACE_1),
        link_btn(&app.strings.about, Message::ShowAbout),
    ]
    .align_y(iced::Alignment::Center);

    container(
        row![brand, Space::with_width(Length::Fill), controls]
            .align_y(iced::Alignment::Center)
            .padding([SPACE_2, SPACE_3]),
    )
    .width(Length::Fill)
    .style(|_| header_style())
    .into()
}

fn link_btn<'a>(label: &'a str, msg: Message) -> Element<'a, Message> {
    button(text(label).size(FONT_BODY_SM))
        .padding([8, 14])
        .style(|_, status| secondary_button(status))
        .on_press(msg)
        .into()
}
