//! Shell view styles shared by header / body.

use crate::tokens::{ACCENT, ACCENT_STRONG, BORDER, DANGER, SURFACE, SURFACE_RAISED, TEXT};
use iced::widget::{button, container};
use iced::{Background, Border, Color};

/// Filled primary CTA (Start).
pub fn accent_button() -> button::Style {
    button::Style {
        background: Some(Background::Color(ACCENT)),
        text_color: Color::WHITE,
        border: Border {
            radius: 6.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

/// Pressed primary CTA.
pub fn accent_button_pressed() -> button::Style {
    button::Style {
        background: Some(Background::Color(ACCENT_STRONG)),
        text_color: Color::WHITE,
        border: Border {
            radius: 6.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

/// Stop / danger CTA.
pub fn danger_button() -> button::Style {
    button::Style {
        background: Some(Background::Color(DANGER)),
        text_color: Color::WHITE,
        border: Border {
            radius: 6.0.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

/// Secondary header / settings button.
pub fn secondary_button() -> button::Style {
    button::Style {
        background: Some(Background::Color(SURFACE_RAISED)),
        text_color: TEXT,
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..button::Style::default()
    }
}

/// Panel behind folder / interval rows.
pub fn panel_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..container::Style::default()
    }
}

/// Header strip.
pub fn header_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(SURFACE)),
        ..container::Style::default()
    }
}
