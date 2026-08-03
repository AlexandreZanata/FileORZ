//! Settings message handlers (navigation + editors).

use crate::persist;
use crate::settings::ext_logic::{set_category_all, set_ext};
use crate::settings::keywords_logic::{self, groups_from_rows};
use crate::settings::msg::SettingsMsg;
use crate::settings::mutex::{
    clamp_days, set_by_created, set_by_modified, set_permanent, set_to_trash,
};
use crate::settings::screen::{SettingsScreen, SETTINGS_HEIGHT, SETTINGS_WIDTH};
use crate::shell::ShellApp;
use crate::tokens::{WINDOW_HEIGHT, WINDOW_WIDTH};
use fileorz_core::advanced_pdf::{load_keywords, save_keywords};
use fileorz_linux::xdg::keywords_json_path;
use iced::window;
use iced::{Size, Task};

use crate::message::Message;

/// Open settings hub and grow the window.
pub fn open_hub(app: &mut ShellApp) -> Task<Message> {
    app.settings = SettingsScreen::Hub;
    app.settings_feedback = None;
    resize(SETTINGS_WIDTH, SETTINGS_HEIGHT)
}

/// Esc / Back navigation; restore main size on Main.
pub fn go_back(app: &mut ShellApp) -> Task<Message> {
    app.settings = app.settings.back();
    app.settings_feedback = None;
    if app.settings == SettingsScreen::Main {
        resize(WINDOW_WIDTH, WINDOW_HEIGHT)
    } else {
        Task::none()
    }
}

/// Dispatch a settings editor message.
pub fn handle(app: &mut ShellApp, msg: SettingsMsg) -> Task<Message> {
    match msg {
        SettingsMsg::OpenExtensions => {
            app.settings = SettingsScreen::Extensions;
            app.settings_feedback = None;
        }
        SettingsMsg::OpenAdvanced => {
            app.settings = SettingsScreen::Advanced;
            load_keyword_rows(app);
        }
        SettingsMsg::OpenAutoDelete => {
            app.settings = SettingsScreen::AutoDelete;
            app.config.auto_delete.max_age_days = clamp_days(app.config.auto_delete.max_age_days);
        }
        SettingsMsg::ExtToggle {
            category,
            ext,
            enabled,
        } => set_ext(&mut app.config, &category, &ext, enabled),
        SettingsMsg::ExtSetAll { category, enabled } => {
            set_category_all(&mut app.config, &category, enabled);
        }
        SettingsMsg::ExtSave => {
            if let Err(err) = persist::save(&app.config) {
                eprintln!("ext save failed: {err}");
            } else {
                app.settings_feedback = Some(app.settings_strings.ext_saved.clone());
            }
        }
        SettingsMsg::AdvEnabled(on) => {
            app.config.advanced_organize = on;
            let _ = persist::save(&app.config);
        }
        SettingsMsg::AdvAddGroup => keywords_logic::add_row(&mut app.keyword_rows),
        SettingsMsg::AdvDeleteGroup(i) => {
            keywords_logic::remove_row(&mut app.keyword_rows, i);
            persist_keywords(app);
        }
        SettingsMsg::AdvName(i, name) => {
            if let Some(row) = app.keyword_rows.get_mut(i) {
                row.name = name;
            }
        }
        SettingsMsg::AdvPhrases(i, phrases) => {
            if let Some(row) = app.keyword_rows.get_mut(i) {
                row.phrases = phrases;
            }
        }
        SettingsMsg::AdvSaveGroup(_) => persist_keywords(app),
        SettingsMsg::AdEnabled(on) => {
            app.config.auto_delete.enabled = on;
            let _ = persist::save(&app.config);
        }
        SettingsMsg::AdByCreated => {
            set_by_created(&mut app.config.auto_delete);
            let _ = persist::save(&app.config);
        }
        SettingsMsg::AdByModified => {
            set_by_modified(&mut app.config.auto_delete);
            let _ = persist::save(&app.config);
        }
        SettingsMsg::AdDays(days) => {
            app.config.auto_delete.max_age_days = clamp_days(days);
            let _ = persist::save(&app.config);
        }
        SettingsMsg::AdTrash => {
            set_to_trash(&mut app.config.auto_delete);
            let _ = persist::save(&app.config);
        }
        SettingsMsg::AdPermanent => {
            set_permanent(&mut app.config.auto_delete);
            let _ = persist::save(&app.config);
        }
    }
    Task::none()
}

fn load_keyword_rows(app: &mut ShellApp) {
    let path = keywords_json_path();
    let groups = load_keywords(&path).unwrap_or_default();
    app.keyword_rows = keywords_logic::rows_from_groups(&groups);
}

fn persist_keywords(app: &mut ShellApp) {
    let groups = groups_from_rows(&app.keyword_rows);
    if let Err(err) = save_keywords(&keywords_json_path(), &groups) {
        eprintln!("keywords save failed: {err}");
    }
}

fn resize(width: f32, height: f32) -> Task<Message> {
    window::get_oldest().then(move |id| match id {
        Some(id) => window::resize(id, Size::new(width, height)),
        None => Task::none(),
    })
}
