//! Launch iced main shell (`fileorz` default / `--ui`).

use crate::exit_code;
use fileorz_i18n::resolve_locale_from_env;
use std::process::ExitCode;

pub fn run(locale_cli: Option<&str>) -> ExitCode {
    let tag = resolve_locale_from_env(locale_cli, None);
    match fileorz_ui::run(&tag) {
        Ok(()) => ExitCode::from(exit_code::OK),
        Err(e) => {
            eprintln!("ui error: {e}");
            ExitCode::from(exit_code::ERROR)
        }
    }
}
