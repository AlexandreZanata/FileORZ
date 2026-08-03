//! Spot-check critical IDs in en / pt-BR + fallback behavior.

use crate::{resolve_locale, Localization};

#[test]
fn crate_name_matches() {
    assert_eq!(crate::crate_name(), "fileorz-i18n");
}

#[test]
fn en_critical_ids() {
    let loc = Localization::embed("en").unwrap();
    assert_eq!(loc.message("app-title"), "FileORZ");
    assert_eq!(loc.message("main-btn-start"), "Start organizing");
    assert_eq!(
        loc.message("error-folder-missing"),
        "Choose a valid folder first."
    );
    assert_eq!(loc.message("tray-quit"), "Quit");
}

#[test]
fn pt_br_critical_ids() {
    let loc = Localization::embed("pt-BR").unwrap();
    assert_eq!(loc.locale(), "pt-BR");
    assert_eq!(loc.message("app-title"), "FileORZ");
    assert_eq!(loc.message("main-btn-start"), "Iniciar Organização");
    assert_eq!(
        loc.message("error-folder-missing"),
        "Selecione uma pasta primeiro!"
    );
    assert_eq!(loc.message("tray-quit"), "Fechar");
}

#[test]
fn unsupported_locale_uses_english_catalog() {
    let loc = Localization::embed("fr").unwrap();
    assert_eq!(loc.locale(), "en");
    assert_eq!(loc.message("main-btn-settings"), "Settings");
}

#[test]
fn missing_id_returns_literal() {
    let loc = Localization::embed("en").unwrap();
    assert_eq!(loc.message("does-not-exist"), "does-not-exist");
}

#[test]
fn from_dir_matches_embed() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("locales");
    let disk = Localization::from_dir(&dir, "pt-BR").unwrap();
    let embed = Localization::embed("pt-BR").unwrap();
    assert_eq!(
        disk.message("main-btn-start"),
        embed.message("main-btn-start")
    );
}

#[test]
fn t_macro_works() {
    let loc = Localization::embed("en").unwrap();
    assert_eq!(crate::t!(loc, "app-title"), "FileORZ");
}

#[test]
fn resolve_defaults_to_en() {
    assert_eq!(resolve_locale(None, None, None, None), "en");
}
