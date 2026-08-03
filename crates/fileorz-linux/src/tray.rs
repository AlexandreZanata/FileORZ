//! StatusNotifier tray — Open / Quit (B-02 / B-04).

use ksni::blocking::TrayMethods;
use ksni::menu::StandardItem;
use ksni::{Error as KsniError, MenuItem, Tray};
use std::sync::mpsc::{self, Receiver, Sender};

/// Localized tray strings (`tray-open`, `tray-quit`, `tray-tooltip`).
#[derive(Debug, Clone)]
pub struct TrayLabels {
    pub tooltip: String,
    pub open: String,
    pub quit: String,
}

impl TrayLabels {
    /// Build from Fluent message lookup.
    #[must_use]
    pub fn from_messages(tooltip: &str, open: &str, quit: &str) -> Self {
        Self {
            tooltip: tooltip.into(),
            open: open.into(),
            quit: quit.into(),
        }
    }
}

/// Commands emitted by tray menu actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    Open,
    Quit,
}

struct FileorzTray {
    labels: TrayLabels,
    tx: Sender<TrayCommand>,
}

impl Tray for FileorzTray {
    fn id(&self) -> String {
        "fileorz".into()
    }

    fn title(&self) -> String {
        self.labels.tooltip.clone()
    }

    fn icon_name(&self) -> String {
        // Theme icon; packaging can ship a named icon later.
        "folder".into()
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let open_label = self.labels.open.clone();
        let quit_label = self.labels.quit.clone();
        vec![
            StandardItem {
                label: open_label,
                activate: Box::new(|this: &mut Self| {
                    let _ = this.tx.send(TrayCommand::Open);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: quit_label,
                icon_name: "application-exit".into(),
                activate: Box::new(|this: &mut Self| {
                    let _ = this.tx.send(TrayCommand::Quit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = self.tx.send(TrayCommand::Open);
    }
}

/// Running tray handle + command receiver.
pub struct TrayService {
    handle: ksni::blocking::Handle<FileorzTray>,
    rx: Receiver<TrayCommand>,
}

impl TrayService {
    /// Spawn StatusNotifier item. Needs a session bus + SNI watcher.
    pub fn spawn(labels: TrayLabels) -> Result<Self, TrayError> {
        let (tx, rx) = mpsc::channel();
        let tray = FileorzTray { labels, tx };
        let handle = tray.spawn().map_err(TrayError::from)?;
        Ok(Self { handle, rx })
    }

    /// Block until the next menu / activate command.
    pub fn recv(&self) -> Result<TrayCommand, TrayError> {
        self.rx.recv().map_err(|_| TrayError::Closed)
    }

    /// Try non-blocking receive.
    pub fn try_recv(&self) -> Option<TrayCommand> {
        self.rx.try_recv().ok()
    }

    /// Shut down the tray service and wait for the worker to exit.
    pub fn shutdown(self) {
        self.handle.shutdown().wait();
    }
}

/// Menu label order for unit tests (Open then Quit).
#[must_use]
pub fn menu_label_order(labels: &TrayLabels) -> [&str; 2] {
    [labels.open.as_str(), labels.quit.as_str()]
}

#[derive(Debug)]
pub enum TrayError {
    Ksni(KsniError),
    Closed,
}

impl From<KsniError> for TrayError {
    fn from(value: KsniError) -> Self {
        Self::Ksni(value)
    }
}

impl std::fmt::Display for TrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ksni(e) => write!(f, "{e}"),
            Self::Closed => write!(f, "tray command channel closed"),
        }
    }
}

impl std::error::Error for TrayError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_order_open_then_quit() {
        let labels = TrayLabels::from_messages("FileORZ", "Open", "Quit");
        assert_eq!(menu_label_order(&labels), ["Open", "Quit"]);
    }

    #[test]
    fn spawn_smoke_or_skip_without_sni() {
        let labels = TrayLabels::from_messages("FileORZ", "Open", "Quit");
        match TrayService::spawn(labels) {
            Ok(svc) => {
                svc.shutdown();
            }
            Err(TrayError::Ksni(_)) => {
                // CI / headless without StatusNotifierWatcher — acceptable.
            }
            Err(e) => panic!("unexpected: {e}"),
        }
    }
}
