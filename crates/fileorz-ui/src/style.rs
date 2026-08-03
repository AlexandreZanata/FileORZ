//! Shell view styles shared by header / body (focus-visible on hover/press).

use crate::tokens::{ACCENT, ACCENT_STRONG, BORDER, DANGER, SURFACE, SURFACE_RAISED, TEXT};
use iced::widget::button;
use iced::{Background, Border, Color};

/// Accent border used as the visible focus / hover ring.
pub fn focus_border() -> Border {
    Border {
        color: ACCENT,
        width: 2.0,
        radius: 6.0.into(),
    }
}

fn with_focus(mut style: button::Style, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered | button::Status::Pressed => {
            style.border = focus_border();
        }
        _ => {}
    }
    style
}

/// Filled primary CTA (Start).
pub fn accent_button(status: button::Status) -> button::Style {
    with_focus(
        button::Style {
            background: Some(Background::Color(ACCENT)),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        },
        status,
    )
}

/// Pressed primary CTA (kept for callers that branch Pressed).
pub fn accent_button_pressed() -> button::Style {
    button::Style {
        background: Some(Background::Color(ACCENT_STRONG)),
        text_color: Color::WHITE,
        border: focus_border(),
        ..button::Style::default()
    }
}

/// Stop / danger CTA.
pub fn danger_button(status: button::Status) -> button::Style {
    with_focus(
        button::Style {
            background: Some(Background::Color(DANGER)),
            text_color: Color::WHITE,
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            ..button::Style::default()
        },
        status,
    )
}

/// Secondary header / settings button.
pub fn secondary_button(status: button::Status) -> button::Style {
    with_focus(
        button::Style {
            background: Some(Background::Color(SURFACE_RAISED)),
            text_color: TEXT,
            border: Border {
                color: BORDER,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..button::Style::default()
        },
        status,
    )
}

/// Panel behind folder / interval rows.
pub fn panel_style() -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..iced::widget::container::Style::default()
    }
}

/// Header strip.
pub fn header_style() -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(Background::Color(SURFACE)),
        ..iced::widget::container::Style::default()
    }
}
