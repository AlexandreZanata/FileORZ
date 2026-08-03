//! WCAG-ish contrast helpers for design tokens.

use iced::Color;

/// Relative luminance (sRGB, WCAG 2.x).
#[must_use]
pub fn relative_luminance(c: Color) -> f32 {
    fn chan(u: f32) -> f32 {
        if u <= 0.039_28 {
            u / 12.92
        } else {
            ((u + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * chan(c.r) + 0.7152 * chan(c.g) + 0.0722 * chan(c.b)
}

/// Contrast ratio between two colors (≥1.0).
#[must_use]
pub fn contrast_ratio(a: Color, b: Color) -> f32 {
    let (l1, l2) = (relative_luminance(a), relative_luminance(b));
    let (hi, lo) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (hi + 0.05) / (lo + 0.05)
}

/// WCAG AA normal text threshold.
pub const AA_NORMAL: f32 = 4.5;
/// WCAG AA large / UI component threshold.
pub const AA_UI: f32 = 3.0;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{ACCENT, BG, DANGER, SURFACE, TEXT, TEXT_MUTED};

    #[test]
    fn primary_text_on_bg_meets_aa() {
        assert!(
            contrast_ratio(TEXT, BG) >= AA_NORMAL,
            "TEXT on BG = {}",
            contrast_ratio(TEXT, BG)
        );
    }

    #[test]
    fn muted_text_on_surface_meets_aa_ui() {
        assert!(
            contrast_ratio(TEXT_MUTED, SURFACE) >= AA_UI,
            "TEXT_MUTED on SURFACE = {}",
            contrast_ratio(TEXT_MUTED, SURFACE)
        );
    }

    #[test]
    fn accent_and_danger_on_bg_meet_aa_ui() {
        assert!(contrast_ratio(ACCENT, BG) >= AA_UI);
        assert!(contrast_ratio(DANGER, BG) >= AA_UI);
        assert!(contrast_ratio(Color::WHITE, ACCENT) >= AA_UI);
    }
}
