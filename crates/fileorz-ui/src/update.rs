//! Message handling for the main shell.

use crate::links::{self, CHANGELOG_URL, GITHUB_URL};
use crate::message::Message;
use crate::organizer::{self, StartOutcome, StartReject};
use crate::persist;
use crate::shell::ShellApp;
use crate::smoke;
use crate::state::{transition, PhaseEvent, RunPhase};
use fileorz_linux::autostart;
use fileorz_linux::tray::TrayCommand;
use iced::window::{self, Mode};
use iced::Task;
use std::path::PathBuf;

/// Apply a message; may hide/show window or exit.
pub fn update(app: &mut ShellApp, message: Message) -> Task<Message> {
    match message {
        Message::PickFolder => pick_folder(app),
        Message::FolderPicked(path) => {
            on_folder(app, path);
            Task::none()
        }
        Message::IntervalChanged(mins) => {
            on_interval(app, mins);
            Task::none()
        }
        Message::ToggleOrganizer => {
            on_toggle(app);
            Task::none()
        }
        Message::SettingsStub => Task::none(),
        Message::AutostartToggled(enabled) => {
            on_autostart(app, enabled);
            Task::none()
        }
        Message::OpenGithub => {
            links::open_url(GITHUB_URL);
            Task::none()
        }
        Message::OpenChangelog => {
            links::open_url(CHANGELOG_URL);
            Task::none()
        }
        Message::ShowAbout => {
            app.feedback = Some(app.strings.about_body.clone());
            Task::none()
        }
        Message::CloseRequested(id) => on_close(app, id),
        Message::TrayPoll => on_tray_poll(app),
        Message::ShowWindow => show_window(),
        Message::Quit => on_quit(app),
        Message::SmokeTick => {
            let Some(path) = app.smoke_path.take() else {
                return Task::none();
            };
            smoke::capture(path)
        }
        Message::SmokeSave {
            path,
            bytes,
            width,
            height,
        } => {
            if let Err(err) = smoke::write_ppm(&path, width, height, &bytes) {
                eprintln!("smoke write failed: {err}");
            } else {
                eprintln!("smoke wrote {}", path.display());
            }
            on_quit(app)
        }
    }
}

fn pick_folder(app: &ShellApp) -> Task<Message> {
    let title = app.strings.folder_dialog.clone();
    let initial = app
        .config
        .folder
        .as_ref()
        .map(PathBuf::from)
        .filter(|p| p.is_dir());
    Task::perform(
        async move {
            let mut dialog = rfd::AsyncFileDialog::new().set_title(title);
            if let Some(dir) = initial {
                dialog = dialog.set_directory(dir);
            }
            dialog.pick_folder().await.map(|h| h.path().to_path_buf())
        },
        Message::FolderPicked,
    )
}

fn on_folder(app: &mut ShellApp, path: Option<PathBuf>) {
    let Some(path) = path else {
        return;
    };
    app.config.folder = Some(path.display().to_string());
    let _ = persist::save(&app.config);
    if app.phase == RunPhase::Error {
        app.phase = transition(app.phase, PhaseEvent::Cleared);
        app.feedback = None;
    }
}

fn on_interval(app: &mut ShellApp, mins: u32) {
    app.config.interval_minutes = persist::clamp_interval(mins);
    let _ = persist::save(&app.config);
}

fn on_toggle(app: &mut ShellApp) {
    if app.phase == RunPhase::Running {
        if let Some(handle) = app.organizer.take() {
            organizer::stop(handle);
        }
        app.phase = transition(app.phase, PhaseEvent::Stopped);
        app.feedback = None;
        return;
    }
    match organizer::try_start(&app.config) {
        StartOutcome::Started(handle) => {
            app.organizer = Some(handle);
            app.phase = transition(app.phase, PhaseEvent::StartAttempt { folder_ok: true });
            app.feedback = Some(app.strings.feedback_started.clone());
        }
        StartOutcome::Rejected(StartReject::MissingFolder) => {
            app.phase = transition(app.phase, PhaseEvent::StartAttempt { folder_ok: false });
            app.feedback = Some(app.strings.err_folder_missing.clone());
        }
        StartOutcome::Rejected(StartReject::InvalidFolder) => {
            app.phase = transition(app.phase, PhaseEvent::StartFailed);
            app.feedback = Some(app.strings.err_folder_invalid.clone());
        }
    }
}

fn on_autostart(app: &mut ShellApp, enabled: bool) {
    let result = if enabled {
        autostart::enable().map(|_| ())
    } else {
        autostart::disable()
    };
    if let Err(err) = result {
        eprintln!("autostart toggle failed: {err}");
        return;
    }
    app.config.autostart = enabled;
    let _ = persist::save(&app.config);
}

fn on_close(app: &mut ShellApp, id: window::Id) -> Task<Message> {
    if app.tray.is_some() {
        window::change_mode(id, Mode::Hidden)
    } else {
        on_quit(app)
    }
}

fn on_tray_poll(app: &mut ShellApp) -> Task<Message> {
    let Some(tray) = app.tray.as_ref() else {
        return Task::none();
    };
    match tray.try_recv() {
        Some(TrayCommand::Open) => Task::done(Message::ShowWindow),
        Some(TrayCommand::Quit) => Task::done(Message::Quit),
        None => Task::none(),
    }
}

fn show_window() -> Task<Message> {
    window::get_oldest().then(|id| match id {
        Some(id) => Task::batch([
            window::change_mode(id, Mode::Windowed),
            window::gain_focus(id),
        ]),
        None => Task::none(),
    })
}

fn on_quit(app: &mut ShellApp) -> Task<Message> {
    if let Some(handle) = app.organizer.take() {
        organizer::stop(handle);
    }
    if let Some(tray) = app.tray.take() {
        tray.shutdown();
    }
    iced::exit()
}
