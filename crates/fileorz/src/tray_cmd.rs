//! `fileorz --tray` — StatusNotifier + iced shell (hide/show).

use crate::exit_code;
use fileorz_core::advanced_pdf::{load_keywords, KeywordGroups};
use fileorz_core::config::{load_config_file, AppConfig};
use fileorz_core::scheduler::{OrganizerHandle, OrganizerOptions};
use fileorz_i18n::{resolve_locale_from_env, Localization};
use fileorz_linux::trash::FreedesktopTrash;
use fileorz_linux::tray::{TrayCommand, TrayLabels, TrayService};
use fileorz_linux::xdg::{config_json_path, keywords_json_path};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

/// Prefer iced shell (hidden); fall back to tray-only if UI cannot start.
pub fn run(locale_cli: Option<&str>) -> ExitCode {
    let config_locale = load_config_locale();
    let tag = resolve_locale_from_env(locale_cli, config_locale.as_deref());

    if std::env::var_os("FILEORZ_TRAY_SMOKE").is_some() {
        return smoke_tray(&tag);
    }

    match fileorz_ui::run_tray(&tag) {
        Ok(()) => ExitCode::from(exit_code::OK),
        Err(e) => {
            eprintln!("ui unavailable ({e}); tray-only fallback");
            run_tray_only(&tag)
        }
    }
}

fn load_config_locale() -> Option<String> {
    let path = config_json_path();
    load_config_file(&path).ok().map(|o| o.config.locale)
}

fn smoke_tray(tag: &str) -> ExitCode {
    let loc = match Localization::embed(tag) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("i18n error: {e}");
            return ExitCode::from(exit_code::ERROR);
        }
    };
    let labels = TrayLabels::from_messages(
        &loc.message("tray-tooltip"),
        &loc.message("tray-open"),
        &loc.message("tray-quit"),
    );
    // Smoke never opens the iced window (E2E-04: worker without window).
    println!("tray-smoke: no-window");
    let mut organizer = maybe_start_organizer();
    let worker = organizer.is_some();
    println!("tray-smoke: worker={worker}");
    match TrayService::spawn(labels) {
        Ok(tray) => {
            std::thread::sleep(Duration::from_millis(200));
            stop_organizer(&mut organizer);
            tray.shutdown();
            println!("tray-smoke: ok");
            ExitCode::from(exit_code::OK)
        }
        Err(e) => {
            stop_organizer(&mut organizer);
            // Headless CI often lacks StatusNotifierWatcher — worker path still counts.
            println!("tray-smoke: no-sni ({e})");
            if worker {
                println!("tray-smoke: worker-ok");
                ExitCode::from(exit_code::OK)
            } else {
                ExitCode::from(exit_code::ERROR)
            }
        }
    }
}

fn run_tray_only(tag: &str) -> ExitCode {
    let loc = match Localization::embed(tag) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("i18n error: {e}");
            return ExitCode::from(exit_code::ERROR);
        }
    };
    let labels = TrayLabels::from_messages(
        &loc.message("tray-tooltip"),
        &loc.message("tray-open"),
        &loc.message("tray-quit"),
    );
    let tray = match TrayService::spawn(labels) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("tray error: {e}");
            eprintln!(
                "hint: GNOME needs an AppIndicator / SNI extension; see docs/LINUX-DESKTOP.md"
            );
            return ExitCode::from(exit_code::ERROR);
        }
    };

    let mut organizer = maybe_start_organizer();
    loop {
        match tray.recv() {
            Ok(TrayCommand::Open) => {
                eprintln!("tray: open (no UI process available)");
            }
            Ok(TrayCommand::Quit) => {
                stop_organizer(&mut organizer);
                tray.shutdown();
                return ExitCode::from(exit_code::OK);
            }
            Err(e) => {
                eprintln!("tray: {e}");
                stop_organizer(&mut organizer);
                return ExitCode::from(exit_code::ERROR);
            }
        }
    }
}

fn stop_organizer(handle: &mut Option<OrganizerHandle>) {
    if let Some(h) = handle.take() {
        let _ = h.stop(Duration::from_secs(5));
    }
}

fn maybe_start_organizer() -> Option<OrganizerHandle> {
    let config_path = config_json_path();
    if !config_path.is_file() {
        return None;
    }
    let outcome = load_config_file(&config_path).ok()?;
    let root = organize_root(&outcome.config)?;
    let keywords = load_keywords_for(&outcome.config);
    let interval = Duration::from_secs(u64::from(outcome.config.interval_minutes.max(1)) * 60);
    let trash: Arc<dyn fileorz_core::autodelete::TrashSink> = Arc::new(FreedesktopTrash);
    Some(OrganizerHandle::start(OrganizerOptions {
        root,
        config: outcome.config,
        keywords,
        interval,
        trash: Some(trash),
    }))
}

fn organize_root(config: &AppConfig) -> Option<PathBuf> {
    let folder = config.folder.as_ref()?;
    let path = PathBuf::from(folder);
    path.is_dir().then_some(path)
}

fn load_keywords_for(config: &AppConfig) -> KeywordGroups {
    if !config.advanced_organize {
        return KeywordGroups::new();
    }
    let path = keywords_json_path();
    load_keywords(&path).unwrap_or_else(|_| KeywordGroups::new())
}
