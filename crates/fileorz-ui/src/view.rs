//! Compose header + body on the dark canvas.

use crate::message::Message;
use crate::shell::ShellApp;
use crate::tokens::BG;
use crate::view_body::view_body;
use crate::view_header::view_header;
use iced::widget::{column, container};
use iced::{Background, Element, Length};

/// Full window content.
pub fn view(app: &ShellApp) -> Element<'_, Message> {
    container(column![view_header(app), view_body(app)].width(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(BG)),
            ..container::Style::default()
        })
        .into()
}
