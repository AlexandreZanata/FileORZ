//! Folder, interval, actions, feedback — card layout (Windows-parity).

use crate::message::Message;
use crate::persist::INTERVAL_CHOICES;
use crate::shell::ShellApp;
use crate::state::RunPhase;
use crate::style::{
    accent_button, accent_button_pressed, danger_button, panel_style, secondary_button,
};
use crate::tokens::{
    DANGER, FONT_BODY, FONT_BODY_LG, FONT_BODY_SM, SPACE_1, SPACE_2, SPACE_3, SUCCESS, TEXT,
    TEXT_MUTED,
};
use iced::widget::{button, column, container, pick_list, row, text, Space};
use iced::{Element, Length};

/// Main body under the header.
pub fn view_body(app: &ShellApp) -> Element<'_, Message> {
    column![
        folder_card(app),
        interval_card(app),
        actions_row(app),
        feedback_row(app),
    ]
    .spacing(SPACE_2)
    .padding(SPACE_3)
    .width(Length::Fill)
    .into()
}

fn folder_card(app: &ShellApp) -> Element<'_, Message> {
    let path = app
        .config
        .folder
        .as_deref()
        .unwrap_or(app.strings.folder_empty.as_str());
    container(
        row![
            column![
                text(&app.strings.folder_label)
                    .size(FONT_BODY_LG)
                    .color(TEXT),
                Space::with_height(SPACE_1),
                text(path).size(FONT_BODY_SM).color(TEXT_MUTED),
            ]
            .width(Length::Fill),
            button(text(&app.strings.folder_button).size(FONT_BODY))
                .padding([10, 18])
                .style(|_, status| secondary_button(status))
                .on_press(Message::PickFolder),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(SPACE_2)
        .padding(SPACE_3),
    )
    .width(Length::Fill)
    .style(|_| panel_style())
    .into()
}

fn interval_card(app: &ShellApp) -> Element<'_, Message> {
    container(
        row![
            column![
                text(&app.strings.interval_label)
                    .size(FONT_BODY_LG)
                    .color(TEXT),
                Space::with_height(SPACE_1),
                text(&app.strings.interval_help)
                    .size(FONT_BODY_SM)
                    .color(TEXT_MUTED),
            ]
            .width(Length::Fill),
            pick_list(
                INTERVAL_CHOICES,
                Some(app.config.interval_minutes),
                Message::IntervalChanged,
            )
            .text_size(FONT_BODY)
            .padding([8, 14]),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(SPACE_2)
        .padding(SPACE_3),
    )
    .width(Length::Fill)
    .style(|_| panel_style())
    .into()
}

fn actions_row(app: &ShellApp) -> Element<'_, Message> {
    let primary_label = if app.phase == RunPhase::Running {
        &app.strings.stop
    } else {
        &app.strings.start
    };
    let primary = button(text(primary_label).size(FONT_BODY_LG))
        .padding([14, 28])
        .style(move |_, status| {
            if app.phase == RunPhase::Running {
                danger_button(status)
            } else if matches!(status, iced::widget::button::Status::Pressed) {
                accent_button_pressed()
            } else {
                accent_button(status)
            }
        })
        .on_press(Message::ToggleOrganizer);

    row![
        button(text(&app.strings.settings).size(FONT_BODY))
            .padding([14, 20])
            .style(|_, status| secondary_button(status))
            .on_press(Message::OpenSettings),
        Space::with_width(Length::Fill),
        primary,
    ]
    .align_y(iced::Alignment::Center)
    .padding(iced::Padding {
        top: SPACE_1,
        ..iced::Padding::ZERO
    })
    .into()
}

fn feedback_row(app: &ShellApp) -> Element<'_, Message> {
    let Some(msg) = app.feedback.as_deref() else {
        return Space::with_height(SPACE_1).into();
    };
    let mut color = match app.phase {
        RunPhase::Error => DANGER,
        RunPhase::Running => SUCCESS,
        RunPhase::Idle => TEXT_MUTED,
    };
    color.a = app.motion.feedback_t;
    text(msg).size(FONT_BODY_SM).color(color).into()
}
