//! Design tokens — dark-first Linux shell (no purple / cream-serif clichés).

use iced::Color;

/// Cool charcoal canvas (not purple-tinted).
pub const BG: Color = Color::from_rgb(0.102, 0.114, 0.137); // #1A1D23

/// Elevated panel / header strip.
pub const SURFACE: Color = Color::from_rgb(0.141, 0.157, 0.188); // #242830

/// Slightly lighter surface for hover/focus rings.
pub const SURFACE_RAISED: Color = Color::from_rgb(0.180, 0.200, 0.235); // #2E3340

/// Teal accent for primary CTA (Start) — avoids indigo/violet defaults.
pub const ACCENT: Color = Color::from_rgb(0.239, 0.608, 0.561); // #3D9B8F

/// Accent pressed / strong.
pub const ACCENT_STRONG: Color = Color::from_rgb(0.180, 0.510, 0.470); // #2E8278

/// Danger / destructive actions.
pub const DANGER: Color = Color::from_rgb(0.910, 0.365, 0.298); // #E85D4C

/// Primary text on dark surfaces.
pub const TEXT: Color = Color::from_rgb(0.910, 0.918, 0.929); // #E8EAED

/// Secondary / help text.
pub const TEXT_MUTED: Color = Color::from_rgb(0.604, 0.627, 0.651); // #9AA0A6

/// Hairline borders.
pub const BORDER: Color = Color::from_rgb(0.227, 0.247, 0.294); // #3A3F4B

/// Success feedback (organized OK).
pub const SUCCESS: Color = Color::from_rgb(0.290, 0.690, 0.478); // #4AB07A

/// 8px spacing grid.
pub const SPACE_1: f32 = 8.0;
pub const SPACE_2: f32 = 16.0;
pub const SPACE_3: f32 = 24.0;
pub const SPACE_4: f32 = 32.0;

/// Type scale (UI-LINUX.md).
pub const FONT_BODY: f32 = 14.0;
pub const FONT_BODY_SM: f32 = 12.0;
pub const FONT_BODY_LG: f32 = 16.0;
pub const FONT_TITLE: f32 = 22.0;

/// Parity starting window size (CustomTkinter / Windows shell).
pub const WINDOW_WIDTH: f32 = 720.0;
pub const WINDOW_HEIGHT: f32 = 460.0;

/// Hex helpers for docs / tests.
#[must_use]
pub fn bg_hex() -> &'static str {
    "#1A1D23"
}

#[must_use]
pub fn accent_hex() -> &'static str {
    "#3D9B8F"
}

#[must_use]
pub fn danger_hex() -> &'static str {
    "#E85D4C"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accent_hex_is_teal_not_purple() {
        assert_eq!(accent_hex(), "#3D9B8F");
        assert_eq!(bg_hex(), "#1A1D23");
        assert_eq!(danger_hex(), "#E85D4C");
    }

    #[test]
    fn bg_is_dark() {
        let luma = 0.2126 * BG.r + 0.7152 * BG.g + 0.0722 * BG.b;
        assert!(luma < 0.25_f32);
    }

    #[test]
    fn window_parity_size() {
        assert!((WINDOW_WIDTH - 720.0).abs() < f32::EPSILON);
        assert!((WINDOW_HEIGHT - 460.0).abs() < f32::EPSILON);
    }
}
