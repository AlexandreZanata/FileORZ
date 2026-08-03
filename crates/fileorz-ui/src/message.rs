//! iced messages for the main shell + settings.

use crate::settings::SettingsMsg;
use iced::window;
use std::path::PathBuf;

/// User and system events.
#[derive(Debug, Clone)]
pub enum Message {
    /// Native folder dialog finished (or cancelled → `None`).
    FolderPicked(Option<PathBuf>),
    /// Open the folder picker.
    PickFolder,
    /// Interval dropdown changed.
    IntervalChanged(u32),
    /// Primary CTA (start when idle/error, stop when running).
    ToggleOrganizer,
    /// Enter key (main screen → ToggleOrganizer).
    EnterKey,
    /// Open settings hub.
    OpenSettings,
    /// Esc / Back within settings / About.
    SettingsBack,
    /// Settings hub / editor events.
    Settings(SettingsMsg),
    /// Autostart switch.
    AutostartToggled(bool),
    /// Language picker (`en` / `pt-BR`).
    LocaleChanged(String),
    /// Open GitHub in browser.
    OpenGithub,
    /// Open changelog in browser.
    OpenChangelog,
    /// Open About dialog.
    ShowAbout,
    /// About: open upstream URL.
    OpenUpstream,
    /// About: open fork URL.
    OpenFork,
    /// About: open notices context.
    OpenNotices,
    /// Window close → hide when tray is active.
    CloseRequested(window::Id),
    /// Poll tray channel.
    TrayPoll,
    /// Tray Open → show window.
    ShowWindow,
    /// Tray Quit → exit.
    Quit,
    /// Advance fade clocks.
    MotionTick,
    /// Window scale factor sample (HiDPI).
    ScaleFactor(f32),
    /// Smoke helper: capture window then exit.
    SmokeTick,
    /// Smoke helper: write capture.
    SmokeSave {
        path: PathBuf,
        bytes: Vec<u8>,
        width: u32,
        height: u32,
    },
}
