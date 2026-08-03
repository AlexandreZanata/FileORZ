//! iced messages for the main shell.

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
    /// Settings stub (phase 15+).
    SettingsStub,
    /// Autostart switch.
    AutostartToggled(bool),
    /// Open GitHub in browser.
    OpenGithub,
    /// Open changelog in browser.
    OpenChangelog,
    /// Show about blurb in feedback.
    ShowAbout,
    /// Window close → hide when tray is active.
    CloseRequested(window::Id),
    /// Poll tray channel.
    TrayPoll,
    /// Tray Open → show window.
    ShowWindow,
    /// Tray Quit → exit.
    Quit,
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
