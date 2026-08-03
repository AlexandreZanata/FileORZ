//! Embedded product icon for header + window chrome.

use iced::widget::image::Handle;
use iced::window::Icon;

/// 128² PNG shipped under packaging (also used as window / header art).
const ICON_PNG: &[u8] =
    include_bytes!("../../../packaging/linux/icons/hicolor/128x128/apps/fileorz.png");

/// iced image handle for the header brand mark (clone per frame is cheap).
#[must_use]
pub fn header_handle() -> Handle {
    Handle::from_bytes(ICON_PNG)
}

/// Optional window icon for the compositor title bar.
#[must_use]
pub fn window_icon() -> Option<Icon> {
    iced::window::icon::from_file_data(ICON_PNG, None).ok()
}
