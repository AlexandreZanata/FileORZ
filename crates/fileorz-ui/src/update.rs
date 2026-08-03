//! Message handling for the main shell.

use crate::links::{self, CHANGELOG_URL, FORK_URL, GITHUB_URL, NOTICES_URL, UPSTREAM_URL};
use crate::message::Message;
use crate::organizer::{self, StartOutcome, StartReject};
use crate::persist;
use crate::settings::SettingsScreen;
use crate::shell::ShellApp;
use crate::smoke;
use crate::state::{transition, PhaseEvent, RunPhase};
use crate::window_ops::{self, on_quit};
use fileorz_linux::autostart;
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
        Message::EnterKey => on_enter(app),
        Message::OpenSettings => crate::settings::update::open_hub(app),
        Message::SettingsBack => crate::settings::update::go_back(app),
        Message::Settings(msg) => crate::settings::update::handle(app, msg),
        Message::AutostartToggled(enabled) => {
            on_autostart(app, enabled);
            Task::none()
        }
        Message::LocaleChanged(tag) => {
            app.apply_locale(&tag);
            Task::none()
        }
        Message::OpenGithub => open(GITHUB_URL),
        Message::OpenChangelog => open(CHANGELOG_URL),
        Message::OpenUpstream => open(UPSTREAM_URL),
        Message::OpenFork => open(FORK_URL),
        Message::OpenNotices => open(NOTICES_URL),
        Message::ShowAbout => {
            app.settings = SettingsScreen::About;
            app.motion.kick_screen();
            Task::none()
        }
        Message::MotionTick => {
            let _ = app.motion.tick();
            Task::none()
        }
        Message::ScaleFactor(scale) => {
            app.scale_factor = scale;
            eprintln!("ui scale_factor={scale:.3}");
            Task::none()
        }
        Message::CloseRequested(id) => window_ops::on_close(app, id),
        Message::TrayPoll => window_ops::on_tray_poll(app),
        Message::ShowWindow => window_ops::show_window(),
        Message::Quit => on_quit(app),
        Message::SmokeTick => smoke_tick(app),
        Message::SmokeSave {
            path,
            bytes,
            width,
            height,
        } => smoke_save(app, path, bytes, width, height),
    }
}

fn open(url: &str) -> Task<Message> {
    links::open_url(url);
    Task::none()
}

fn on_enter(app: &mut ShellApp) -> Task<Message> {
    if app.settings == SettingsScreen::Main {
        on_toggle(app);
    }
    Task::none()
}

fn smoke_tick(app: &mut ShellApp) -> Task<Message> {
    let Some(path) = app.smoke_path.take() else {
        return Task::none();
    };
    smoke::capture(path)
}

fn smoke_save(
    app: &mut ShellApp,
    path: PathBuf,
    bytes: Vec<u8>,
    width: u32,
    height: u32,
) -> Task<Message> {
    if let Err(err) = smoke::write_ppm(&path, width, height, &bytes) {
        eprintln!("smoke write failed: {err}");
    } else {
        eprintln!("smoke wrote {}", path.display());
    }
    on_quit(app)
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
            app.motion.kick_feedback();
        }
        StartOutcome::Rejected(StartReject::MissingFolder) => {
            app.phase = transition(app.phase, PhaseEvent::StartAttempt { folder_ok: false });
            app.feedback = Some(app.strings.err_folder_missing.clone());
            app.motion.kick_feedback();
        }
        StartOutcome::Rejected(StartReject::InvalidFolder) => {
            app.phase = transition(app.phase, PhaseEvent::StartFailed);
            app.feedback = Some(app.strings.err_folder_invalid.clone());
            app.motion.kick_feedback();
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
