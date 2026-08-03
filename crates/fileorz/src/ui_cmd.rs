//! Launch iced main shell (`fileorz` default / `--ui`).

use crate::exit_code;
use fileorz_core::config::load_config_file;
use fileorz_i18n::resolve_locale_from_env;
use fileorz_linux::xdg::config_json_path;
use std::process::ExitCode;

pub fn run(locale_cli: Option<&str>) -> ExitCode {
    let config_locale = load_config_file(&config_json_path())
        .ok()
        .map(|o| o.config.locale);
    let tag = resolve_locale_from_env(locale_cli, config_locale.as_deref());
    match fileorz_ui::run(&tag) {
        Ok(()) => ExitCode::from(exit_code::OK),
        Err(e) => {
            eprintln!("ui error: {e}");
            ExitCode::from(exit_code::ERROR)
        }
    }
}
