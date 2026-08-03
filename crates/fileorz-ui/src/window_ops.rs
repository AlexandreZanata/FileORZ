//! Window / tray helpers for the main shell update loop.

use crate::message::Message;
use crate::organizer;
use crate::shell::ShellApp;
use fileorz_linux::tray::TrayCommand;
use iced::window::{self, Mode};
use iced::Task;

pub fn on_close(app: &mut ShellApp, id: window::Id) -> Task<Message> {
    // Normal GUI launch: close = quit (Pop!_OS / GNOME often hide the tray icon).
    // `--tray` / start_hidden: close hides; Quit from the tray menu exits.
    if app.start_hidden && app.tray.is_some() {
        window::change_mode(id, Mode::Hidden)
    } else {
        on_quit(app)
    }
}

pub fn on_tray_poll(app: &mut ShellApp) -> Task<Message> {
    let Some(tray) = app.tray.as_ref() else {
        return Task::none();
    };
    match tray.try_recv() {
        Some(TrayCommand::Open) => Task::done(Message::ShowWindow),
        Some(TrayCommand::Quit) => Task::done(Message::Quit),
        None => Task::none(),
    }
}

pub fn show_window() -> Task<Message> {
    window::get_oldest().then(|id| match id {
        Some(id) => Task::batch([
            window::change_mode(id, Mode::Windowed),
            window::gain_focus(id),
        ]),
        None => Task::none(),
    })
}

pub fn on_quit(app: &mut ShellApp) -> Task<Message> {
    if let Some(handle) = app.organizer.take() {
        organizer::stop(handle);
    }
    if let Some(tray) = app.tray.take() {
        tray.shutdown();
    }
    iced::exit()
}
